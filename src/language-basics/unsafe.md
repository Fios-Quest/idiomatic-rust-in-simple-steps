Unsafe Rust
===========

One of the best features of Rust is that it keeps you safe.

The reason that Rust has ownership rules and borrow checkers is to make sure that we, the software engineers, don't do
anything too risky, unsafe. So the obvious question is, why would we _want_ to do unsafe things... (other than for the
thrill of it)?

Rust is safe, but Rust is not an island.

Sometimes when we use Rust, we need to access heap memory, utilise peripherals, and talk to other software. We can't
guarantee that any of these actions are safe.

But wait! Haven't we _been_ accessing Heap Memory throughout this book? Yes, we have. Types like `Vec` and `String`
use the Heap to store their data. They take responsibility for and abstract away unsafe work, meaning that _using_
`Vec`, `String` and similar types _is_ considered safe.

As a Rust engineer, most of the time you won't personally need to worry about unsafe Rust. You can get by with using
other peoples APIs like the standard library. The point of this chapter isn't to prepare you to write lots of unsafe
Rust, its to make you feel comfortable for the odd occasion you might have to touch it.

Not really all that unsafe
--------------------------

It's important to note that Unsafe Rust doesn't turn _off_ any of Rusts safety measures, what it does do is turn _on_ a
whole other set of language features on which Rusts safety tools can not work.

I really can't stress this enough as it might be one of the greatest misconceptions in Rust. Unsafe Rust does _not_
turn off _any_ safety measures. It turns on tools that Rust can not guarantee are safe, so you need to make extra
certain you are using them safely. 

For example, in safe Rust we use references. These work a lot like pointers in other languages, but they are not
pointers. References in unsafe Rust still must abide by the rules of the borrow checker. Unsafe Rust doesn't turn off
the borrow checker, instead it gives us access to raw pointers which can't be borrow checked.

Unlike what you might have been lead to believe, unsafe Rust is not the wild west, and you will not lose all control
simply by using it. Being mindful of the language features that are unsafe will help keep you focused on writing sound
code.

Some of these tools exist in other commonly used low level languages, that have been around for decades and are still,
rightly, popular today. In these langauges, these tools are available at any time. They're necessary tools that we
need in order to do some things that there is no other way to do.

How to use unsafe
-----------------

Any time we use unsafe code we need to wrap it inside an `unsafe` block. The code below uses an `unsafe` block to call a
function that is itself marked as `unsafe`. Because the function is marked as `unsafe` it can _only_ be called within
`unsafe` code, however, even within that function, code is treated as safe until you use another `unsafe` block. We'll
talk about what it means to mark functions as `unsafe` further on.

```rust
fn main() {
    // SAFETY: This function is a no-op
    unsafe {
        this_code_is_unsafe();
    }
}

/// # Safety
/// 
/// This function doesn't actually do anything therefore you don't need to do
/// anything in particular to use it safely.
unsafe fn this_code_is_unsafe() {}
```

What's with all the comments?

This is not necessarily a widely used practice, however, the Rust Standard Library team, who have to work with `unsafe`
a lot, have standardised around making safety communication almost contractual.

Prior to the `unsafe` block, the first thing we see is a `SAFETY:` comment. This tells the reader how the author made
sure this code was safe. This may seem odd. If the code is provably safe, why do we need `unsafe` at all? `unsafe` turns
on language features that can't be proven safe by the compiler, but that's no excuse for writing `unsafe` code unsafely
though.

The practice of writing a `SAFETY:` comment ensures that when we write `unsafe` code, we think hard about how we know
this code isn't going to hurt us later. Documenting how we know this code is safe is crucial. When you write or review 
`unsafe` code, it's crucial to make sure this comment covers as many foreseeable cases as possible. We'll see more
examples as we go.

The `unsafe` function also has "Safety" doc comment. It's a doc comment because it's for people who are going to consume
this function. It explains how to make sure you use the function safely in the API documentation.

You can read more about this practice in the official 
[Standard library developers Guide](https://std-dev-guide.rust-lang.org/policy/safety-comments.html)

Mutable Statics
---------------

There's a type of "variable" that can be read from anywhere, the static.

```rust
static HELLO_MESSAGE: &str = "Hello!";

fn main() {
    println!("This function can read HELLO_MESSAGE without having ownership: {HELLO_MESSAGE}");
    another_function();
}

fn another_function() {
    println!("So can this one: {HELLO_MESSAGE}");
}
```

Static variables are a bit like global variables in other languages. They're really handy if you want to read data from
anywhere in your application, you want to minimise memory footprint (this will only appear in the binary itself) and,
importantly, you never want to change it.

In the [Threads](./threads.md#sharing-state) chapter, we briefly discussed the danger of modifying data across multiple
threads. Let's get `unsafe` with it.

Rust allows you to mutate `static`s making the static mutable also makes it `unsafe`.

```rust
static mut HELLO_MESSAGE: &str = "Hello!";

fn main() {
    another_function();
    
    // SAFETY: We only ever modify this variable from the main thread, HELLO_MESSAGE is never used by other threads
    unsafe {
        HELLO_MESSAGE = "CHANGED!";
    }
    
    another_function();
}

fn another_function() {
    // SAFETY: This function is only called in the main thread
    unsafe {
        println!("HELLO_MESSAGE is currently: {HELLO_MESSAGE}");
    }
}
```

Notice that it's not just unsafe to write to the static, its also considered unsafe to read from it. However, so long as
we never modify this in a different thread, we know this behaviour is safe.

Raw Pointers
------------

Our previous example was pretty tame. We were using static data so, although there was some risk with relation to
threads it was still on the safer side. Let's play with fire.

We use References in Rust a bit like pointers are used in other languages, but references have a number of features
that make them safer to use. A pointer is essentially just a umber that is an address to a location in memory. When
you allocate heap data, even in Rust, the operating system amongst other things provides you with a pointer to the
location where the memory was allocted.

If we just used a pointer, it would still contain an address to that location even if we'd subsequently told the
operating system to free that memory and, programatically, we have no way to know if that location is still ours to use
later. Using that pointer after the memory its been pointed to has been freed is, well, unsafe, and is the root of an
extremely common bug you might have heard of, use after free.

In fact, because we don't know from just the pointer whether the memory was freed or not, we might try to free the
memory again, leading to anothr bug "double free".

References help us avoid that because we can track their use at compile time, helping us make sure that they are always
valid before we even run the code... but the operating system doesn't use references. Actually, pointers can't be used
between _any_ two separate pieces of software, beecause of the compile time nature of them. We cn however share pointer
locations.

So, even in Rust, we occassionally need to deal with pointers.

You can actually get pointers in safe Rust. Try running this program multiple times:

```rust
fn main() {
    let hello = String::from("Hello, world!");
    let pointer = &raw const hello;
    println!("{hello} is located at {}", pointer as usize);
}
```

If you run this code multiple times, you should get a different number every time (this may depend on underlying memory
management)

What we can't do is use those pointers to get data in safe Rust. For that we need to dip into unsafe:


```rust,compile_fail
fn main() {
    let hello = String::from("Hello, world!");
    let pointer = &raw const hello;
    unsafe {
        println!("At location {pointer} is the string {}", *hello);
    }
}
```

Ah, but this doesn't work. The reason is that strings, in any langauge, take up more than one word of memory. The
compiler fails because (incoming pun), 

