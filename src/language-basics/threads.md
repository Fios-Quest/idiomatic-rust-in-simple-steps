Threads
=======

Threads allow us to build programs where parts of the program can run independently of one another.

Threads _can_ (big emphasis on "can") help you make faster and more responsive programs.

For example:

- As a web developer, I would like the server framework I'm using to start responding to the next request before it's
  finished responding to the previous request
- As a game developer, I would like my game engine to capture player inputs without being blocked by the renderer
- As a data engineer, I would like to process large sets of data in parallel

We'll step through:

- how we can run code in a thread, including sending data before it starts
- how we can wait for a thread to end, including receiving data when it ends
- how we can communicate with running threads
- how we can share state between threads

We'll also be touching again on our marker traits Send and Sync

What is a thread?
-----------------

Before we get into the Rust, it's worth discussing what a thread is.

When you run a program, that specific instance of the program is called a Process. The process incorporates not just the
instructions to be run but is an abstraction around various resources that the program has access to, such as memory.

You can run multiple processes which the operating system will schedule separate which could allow you to do more things
at once, however, those processes won't (or at least, shouldn't) have access to the same memory. There are ways to
communicate between processes, but they can be slower and more restrictive than if we could share memory.

The part of the process responsible for executing your code is called a thread, and a single process can have multiple
threads. Threads are scheduled by the operating system independently, allowing one process to do multiple things
effectively concurrently.

Starting a thread
-----------------

Your program always has at least one thread, even your basic hello-world program runs in a thread.

```rust
fn main() {
    println!("Hello, I am in a thread!")
}
```

What we're interested in today is how we start more threads. This is a process called Spawning.

To spawn a thread, we use `std::thread::spawn`... but, this will do little on its own. Run the code below, see what's
missing in the output?

```rust
use std::thread::spawn;

fn main() {
    println!("This is the main thread");
    spawn(|| {
        println!("This is a child thread");
    });
    println!("This is the end of the main thread");
}
```

Spawning a thread returns a join handler. The join handler is what ties the spawned thread to the thread that spawned
it. When the join handler is dropped, the thread is orphaned. It may still run but, in this case, the process ends at
the end of main, so our spawned thread never got a chance to do anything.

We can tell our main thread to pause and wait for a running thread to end by using the join handler.

```rust
use std::thread::spawn;

fn main() {
    println!("This is the main thread");
    let handler = spawn(|| {
        println!("This is a child thread");
    });
    handler.join().expect("Child thread panicked");
    println!("This is the end of the main thread");
}
```

Note, we're using closures here, but functions work just as well and can be better for more complex programs. The only
restriction is: it needs to be `FnOnce() -> T + Send + 'static`. See
[the documentation](https://doc.rust-lang.org/std/thread/fn.spawn.html) for more details.

```rust
use std::thread::spawn;

fn child() {
    println!("This is also a child thread");
}

fn main() {
    println!("This is the main thread");
    let handler = spawn(child);
    handler.join().expect("Child thread panicked");
    println!("This is the end of the main thread");
}
```

Exactly _when_ threads are allowed to execute code is controlled by a scheduler which we can't directly manage
ourselves, but we can influence it. Putting one thread to sleep can allow another thread to run. Run this code, then
uncomment the commented lines and run it again.

```rust,editable
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let handler = spawn(|| {
        for i in 0..10 {
            println!("Child iteration: {i}");
            // sleep(Duration::from_millis(1));
        }
    });

    for i in 0..10 {
        println!("Main iteration: {i}");
        // sleep(Duration::from_millis(1));
    }

    handler.join().expect("Child thread panicked");
}
```

So now we can run threads, let's start looking at how to send data back and forth between them.

We can pass data into a thread before it starts so long as the data is `Send`. We previously talked about this trait in
the [Traits](./traits.md) chapter, but to recap, data is `Send` so long as it can be safely sent between threads, and
this trait is auto-implemented for all types that can be `Send` (though it is also possible to opt out of it).

We can move data into the closure that will be sent to the thread using the `move` keyword.

For example:

```rust
use std::thread::spawn;

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    let handler = spawn(move || {
        data
            .into_iter()
            .for_each(|i| println!("Processing item {i} from the main thread"));
    });

    handler.join().expect("Child thread panicked");
}
```

You can also return data via the join handler. This means you could pass hard work to a thread and do other work, coming
back to check on the thread at a later time. We can check if the thread is finished with `.is_finished()`;

```rust
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let data = u16::MIN..u16::MAX;

    let handler = spawn(move || {
        data.map(|i| i as u32).sum::<u32>()
    });

    while !handler.is_finished() {
        println!("Still working!");
        sleep(Duration::from_nanos(100));
    }

    let answer = handler.join().expect("Child thread panicked");

    assert_eq!(answer, 2147385345);
}
```

Sending messages
----------------

Now we can start one thread, there's no stopping us!

Modern schedulers can manage a _lot_ of threads at once, however, so far we can only send data between a child thread
and the parent that started it. What if we want to communicate across multiple threads, or send data to a thread after
we already started it?

Multi-producer, single-consumer (MPSC) is a queue pattern that allows us to create channels with multiple `Sender`s that
can send messages, and a single `Reciever` that can receive them. As per the name, Multi-producer, you can clone
`Sender`s but each of those clones can only send to a single `Reciever`. The `Sender` and `Receiver` types are `Send`
meaning that you can create them in one thread and send them to another.

Let's create a bunch of threads and give each of them a `Sender` that points back to a single `Reciever`, we'll send
that `Reciever` to a final thread that will collect the data from the other threads.

```rust
use std::sync::mpsc::channel;
use std::thread::spawn;

fn main() {
    let (sender, receiver) = channel();

    let thread_ids = 0..10;

    // move sender into the closure
    let sending_handlers = thread_ids.map(move |id| {
        // sender is owned by this closure, we want to pass a copy to each
        // child thread so we'll clone it on each iteration
        let cloned_sender = sender.clone();
        // move the cloned sender to the next thread
        spawn(move || {
            cloned_sender.send(format!("Reporting in from thread {id}"))
                .expect("The Receiver was dropped");
        })
    });

    let receiving_handler = spawn(move || {
        while let Ok(message) = receiver.recv() {
            println!("Received message: {message}");
        }
    });


    sending_handlers.for_each(|h| h.join().expect("A sending thread panicked"));

    receiving_handler.join().expect("receiving thread panicked");
}

```

For what its worth, there's no built-in way to create a channel with multiple receivers (`Receiver` is not `Clone`),
however, there's nothing stopping you building your own type for that, or there are crates that support it like
[Crossbeam](https://docs.rs/crossbeam/latest/crossbeam/).

Sharing State
-------------

So now we can send messages across threads, but what if we need multiple threads to have access to the _same_ data,
maybe even able to edit that data. To do this, we need to use types that implement the `Sync` trait.

Something is `Send` if it can be sent between threads, but doing this moves ownership from one thread to another.

Something is `Sync` if a reference to it can be sent between threads, i.e. `T` is `Sync` if `&T` is `Send`.

Most things are `Sync`, but we still have to abide the rules of references in that we can have as many immutable
references as we like, but we can only have one mutable reference. Furthermore, references cannot outlive the data they
reference... which is a little harder to confirm with threads. How do you know the thread referencing your data doesn't
exist for longer than the data it's referenced?

This is where `std::thread::scope` can help us, by providing scoped threads.

```rust
// We will create a scope and use that to spawn threads instead of spawning 
// them directly.
use std::thread::scope;

fn main() {
    let mut data = String::from("This data is owned by the main thread");

    // The scope function takes a closure with a single parameter that contains
    // the scope context. You use the context to spawn threads
    scope(|s| {
        (0..10).for_each(|_| {
            // We don't _need_ to track the join handler this time, all scoped
            // threads are joined at the end of the scope closure, but if you
            // want to handle a potential thread panic, you can still do that
            // in a scoped thread, by joining the join_handle you get from
            // the `.spawn` method like you would with an unscoped thread from
            // the `spawn` function.
            s.spawn(|| {
                println!("Thread accessing data {}", &data)
            });
        });
    });

    // All scoped threads are joined before the scope function ends, so we are
    // safe to modify the original data.
    data.push_str(" still");

    assert_eq!(&data, "This data is owned by the main thread still");
}
```

This also works with mutable references but, bear in mind, only one thread can access the mutable reference, and it must
end before we can access our data again.

```rust
use std::thread::scope;

fn main() {
    let mut data = String::from("This is owned by the main thread");

    scope(|s| {
        s.spawn(|| {
            data.push_str(" but can be modified by a child thread");
        });
    });

    assert_eq!(
        &data,
        "This is owned by the main thread but can be modified by a child thread"
    );
}
```

So we can share readable data across multiple threads with immutable references, or share writable data temporarily with
a single thread, but what if we want to share read/write access to data across multiple threads.

Let's start by thinking why we can't do this with just references. When we're using threads, multiple parts of our
program can be executed at the same time. Imagine we have two threads that want to change the data behind a reference
based on what is currently stored there, something simple like each thread wants to multiply the data.

1. Thread 1 reads the value from memory into a register
2. Thread 2 reads the value from memory into a register
3. Thread 1 multiplies the data and stores it back in memory
4. Thread 2 multiplies the data and stores it back in memory

In this situation, we've lost the effect of Thread 1, which _could_ be a bug.

Let's consider a more serious version of this. Imagine the data rather than just being a single value, is an image
stored in an array like structure, and you're applying multiple processes to the image at the same time. This time, if
one thread were to override another's work, we have a much more obvious problem.

To get around this, we need to prevent two threads accessing the same piece of data at the same time. There is a general
software abstraction concept called a "mutex" that makes access to the data MUTually EXclusive. Rust provides it's mutex
through `std::sync::Mutex`.

Once you place data inside a Mutex, to access it again, you need to "lock" the Mutex. If the Mutex is already locked,
then the thread currently trying to access the data needs to wait for the existing lock to be released.

```rust
use std::thread::scope;
use std::sync::Mutex;

fn main() {
    let mut data = Mutex::new(Vec::with_capacity(10));

    scope(|s| {
        (0..10).for_each(|_| {
            s.spawn(|| {
                // .lock() returns a MutexGuard. When it goes out of scope,
                // the lock is dropped. MutexGuard implements Deref and
                // DerefMut for the type inside the Mutex
                let mut guard = data.lock()
                    .expect("another thread with the lock panicked");
                guard.push("Thread reporting in!".to_string());
                // The MutexGuard is dropped after this line
            });
        });
    });

    let guard = data.lock().unwrap();
    assert_eq!(guard.len(), 10);
    guard
        .iter()
        .for_each(|s| assert_eq!(s, &"Thread reporting in!".to_string()));
}
```

However, there's still a slight problem here. We're currently very dependent on using scoped threads because we need our
references to point back to the owned data, but scoped threads aren't the norm. In fact, most of the time you use
threads in Rust, they will probably be abstracted behind some other framework (for example, a web server, a game engine,
or data processing tools).

The problem, of course, is that we don't know when the owned data will go out of scope and no longer be accessible.

We can solve this problem using an Atomic Reference Count. We haven't discussed reference counting yet as it's usually
fairly niche, however, reference counting allows you to share data around an application without needing to clone it and
side stepping complex reference rules. It works by moving the data you want to share onto the heap, and allowing access
through a reference count type. When you clone the reference count value, instead of the data being cloned, it modifies
its internal count of how many clones currently exist. Every time a reference count type goes out of scope, the count is
decreased. Once the count hits zero, there are no further references to the data and so it can be cleaned up.

Now, if you've paid attention as to why we need a Mutex for modifying data across threads, you'll see that using a
normal reference count won't work. If the reference counter is cloned or dropped while also being cloned or dropped in
another thread, you could end up with an inconsistent number count of references, meaning data gets dropped at the wrong
time. This is why we need a special reference count type, `std::sync::Arc`, an Atomic Reference Count.

Atomic data types guarantee atomic changes. Atomic changes are guaranteed to appear to be instantaneous to all external
observers, meaning that two threads can change the value, but that this change cannot overlap. `Arc` is a little slower
than Rusts built in basic reference counting type `std::rc::Rc`, but prevents corruption across threads.

> Authors note: I don't think I've _ever_ used `Rc`, but I use `Arc` all the time, so don't worry that we didn't
> cover it in this book. If you need to pass data around wrapped in its own container its there to use

So, armed with this knowledge, we can go back to unscoped threads!

```rust
use std::thread::spawn;
use std::sync::{Arc, Mutex};

fn main() {
    let mut data = Arc::new(Mutex::new(Vec::with_capacity(10)));

    let handles = (0..10).map(|_| {
        // We'll clone the arc and move it into the thread
        let cloned_arc = data.clone();
        spawn(move || {
            // Arc also impls Deref for its containing type so we can call lock
            // on the Mutex from the Arc
            let mut guard = cloned_arc
                .lock()
                .expect("another thread with the lock panicked");
            guard.push("Thread reporting in!".to_string());
        })
    });

    handles.for_each(|handle| handle.join().expect("thread panicked"));

    let guard = data.lock().unwrap();
    assert_eq!(guard.len(), 10);
    guard
        .iter()
        .for_each(|s| assert_eq!(s, &"Thread reporting in!".to_string()));
}
```

Next Chapter
------------

Macros! We'll be looking at the `macro_rules!` macro that lets you make more macros. We'll learn how `macro_rules!` 
is used, how we can use it to remove repetitive code or even write our own domain-specific language (DSL!
