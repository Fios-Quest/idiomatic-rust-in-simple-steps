Iterators
=========

Iterators are a way to produce and perform operations on a sequence of values.

We often use them with [collections](collections.md) so that we can perform the same operation on each item in a 
collection, or reduce a collection to a single value. They're also often used implicitly in some forms of loop.

We'll talk about these common uses later in the chapter, but to get an understanding of how Iterators work, lets make
our own.

The Iterator Trait
------------------

The Iterator trait can be applied to any type and has a single required method:

```rust
pub trait Iterator {
    type Item;

    // Required method
    fn next(&mut self) -> Option<Self::Item>;
}
```

> ℹ️ Iterator also has 75 provided methods, which perhaps goes to show how incredibly useful and flexible this trait is.
> We'll be talking about a lot of these methods, but it's well worth checking out the documentation to see what else
> is possible. One amazing thing to bear in mind is that some methods have special trait bounds, meaning they're only
> available if the Type the trait is implemented for or the Item being returned abides certain constraints. We'll talk
> about some of these later too!

We're going to build an Iterator that produces the Fibonacci sequence.

The Fibonacci sequence is a sequence of numbers starting `1, 1`[^fib-seq] where each subsequent number is the previous
two added together. Creating a function that returns the nth Fibonacci number is a common question in software
engineering interviews[^fib-interview], intending to show your understanding of recursion.

Instead of using recursion, though, we'll create an iterator type that produces the numbers in sequence. To do this,
we'll create a struct that stores the current state of the Iterator. To keep things simple, we'll use a `u8` to store
the current number (that gives us numbers 0-255), and we'll use an Option to prevent overflowing the `u8`.

```rust,ignore
{{#include iterators/src/bin/fib.rs:struct}}
# fn main() {}
```

The only method we need to provide to implement Iterator is `next`, though we also use an associated type to provide
the information on what is returned from `next`. The reason for the associated type is that it is re-used multiple times
throughout the Provided methods, and there should only ever be one Iterator trait applied to a type, so generics aren't
the right solution.

```rust,ignore
{{#include iterators/src/bin/fib.rs:impl_iterator}}
# fn main() {}
```

So now we have our iterator type! We can obviously get each item off the iterator one at a time:

```rust
# {{#rustdoc_include iterators/src/fibonacci.rs:0}}
# 
# fn main() {
{{#include iterators/src/bin/fib.rs:next}}
# }
```

But that's pretty boring, no one wants to iterate through things by hand. What if we want to print out all the Fibonacci
values that fit inside a `u8`? You can give an Iterator to a `for ... in ...` loop, and it will automatically unwrap the
`Option` for you. Once the loop hits a `None` the loop ends.

This code will print out each number on a new line, try hitting the play button to see it in action!

```rust
# {{#rustdoc_include iterators/src/fibonacci.rs:0}}
# 
# fn main() {
{{#include iterators/src/bin/fib.rs:loop}}
# }
```

That's cool, but on its own, it's still not very interesting.

Iterators are designed to be chained. Those 75 provided methods I mentioned earlier allow you to do some exceptional
tricks. For example, a list of Fibonacci numbers might be more useful if we knew what number in the sequence we're on.
We can chain a method called `.enumerate` which will consume the old iterator and give us a new one where each `next`
now returns a tuple of `(position, T)` where `T` was the original piece of data.

```rust
# {{#rustdoc_include iterators/src/fibonacci.rs:0}}
# 
# fn main() {
{{#include iterators/src/bin/fib.rs:enumerate}}
# }
```

What's brilliant about this though is that when I say it "consumes the iterator", it doesn't try to process every item
in the iterator, it merely takes ownership of it. Rust iterators are "lazy" meaning that they only call `next` when its
necessary to get the next item in the list. This has huge performance benefits, and we'll talk more about this later in
the chapter.

---

[^fib-seq]: Ok, it depends on who you ask, and this can be a good clarifying questions. Some people start the sequence
`1, 1, ...`, some people start it `0, 1, ...`, although Fibonacci himself actually started it `1, 2, ...`. Making our
code work for the sequence `1, 2, ...` is trivial, just changing the starting numbers, but as a challenge, can you make
it work starting at `0, 1, ...` (one way to do it hidden below)

```rust
// Press ▶️ to see this code work, press 👁️ to reveal the method I used
# struct Fibonacci {
#     current: Option<u8>,
#     next: Option<u8>,
# }
# 
# impl Fibonacci {
#     fn new() -> Self {
#         Self {
#             current: Some(0),
#             next: Some(1),
#         }
#     }
# }
# 
# impl Iterator for Fibonacci {
#     type Item = u8;
# 
#     fn next(&mut self) -> Option<Self::Item> {
#         // Store "current" value (we're going to overwrite it)
#         let current = self.current?;
# 
#         self.current = self.next;
#         
#         if let Some(next) = self.next {
#             // Update internal values
#             self.next = current.checked_add(next);
#         }
# 
#         // Return the "current" value
#         Some(current)
#     }
# }
# 
# fn main() {
#     let mut fib = Fibonacci::new();
# 
#     assert_eq!(fib.next(), Some(0));
#     assert_eq!(fib.next(), Some(1));
#     assert_eq!(fib.next(), Some(1));
#     assert_eq!(fib.next(), Some(2));
#     assert_eq!(fib.next(), Some(3));
#     assert_eq!(fib.next(), Some(5));
#     assert_eq!(fib.next(), Some(8));
# 
#     // Make sure we definitely return the last number correctly!
#     assert_eq!(fib.last(), Some(233));
# 
#     for (n, f) in Fibonacci::new().enumerate() {
#         println!("{n}: {f}");
#     }
# }
```


[^fib-interview]: In my opinion... it's not a very good question to ask. It's supposed to show that you have an 
                  understanding of recursion, and can lead to follow up questions on things like memoization, but it's a
                  bad test of whether you are a good software engineer. You're unlikely to ever use the Fibonacci
                  sequence outside an interview. Even if you were asked to find the nth number, you'd simply look up
                  a formula (see one way to do it below). You wouldn't solve it in this way you're expected to do in
                  the interview.
```rust
// Press 👁️ to reveal a method of calculating the nth value of the sequence
// without having to use recursion of a loop!
# const GOLDEN_RATIO: f64 = 1.618033988749894848204586834365638118;
# const SQRT_FIVE: f64 = 2.23606797749979;
# 
# fn fib(n: i32) -> i32 {
#     ((GOLDEN_RATIO.powi(n) - (1.0 - GOLDEN_RATIO).powi(n)) / SQRT_FIVE).round() as i32
# }
# 
# fn main() {
#     assert_eq!(fib(6), 8);
#     assert_eq!(fib(19), 4181);
#
#     // We can even test it against our version
#     for (i, f) in Fibonacci::new().enumerate() {
#         // Note this fib(0) => 0 so we'll add 1 to match our sequence
#         // or you could use the version from the previous footnote
#         assert_eq!(f, fib(i as i32 + 1) as u8);
#     }
# }
#
# // ------- Our version below -------
#
# {{#rustdoc_include iterators/src/fibonacci.rs:0}}
```

Getting Iterators
-----------------

While I wanted to show you how to make your own Iterator to give you an understanding of how they work, most often
you'll get an Iterator from a collection. All the collections we discussed in [the last chapter](collections.md) can
give you an iterator.

As with most things in Rust, Iterators (or specifically, the items being iterated) can be thought of in three groups:

### 1. Owned data (`T`)

For each built in collection type, you can use `.into_iter()` to consume the collection (take ownership of it) and give
you an Iterator where the Item type is not referenced... any further than it already was.

To understand this, consider these two examples:

```rust
let hello = "Hello".to_string();
let world = "World".to_string();

let v1 = vec![hello.clone(), world.clone()];
let v2 = vec![&hello, &world];

let mut i1 = v1.into_iter(); // v1 does not exist past this point
let mut i2 = v2.into_iter(); // v2 does not exist past this point

// The values in i1 are of type String
assert_eq!(i1.next(), Some("Hello".to_string()));
assert_eq!(i1.next(), Some("World".to_string()));
assert_eq!(i1.next(), None);

// The values in i2 are of type &String
assert_eq!(i2.next(), Some(&"Hello".to_string()));
assert_eq!(i2.next(), Some(&"World".to_string()));
assert_eq!(i2.next(), None);
```

### 2. Referenced data (`&T`)

### 3. Mutably referenced (`&mut T`)

