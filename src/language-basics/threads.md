Threads
=======

Threads allow us to build programs where parts of the program can run independently of one another.

Threads _can_ (big emphasis on "can") help you make faster and more responsive programs.

For example:
- As a web developer, I would like the server framework I'm using to start responding to the next request before it's
  finished responding to the previous request
- As a game developer, I  would like my game engine to capture player inputs without being blocked by the renderer
- As a data engineer, I would like to process large sets of data in parallel

We'll step through:
- how we can run code in a thread, including sending data before it starts
- how we can wait for a thread to end, including receiving data when it ends
- how we can communicate with running threads
- how we can share state between threads

We'll also be touching again on our marker traits Send and Sync

what is a thread
----------------

Before we get into the Rust, its worth discussing what a thread is.

When you run a program, that specific instance of the program is called a "process". The process incorporates not just
the instructions to be run but is an abstraction around various resources that the program has access to, such as
memory.

You can run multiple processes which the opperating system will schedule seperately which could allow you to do more
things at once, however, those process won't (or at least, shouldn't) have access to the same memory. There are ways
to communicate between processes, but they can be slower and more restrictive than if we could share memory.

The part of the process responsible for executing your code is called a thread, and a single process can have multiple
threads. Threads are scheduled by the operating system independently, allowing one process to do multiple things
effectively concurrently.


Starting a thread
-----------------

You're program always has at least one thread, even your basic hello-world program runs in a thread.

```rust
fn main() {
    println!("Hello, I am in a thread!")
}
```

What we're interest in today is how we start more threads. This is a process called "spawning".

To spawn a thread, we use `std::thread::spawn`... but, this won't do much on its own. Run the code below, see what's
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
it. When the join handler is dropped, the thread is orphaned, however, it may still run. In this case, the process ends
at the end of main so our spawned thread never got a chance to do anything.

We pause our main thread can wait for a running thread using the join handler.

```rust
use std::thread::spawn;

fn main() {
    println!("This is the main thread");
    let handler = spawn(|| {
        println!("This is a child thread");
    });
    handler.join().expect("child thread panicked");
    println!("This is the end of the main thread");
}
```


Note, we're using closures here, but functions work just as well and can be better for more complex programs. The only
restriction is it needs to be `FnOnce() -> T + Send + 'static`. See
[the documentation](https://doc.rust-lang.org/std/thread/fn.spawn.html) for more details.


```rust
use std::thread::spawn;

fn child() {
    println!("This is also a child thread");
}

fn main() {
    println!("This is the main thread");
    let handler = spawn(child);
    handler.join().expect("child thread panicked");
    println!("This is the end of the main thread");
}
```

When threads are allowed to execute code is controlled by a scheduler which we can't directly control ourselves, but
we can influence it. Putting one thread to sleep can allow another thread to run. Run this code, then uncomment the
commented lines and run it again.

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

    handler.join().expect("child thread panicked");
}
```

So now we can run threads, lets start looking at how to send data back and forth between them.

We can pass data into a thread before it starts so long as the data is is `Send`. We previously talked about this trait
in the [Traits](./traits.md) chapter, but to recap, data is `Send` so long as it can be safely sent between threads,
and this trait is auto-implemented for all types that can be `Send` (though it is also possible to opt out of it).

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

    handler.join().expect("child thread panicked");
}
```

You can also return data via the join handler. This means you could pass hard work to a thread and do other work,
coming back to check on the thread at a later time. We can check if the thread is finished with `.is_finished()`;

```rust
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let data = u16::MIN..u16::MAX;

    let handler = spawn(move || {
        data.map(|i| i as u32).sum::<u32>()
    });

    while !handler.is_finished() {
        println!("still working");
        sleep(Duration::from_nanos(100));
        
    }

    let answer = handler.join().expect("child thread panicked");

    println!("Got out answer: {answer}");
    assert_eq!(answer, 2147385345);
}
```

Sending messages
----------------

Now we can start one thread, there's no stopping us!

Modern schedulers handle a _lot_ of threads at once, however, so far we can only send data to between a child thread
and the parent that started it. What if we want to communicate across multiple threads, or send data to a thread after
we already started it?

Multi-producer, single-consumer (MPSC) allow us to create channels with `Sender`s that can send messages, and
`Reciever`s that can recieve them. As per the name, Multi-producer, you can clone `Sender`s each of those clones can
only send to a single `Reciever`. `Sender` and `Receiver` are `Send` meaning that you can create them in one thread
and send them to another.

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
        let cloned_sender = sender.clone();
        // move the cloned sender to the next thread
        spawn(move || {
            cloned_sender
                .send(format!("Reporting in from thread {id}"))
                .expect("could not send");
        })
    });

    let receiving_handler = spawn(move || {
        while let Ok(message) = receiver.recv() {
            println!("Received message: {message}");
        }
    });


    for handler in sending_handlers {
        handler.join().expect("a sending thread panicked");
    }
    
    receiving_handler.join().expect("receiving thread panicked");
}

```

FWIW, there's no built in way to create a channel with multiple receivers (`Receiver` is not `Clone`), however, there's
nothing stopping you building your own type for that, or there are crates that support it like Crossbeam.

Sharing State
-------------

So now we can send messages across threads, but what if we need multiple threads to have access to the same data, maybe
even able to edit that data. To do this, we need to use types that implement the `Sync` trait.

Something is `Send` if it can be sent between threads, but doing this moves ownership from one thread to another.

Something is `Sync` if a reference to it can be sent between threads, ie, `T` is `Sync` if `&T` is `Send`.

Most things are `Sync`, but we still have to abide the rules of references in that we can have as many immutable
references as we like but we can only have one mutable reference. Furthermore, references can not outlive the data they
reference... which is a little harder to confirm with threads. How do you know the thread referencing your data doesn't
exist for longer than the data it's referencing.

This is where `std::thread::scope` can help us, by providing scoped threads.

```rust
// We will create a scope and use the scope to spawn threads instead of
// spawning them directly.
use std::thread::scope;

fn main() {
    let mut data = String::from("This data is owned by the main thread");

    let thread_ids = 0..10;

    // The scope function takes a closure with a single parameter that contains
    // the scope context. You use the context to spawn threads
    scope(|s| {
        &thread_ids.for_each(|_| {
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

    println!("All scoped threads have completed");

    // All scoped threads are joined before the scope function ends so we are
    // safe to modify the original data.
    data.push_str(" still");

    assert_eq!(&data, "This data is owned by the main thread still");
    
}
```

This also works with mutable references but, bare in mind, only one thread can access the mutable reference and it must
end before we can access our data again.


```rust
use std::thread::scope;

fn main() {
    let mut data = String::from("This data is owned by the main thread");

    let thread_ids = 0..10;

    scope(|s| {
        s.spawn(|| {
            data.push_str(" but can be modified by a child thread");
        });
    });

    println!("All scoped threads have completed");

    assert_eq!(
        &data,
        "This data is owned by the main thread but can be modified by a child thread"
    );
    
}
```


