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

Notice that it's not just unsafe to write to the static, it's also considered unsafe to read from it. However, so long
as we never modify this in a different thread, we know this behavior is safe.

There's a catch here though. Remember, `HELLO_MESSAGE` is a reference to some data that exists in static memory. What
we've done here is change the reference itself to point to the location of `"CHANGED!"` which is also built into the
programs static memory.

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

You can actually get pointers in safe Rust. Try running this program multiple times, you should get a different number
every time:

```rust
let the_answer = 42;
let pointer = &raw const the_answer;
println!("The variable at {pointer:p} the data '{the_answer}'");
```

One cool thing worth pointing out is that Rust even types your pointers making it harder to muddle them up later. 

```rust
# use std::any::type_name;
# 
# // Thanks to @DevinR528 https://github.com/DevinR528
# // Source: https://users.rust-lang.org/t/how-check-type-of-variable/33845/2
# fn type_of<T>(_: T) -> &'static str {
#     type_name::<T>()
# }
# 
# fn main() {
    let the_answer = 42;
    let pointer = &raw const the_answer;
    println!("{}", type_of(pointer));
# }
```

Remember, in some circumstances, the variable that you're accessing the data via, does not contain the actual data.
Strings are a really, good example of this. The pointer to the variable does not point to the data, but we can access
the pointer to the data via a method on the string itself (this is inherited from string slices). Again, there's nothing
unsafe about doing this.

```rust
let hello = String::from("Hello, world!");
let pointer_to_variable = &raw const hello;
let pointer_to_data = hello.as_ptr();
println!(
    "The variable at {pointer_to_variable:p}, \
    points to {pointer_to_data:p} \
    which contains the data '{hello}'",
);
```

Getting pointers is perfectly safe, what we can't do is use those pointers to get data in safe Rust. For that we need to
dip into unsafe. Below we dereference the pointer to go back from the location to the data that's stored there.

```rust
let the_answer = 42;
let pointer = &raw const the_answer;
unsafe {
    let data_at_pointer = *pointer;
    assert_eq!(data_at_pointer, 42);
}
```

This is unsafe because the validity of the pointer cannot be confirmed. `pointer` could outlive `the_answer`, after 
which, what is pointer pointing at? There's no real way to know.

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

    let pointer = original_string.as_mut_ptr();

    // SAFETY: We create a string from the original, but we prevent the new string
    // from being moved by staying inside its capacity, and we prevent it being
    // dropped by using ManuallyDrop.
    unsafe {
        // `from_raw_parts` is an `unsafe` method so can only be called inside an
        // unsafe block
        let overlapping_string = String::from_raw_parts(pointer, 15, capacity);

        // Before we do anything else, we're going to prevent `overlapping_string` 
        // from being dropped, which would otherwise cause a double free when 
        // `original_string` is dropped later. We could equally prevent 
        // `original_string` being dropped instead, but, to me, it makes sense to
        // have this behaviour in the inner code block.
        // 
        // The ManuallyDrop type is actually safe. Although it prevents the memory
        // being freed, and _could_ result in memory leaks that's not considered
        // unsafe in the same way as other things in this chapter. Just be careful
        // using it.
        let mut overlapping_string = std::mem::ManuallyDrop::new(overlapping_string);

        // Confirm we have the right location!
        assert_eq!(overlapping_string.deref(), &"This string is ".to_string());
        
        // Push some data onto the back of the string
        overlapping_string.push_str("short");
        
        assert_eq!(overlapping_string.deref(), &"This string is short".to_string());
    }
    
    // Un oh!
    assert_eq!(original_string, "This string is shortger string but less than the capacity");
}
```

It's not unusual to create an unsafe function (because conceptually what you're doing is unsafe, like creating a string
directly from memory), but then wrap that function in safe code. For example, the internal representation of data inside
the String is a slice, which also has the method `from_raw_parts` (though it works slightly differently as slices don't
have capacity, just length). `slice::from_raw_parts_mut` is unsafe, but it's used inside the safe method
`String::retain`.

Creating safe abstractions might look something like this:

```rust
// SAFETY: To use this safely you must be sure of the following:
// - obviously there isn't a checklist here because this function does nothing
unsafe fn conceptually_dangerous_function() -> bool {
    true
}

fn safe_abstraction() -> bool {
    // Do some checks
    
    // SAFETY: We confirmed safety by doing the following checks
    // - again, the function does nothing so nothing to really check here
    unsafe {
        conceptually_dangerous_function()
    }
}

fn main() {
    // We can safely call the safe abstraction to do unsafe things safely
    let output = safe_abstraction();
    assert!(output);
}
```

It's worth noting that if you have a trait where any of its methods are unsafe, then the entire trait is considered
unsafe, and so is it's implementation. It's actually kind of rare to _have_ to use this feature. If you're trait has
an unsafe method but a safe abstraction, you could move the unsafe method to an unsafe function.

For example, the trait has two provided methods, but we still can't implement it safely. 

```rust,compile_fail
unsafe trait WontWork {
    // SAFETY: This method isn't actually unsafe
    unsafe fn conceptually_dangerous_method(&self) -> bool {
        true
    }
    
    fn safe_abstraction(&self) -> bool {
        // SAFETY: The method called isn't actually unsafe
        unsafe {
            self.conceptually_dangerous_method()
        }
    }
}

struct ExampleUnitType;

impl WontWork for ExampleUnitType {}
```

However, if we don't need to ever overwrite the unsafe method, we could just extract it from the trait entirely

```rust
// SAFETY: This method isn't actually unsafe
unsafe fn conceptually_dangerous_method<T: WillWork + ?Sized>(w: &T) -> bool {
    true
}

trait WillWork {
    fn safe_abstraction(&self) -> bool {
        // SAFETY: The method called isn't actually unsafe
        unsafe {
            conceptually_dangerous_method(self)
        }
    }
}

struct ExampleUnitType;

impl WillWork for ExampleUnitType {}
```

You're likely to need unsafe Traits only when the behaviour the trait describes itself is unsafe. For example, `Send`
and `Sync` are automatically applied to all types that are only constructed from types that are also `Send` and `Sync`.
If your type contains types that are not `Send` and/or `Sync` then the compiler can no longer guarantee safety itself.
You can still implement `Send` and `Sync` for your type manually but its now up to you to check the implementation is
safe, so the traits themselves are `unsafe`.

Unions
------

Unions, in software engineering, are a way of storing different types in the same section of memory. They're typically 
broken into two types, tagged and untagged. "Tagged" simply means the type is part of data, so you can only access the
data _as_ the type that it is. We use tagged unions in Rust all the time, and they're perfectly safe:

```rust
enum ThisIsATaggedUnion {
    Number(u64),
    Character(char),
}
```

Enums are tagged unions, they only ever take up as much memory as is taken by the largest data type representable inside
them, plus a discrimination value which differentiates the variants at runtime (usually represented as an isize but
Rust compilers _may_ use smaller numeric types):

```rust
enum ThisIsATaggedUnion {
    Number(u64),
    Character(char),
}

fn main() {
    let number = ThisIsATaggedUnion::Number(42);
    let character = ThisIsATaggedUnion::Character('c');
    
    assert_eq!(size_of_val(&number), size_of_val(&character));
    assert_ne!(size_of_val(&'c'), size_of_val(&character));
    
    println!("Size of character: {} bytes", size_of_val(&'c'));
    println!("Size of u64: {} bytes", size_of_val(&42_u64));
    
    let discriminant = std::mem::discriminant(&number);
    println!("Size of enum discriminant: {} bytes", size_of_val(&discriminant));
    
    println!("Size of enum number: {} bytes", size_of_val(&number));
    println!("Size of enum character: {} bytes", size_of_val(&character));
}
```

But Rust also has "untagged" unions, where the type being used is not part of the data, and you can access the data as
either type. This is obviously unsafe but provide several useful features, either by interrogating the data in different
ways, or for working with other programming languages that use untagged unions.

> Note: My first attempt at an example for unions was an IPv4 addresses that used both a 32bit integer, _and_ a 4 byte
> array, however, with that example we have to consider "endianness" which is the order in which bytes are stored in
> memory. This felt like it went too far off-topic, however its still worth pointing out that when creating unions that
> share multiple bytes of data, you _may_ need to consider endianness.

In this example, we can interrogate characters as u32's (characters in Rust are four bytes, although most string types
use a variable byte width encoding such as utf-8).

```rust
union CharOrNumber {
    number: u32,
    character: char,
}

fn main() {
    // Creating unions is safe:
    let mut h = CharOrNumber { character: 'O' };
    // Reading unions is unsafe 
    unsafe {
        println!("The numeric value of the character {} is 0x{:x}", h.character, h.number)
    }
    
    // Writing values is safe, 
    h.character = 'o';
    
    // See how both character and number change
    unsafe {
        println!("The numeric value of the character {} is 0x{:x}", h.character, h.number)
    }
}
```

Assembly
--------

This next example of unsafe is so incredibly unsafe the only time you're ever likely to use it is if you need insane
speed and know _exactly_ what you're doing with the _exact_ hardware you're targeting.

You might have heard of assembly, but crucially its not one language. Assembly languages are any language that have a
near 1:1 relationship with the actual instructions of the CPU you're building for.

In the below example you can see a function that takes a number and multiplies it by 6 using assembly. There are two
versions of the function, one that works using the "x86_64" (most Windows and Linux machines and very old macs) and
another that works using "aarch64" (all modern Macs but some newer Windows and Linux machines). As you can see, apart
from `mov`, the other instructions look very different but do the same things.

```rust
use std::arch::asm;

#[cfg(target_arch = "x86_64")]
fn multiply_by_six(input: u64) -> u64 {
    let mut x = input;
    unsafe  {
        asm!(
            "mov {tmp}, {x}",
            "shl {tmp}, 1",
            "shl {x}, 2",
            "add {x}, {tmp}",
            x = inout(reg) x,
            tmp = out(reg) _,
        );
    }
    x
}

#[cfg(target_arch = "aarch64")]
fn multiply_by_six(input: u64) -> u64 {
    let mut x = input;
    unsafe  {
        asm!(
            "mov {tmp}, {x}",
            "lsl {tmp}, {tmp}, #1",
            "lsl {x}, {x}, #2",
            "add {x}, {x}, {tmp}",
            x = inout(reg) x,
            tmp = out(reg) _,
        );
    }
    x
}

fn main() {
    assert_eq!(multiply_by_six(4), 24);
    println!("4 * 6 is {}", multiply_by_six(4));
}
```

For obvious reasons Rust can not help keep you safe when you're sending instructions straight to the CPU, so assembly is
only available within unsafe code. Of all Rust's unsafe features, this is the one you're least likely to need to touch,
but, as with the others, it's there if you need it.

Summary
-------

Outside of specialist use cases, you're unlikely to have to write much, if any, unsafe code yourself. Nonetheless,
hopefully after this chapter you see that it's not as scary as it seems. You still have all the normal safety checks
plus some additional features, and, now you know what to look for to keep yourself safe when the compiler can no longer
help.

If you are going to be writing unsafe Rust, there's a tool called [Miri](https://github.com/rust-lang/miri) that will
help you detect potentially undefined behaviour you might have missed in your running code. It's not a silver bullet but
is a final layer of protection you should use to protect yourself.

Next Time
---------

We're going to lean into pretty much everything we've learned so far to learn async Rust. This is going to be a bit of
a weird chapter. We'll go deeper than you generally need to go in our exploration of the space (typically you would just
grab a crate to do all the hard stuff), but you should come out the other side with a much better idea of how async
works under the hood, and feel comfortable with what I think many would agree is the last remaining truly sharp edge of
Rust programming.


