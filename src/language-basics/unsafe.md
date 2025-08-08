Unsafe Rust
===========

One of the best features of Rust is that it keeps you safe.

The reason that Rust has ownership rules and borrow checkers is to make sure that we, the software engineers, don't do
anything too risky, unsafe. So the obvious question is, why would we _want_ to do unsafe things... (other than for the
thrill of it)?

Rust is safe, but Rust is not an island.

Sometimes when we use Rust, we need to access heap memory, use peripherals, and talk to other software. We can't
guarantee that any of these actions are safe.

But wait! Haven't we _been_ accessing Heap Memory throughout this book? Yes, we have. Types like `Vec` and `String`
use the Heap to store their data. They take responsibility for and abstract away unsafe work, meaning that _using_
`Vec`, `String` and similar types _is_ considered safe.

As a Rust engineer, most of the time you won't personally need to worry about unsafe Rust. You can get by with using
other people's APIs like the standard library. The point of this chapter isn't to prepare you to write lots of unsafe
Rust; it's to make you feel comfortable for the odd occasion you might have to touch it.

Recap on Memory
---------------

Typically, when we talk about memory in programming, we're talking about RAM, but even then we subdivide memory into
three main types of usages, each of which has different pros and cons.

Stack Memory is where all variables live. It's very fast because the entire stack is pre-allocated when your program
runs, but there are some catches. 

First, it's actually quite limited. When your program starts its given an amount of memory that you can not determine
ahead of time, and you can not change. It's _usually_ 2MiB, but you might find it's less on things like embedded 
devices. If you go over this small amount, your program will crash.

Second, stack data must be of known size at compile time. You don't really need to worry about why this is (it's because
stack frames, the memory for each function on the stack must have a known size) only that your data can not change in
size.

But wait! We've stored lots of things in variables that have variable size; Strings, Vec's, HashMaps, etc. The data for
these types is not actually stored in the variable. What typically happens is that data is stored on the Heap, and the
data's location, which is of known size, is stored on the stack.

Semantically, it's probably fine to say that the variable contains that data, people will always know what you mean.
However, for the purpose of this chapter, we really need to differentiate what is on the stack and what isn't.

In the below code, `number_example` stores the actual number on the heap, since its of a known size, 32 bits unless
otherwise specified. `string_example`, however, contains the location of the string data, not the data itself, which is
stored on the heap.

```rust
fn main() {
    let number_example = 42;
    let string_example = String::from("This is string data too");
}
```

So, the Heap, is where we can store things of arbitrary size, and we can (usually) store things far larger than the 
stack. Technically, we can't resize heap allocations, but we can request new, larger portions of heap memory, copy our
data there, free the old memory and add more things in the new location until we run out of space again.

So it's bigger and more flexible than the stack, but, it also has some downsides. It's much slower than stack memory
because it must be allocated on request, and freed once we're done with it. Allocation and Freeing in Rust is usually
handled by the standard library and, other than what we're going to discuss in this chapter, you almost never need to
think about that process. However, it still takes time.

> Note: Once heap memory is allocated, it's _almost_ free to use, with the only overhead essentially being the
> redirection from the stack to the heap in O(1) time. For this reason, some software will actually allocate large 
> amounts of memory called Page Files that can store lots of different things. This can be done in Rust too and there
> are pros and cons to this too, but it's far outside the scope of this guide.

There's a third kind of memory we don't really talk about as much but it might be the most important.

Static Memory is where all values and data you write into your program are initially stored, though often times its
subsequently moved somewhere else. For example, in the program:

```rust
const CONST_EXAMPLE: &str = "This is string data";

fn main() {
    let number_example = 42;
    let string_example = String::from("This is string data too");
}
```

The data for `CONST_EXAMPLE` remains in static memory, but similar to variables that contain heap data locations,
`CONST_EXAMPLE` itself is a reference to that data (note the`&`). `42` and `"This is string data too"` are also
initially stored in static data, however, `42` is copied to the stack in `number_example` whereas 
`"This is string data too"` is cloned onto the heap and the location of the data is stored in `string_example`.

Differentiating where things are stored is about to become _very_ important, and it's easy to make mistakes if we don't
differentiate between the stack, the heap and static memory. (Thank-you [@ChillFish8](https://github.com/ChillFish8)
for helping me out when I made that exact mistake writing this chapter 😉)

Not really all that unsafe
--------------------------

It's important to note that Unsafe Rust doesn't turn _off_ any of Rusts safety measures, what it does do is turn _on_ a
whole other set of language features on which Rusts safety tools cannot work.

I really can't stress this enough as it might be one of the greatest misconceptions in Rust. Unsafe Rust does _not_
turn off _any_ safety measures. It turns on tools that Rust cannot guarantee are safe, so you need to make extra
certain you are using them safely. 

For example, in safe Rust we use references. These are similar to pointers in other languages, but they are not
pointers. References in unsafe Rust still must abide by the rules of the borrow checker. Unsafe Rust doesn't turn off
the borrow checker, instead it gives us access to raw pointers which can't be borrow checked.

Unlike what you might have been lead to believe, unsafe Rust is not the wild west, and you will not lose all control
simply by using it. Being mindful of the language features that are unsafe will help keep you focused on writing sound
code.

Some of these tools exist in other commonly used low-level languages that have been around for decades and are still,
rightly, popular today. In these languages, these tools are available at any time. They're necessary tools that we
need to do things that there is no other way to do.

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
/// This function doesn't _actually_ do anything, therefore, you don't need to 
/// do anything in particular to use it safely.
unsafe fn this_code_is_unsafe() {}
```

What's with all the comments?

This is not necessarily a widely used practice, however, the Rust Standard Library team, who have to work with `unsafe`
a lot, have standardized around making safety communication almost contractual.

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
[Standard library developer's Guide](https://std-dev-guide.rust-lang.org/policy/safety-comments.html)

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
anywhere in your application, you want to minimize memory footprint (this will only appear in the binary itself) and,
importantly, you never want to change it.

In the [Threads](./threads.md#sharing-state) chapter, we briefly discussed the danger of modifying data across multiple
threads. Let's get `unsafe` with it.

Rust allows you to mutate `static`s making the static mutable also makes it `unsafe`.

```rust
static mut HELLO_MESSAGE: &str = "Hello!";

fn main() {
    another_function();
    
    // SAFETY: We only ever modify this variable from the main thread, 
    // HELLO_MESSAGE is never used by other threads
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

Notice that it's not just unsafe to write to the static, it's also considered unsafe to read from it. However, so long as
we never modify this in a different thread, we know this behavior is safe.

Raw Pointers
------------

Our previous example was pretty tame. We were using static data, so, although there was some risk with relation to
threads, it was still on the safer side. Let's play with fire.

We use References in Rust a bit like other languages use pointers, but references have a number of features that make
them safer to use. A pointer is essentially just a number that is an address to a location in memory. When you allocate
heap data, even in Rust, the operating system amongst other things provides you with a pointer to the location where the
memory was allocated.

If we just used a pointer, it would still contain an address to that location even if we subsequently told the
operating system to free that memory. Programmatically, we have no way to know if that location is still ours to use
later. Using that pointer after the memory it's been pointed to has been freed is, well, unsafe, and is the root of an
extremely common bug you might have heard of, use after free.

In fact, because we don't know from just the pointer whether the memory was freed or not, we might try to free the
memory again, leading to another bug "double free".

References help us avoid that because we can track their use at compile time, helping us make sure that they are always
valid before we even run the code... but the operating system doesn't use references. Actually, pointers can't be used
between _any_ two separate pieces of software, because of the compile time nature of them. We can, however, share 
pointer locations.

So, even in Rust, we occasionally need to deal with pointers.

You can actually get pointers in safe Rust. Try running this program multiple times:

```rust
let hello = String::from("Hello, world!");
let pointer = &raw const hello;
println!("{hello} is located at {}", pointer as usize);
```

If you run this code multiple times, you should get a different number every time (this may depend on underlying memory
management)

What we can't do is use those pointers to get data in safe Rust. For that we need to dip into unsafe. Below we
dereference the pointer to go back from the numeric location to the data that's stored there.

```rust
let hello = String::from("Hello, world!");
let pointer = &raw const hello;
// SAFETY: The string data `pointer` points to has not been freed
unsafe {
    println!("At location {} is the string '{}'", pointer as usize, *pointer);
}
```

This is unsafe because the validity of the pointer cannot be confirmed. If we dropped the String before the unsafe 
block, this code would still compile, but there's now a serious risk that the data at that location no longer represents
our string.

There is more that we can do with raw pointers, which we'll come to later.

Unsafe functions
----------------

When we write code, we regularly break it up into small reusable chunks known as functions. You are, at this point, I
hope, very familiar with this idea.

So far we've demonstrated that we can place unsafe code inside a block to encapsulate unsafe behavior. This means that
you can write unsafe code inside a function, but the function makes sure that there's no risk, meaning calling the 
function itself _is_ safe.

A good example of this is the [`std::mem::swap`](https://doc.rust-lang.org/std/mem/fn.swap.html) which swaps the values
at two mutable locations:

```rust
let mut left = "Left".to_string();
let mut right = "Right".to_string();

println!("Left is {left}");
println!("Right is {right}");

std::mem::swap(&mut left, &mut right);
    
assert_eq!(left, "Right".to_string());
assert_eq!(right, "Left".to_string());

println!("Left is {left}");
println!("Right is {right}");
```

Because `swap` guarantees the types are the same and, through using mutable references, knows nothing else is accessing
the memory while it does its thing, conceptually this function is safe, even if the first thing it does internally is
run unsafe code. This is what we call a safe abstraction around unsafe code.

But that's not always possible. Sometimes, the very concept a function or method represents is unsafe.

Let's say that through arbitrary means, we've got a pointer to some heap data that we know represents a String. We know
how long the String is and how much memory at that location is ours. We want to take ownership of that memory and turn
it into a `String` type.

We can use the `from_raw_parts` on the `String` type to build a `String` directly from memory, but the entire concept
of manually creating a string like this is unsafe.

Firstly, something else likely manages that heap memory. If we create a `String` from it, we're going to take ownership
of the heap data, and when the String goes out of scope, Rust will try to free it. How do we prevent a double free when
the thing that originally created the data also wants to free it.

Secondly, `from_raw_parts` takes a pointer, a length, and a capacity, none of which it can work out is valid at compile
time.

Remember, length and capacity of collections including `String` are different. Length is how much data is being used
currently. Capacity is how much memory is available to contain the data. Most types will cause a reallocation when the
capacity is filled, causing us another problem to look out for!

Luckily, by being aware of the problems ahead of time, we can still use this function safely.

```rust
use std::ops::Deref;

fn main() {
    // We'll manually make sure our string never exceeds 100 bytes
    let capacity = 100;

    let mut original_string = String::with_capacity(capacity);
    // 57 ascii chars = 57 bytes
    original_string.push_str("This string is a longer string but less than the capacity");

    let pointer = &raw mut original_string as *mut u8;

    // SAFETY: We create a string from the original, but we prevent the new string
    // from being moved by staying inside its capacity, and we prevent it being
    // dropped by using ManuallyDrop.
    unsafe {
        let overlapping_string = String::from_raw_parts(pointer, 15, capacity);

        // Before we do anything else, we're going to prevent overlapping_string 
        // from being dropped, which will cause a double free when original_string
        // is dropped. We could equally prevent the original string being dropped,
        // but because of scoping it's safer to do it this way around. 
        // 
        // The ManuallyDrop type is also unsafe
        let mut overlapping_string = std::mem::ManuallyDrop::new(overlapping_string);

        assert_eq!(overlapping_string.deref(), &"This string is ".to_string());
    }
}



```
