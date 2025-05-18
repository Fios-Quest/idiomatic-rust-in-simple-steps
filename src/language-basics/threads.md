Threads
=======

Threads allow us to build programs where parts of the program can run independently of one another.

This _can_ (big emphasis on "can") help you make faster and more responsive programs.

For example:
- As a web developer, I like to know that my server framework of choice can start responding to the next request before
  it's finished responding to the previous request. 
- Game developers would like their program to be able to capture player inputs without being blocked by the renderer. 
- And data engineers often want to process huge datasets in parallel.

We can do all of this by asking the operating system to run parts of our program as a separate thread. These separate
parts of the program can communicate with each other in a variety of ways, enabling the program to work as a single
whole.

We'll step through:
- how we can run code in a thread, including sending data before it starts
- how we can wait for a thread to end, including receiving data when it ends
- how we can communicate with running threads
- how we can share state between threads

We'll also be touching again on our marker traits Send and Sync

Starting a thread
-----------------

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
        10
    });

    assert_eq!(handle.join().unwrap(), 10);
}
```

Stopping a thread
-----------------

Sending messages
----------------

Sharing State
-------------

