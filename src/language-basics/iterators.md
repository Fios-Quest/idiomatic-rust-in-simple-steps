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

Facts you should know about Iterators
-------------------------------------

Iterators are designed to be chained together.

Iterators in Rust are "lazy". That means that each item is only processed as it's needed.  

Getting Iterators
-----------------

While I wanted to show you how to make your own Iterator to give you an understanding of how they work, most often
you'll get an Iterator from a collection.

As with most things in Rust, Iterators (or specifically, the items being iterated) can be thought of in three groups, 
and all built in collection types support all 3 of these and all the collections we discussed in
[the last chapter](collections.md) can give you an iterator in any of the following forms.

### 1. Referenced data (`&T`)

Often we don't actually need to _own_ the data we're iterating over, it can be enough to just read it. All built in
collections have a method called `.iter()` which returns an Iterator type.

> The specific struct returned varies per collection because, while they all implement `Iterator`, some of them 
> implement traits not possible for others, such as `DoubleEndedIterator` which we'll discuss later.

This Iterator will produce references that point to data in the original collection. 

```rust
let hello = String::from("Hello");
let world = String::from("World");

// Ownership moves into v
let v = vec![hello, world];

let mut iter = v.iter();

// The iterator contains references to the original data
assert_eq!(iter.next(), Some(&String::from("Hello")));
assert_eq!(iter.next(), Some(&String::from("World")));
assert_eq!(iter.next(), None);

// The vector still exists and contains the original data
assert_eq!(
    v, 
    vec![
        String::from("Hello"),
        String::from("World"),
    ]
);
```

One thing to bear in mind is that if the collection contains references, then `.iter()` will give you an Iterator that
produces reference to references.

```rust
let hello = String::from("Hello");
let world = String::from("World");

// This time we'll reference the original data
let v = vec![&hello, &world];

let mut iter = v.iter();

// The iterator contains references to references to the original data
assert_eq!(iter.next(), Some(&&hello));
assert_eq!(iter.next(), Some(&&world));
assert_eq!(iter.next(), None);

// The vector still exists and contains the original references
assert_eq!(v, vec![&hello, &world]);
```

### 2. Mutably referenced (`&mut T`)

Sometimes (though I've found, less than I might have expected), you need to edit things while iterating through them.
There's actually a couple of ways to take an item from an iterator and get new data from it, but in the event that you
want to edit data in place, the right way to do this is with an iterator of mutable references.

All Rusts built in collections can give you a mutable iterator (if the underlying collection is mutable) using
`.iter_mut()`.

```rust 
// The source data must be mutable, not just the iterator
let mut v = vec![1, 2, 3, 4, 5];

for n in v.iter_mut() {
    // Dereferencing to edit the underlying data
    *n += 10
}

// The original Vec, v, has had 10 added to each number
assert_eq!(v, vec![11, 12, 13, 14, 15]);
```

### 3. Owned data (`T`)

Finally, you may want to take ownership of the underlying data. This can be very useful in particular if you want to
turn one collection type into another collection type, (though there are other reasons you may want ownership of the
underlying data without duplicating it).

There is a trait called `FromIterator<A>` that is implemented for types that can consume an iterator and populate
themselves. This is almost always used with the collect iterator, though you need to be explicit about what you're
collecting into, either by typing the variable you're collecting into, or by using the turbofish operator that allows
you to be explicit about the concrete types to fill in generics.

```rust
use std::collections::LinkedList;

let hello = String::from("Hello");
let world = String::from("World");

// String ownership moves into v
let mut ll = LinkedList::new();
// Adding the words backwards because we can in a LinkedList
ll.push_front(world);
ll.push_front(hello);


// The inner type can be elided as Rust can work out that its String 
let v: Vec<_> = ll
    .into_iter() // This takes ownership of the contents of the linked list 
    .collect(); // This collects the data into the vector which now owns the inner data

assert_eq!(
    v,
    vec![
        String::from("Hello"),
        String::from("World"),
    ]
);
```
Sometimes you may not want to have an additional variable just to provide information, that's when the turbofish
operator comes in handy.

```rust
# use std::collections::LinkedList;
#
# let hello = String::from("Hello");
# let world = String::from("World");
# 
# // String ownership moves into v
# let mut ll = LinkedList::new();
# // Adding the words backwards because we can in a LinkedList
# ll.push_front(world);
# ll.push_front(hello);
# 
assert_eq!(
    ll.into_iter().collect::<Vec<_>>(),
    vec![
        String::from("Hello"),
        String::from("World"),
    ]
);
```

Copying and cloning Items
-------------------------

Using what we've learned above, what if we want to use owned data, but we need to keep the original collection, so 
`.into_iter()` is out of the question?

There are two methods on `Iterator` for dealing with the `.copied()` and `.cloned()`.

`.copied()` only works on Iterators where the item is `Copy` will consume the iterator and return a new iterator which
returns each Item copied. 

```rust
let v = vec![1, 2, 3, 4, 5];

let iter: Vec<_> = v.iter().collect();
let copied: Vec<_> = v.iter().copied().collect();

assert_eq!(iter, vec![&1, &2, &3, &4, &5]);
assert_eq!(copied, vec![1, 2, 3, 4, 5]);
```

`.cloned()` does the same for types that are `Clone`. 

```rust
let v = vec![String::from("Hello"), String::from("World")];

let iter: Vec<_> = v.iter().collect();
let cloned: Vec<_> = v.iter().cloned().collect();

assert_eq!(iter, vec![&"Hello", &"World"]);
assert_eq!(cloned, vec![String::from("Hello"), String::from("World")]);
```

Other Ways to get Iterators
---------------------------

Beyond collections there are other things that can be iterated through.

Ranges are iterators, it's why you often see them used in for loops:

```rust
for n in 0..5 {
    println!("Number: {n}");
}

assert_eq!((0..5).collect::<Vec<_>>(), vec![0, 1, 2, 3, 4]);
```

As previously highlighted, you can create an infinitely repeating iterator with the `repeat(T)` function. You can 
repeat any value of any type so long as that type implements the `Clone` trait.

```rust
use std::iter::repeat;

let mut repeater = repeat("Badger".to_string());

assert_eq!(repeater.next(), Some("Badger".to_string()));
assert_eq!(repeater.next(), Some("Badger".to_string()));
assert_eq!(repeater.next(), Some("Badger".to_string()));
assert_eq!(repeater.next(), Some("Badger".to_string()));
# // Mushroom Mushroom (no I don't know "what's wrong with me", why?)
```

This can have benefits over using something like an open range which will eventually overflow and panic

```rust,should_panic
let iter = 0u8..;
assert_eq!(iter.count(), 255); // ‼️ Panics because the iterator does not end at u8::MAX ‼️
```

Fun fact, if you want to take an existing finite Iterator and repeat that infinitely, there's a method for that too, 
though the Item type must implement `Clone`.

```rust
let mut iter = vec![0, 1, 2].into_iter().cycle();

assert_eq!(iter.next(), Some(0));
assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(0));
assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(0));
// ...and so on
```

Though don't forget, immutable references are `Clone`.

```rust
#[derive(Debug, PartialEq)]
struct NotCloneable;

// Value will own our data
let value = NotCloneable {};

// The vec we base the iterator on will contain a reference
let mut iter = vec![&value].into_iter().cycle();

assert_eq!(iter.next(), Some(&NotCloneable {}));
assert_eq!(iter.next(), Some(&NotCloneable {}));
assert_eq!(iter.next(), Some(&NotCloneable {}));
```

You can also create iterators by combining other iterators, but they must be of the same type:

```rust
let mut i1 = vec![0, 1, 2].into_iter();
let i2 = vec![3, 4, 5].into_iter(); // i2 does not need to be mutable as we're taking ownership

assert_eq!(i1.chain(i2).collect::<Vec<_>>(), vec![0, 1, 2, 3, 4, 5]);
```

Many other Types in Rust can also be broken down into Iterators. This Chapter of the book can be represented as one
large `String`, which Dereferences to `str` which allows you to break the data down by `.lines()`, `.chars()` or 
`.bytes()`.

> ℹ️ Don't forget a `char` is not the same as a byte (`u8`) in Rust, and in this Chapter I've used several multibyte 
> characters 😉

Cool ways to use Iterators
--------------------------

### Mathematics

A common use case for iterators over things like numbers is adding everything together, or multiplying things together.

For iterators of items that implement the `Sum` trait (eg, numbers) `.sum()` will add all the items in the iterator:

```rust
assert_eq!(
    vec![1, 2, 3, 4].iter().sum::<i32>(), 
    10
);
```

For iterators of items that implement the `Product` trait (eg, again, numbers) `.product()` will multiply all the items
in the iterator, eg:

```rust
assert_eq!(
    vec![1, 2, 3, 4].iter().product::<i32>(), 
    24
);
```

Its worth noting that some surprising things implement `Sum` and `Product`, including blanket implementations for 
`Option<T>` and `Result<T, E>` where `T` already implements the trait.

```rust
let v: Vec<Option<usize>> = vec![
    Some(10),
    Some(20),
    Some(12),
];

// Note: the Option needs to be owned, references don't work, so we'll use .into_iter()
let total: Option<usize> = v.into_iter().sum(); 
assert_eq!(total, Some(42));
```

For iterators of items that implement `Ord` you can use `.min()` and `.max()` to find the largest and smallest values
respectively:

```rust
let v = vec!['H', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'];

assert_eq!(v.iter().min(), Some(&'H'));
assert_eq!(v.iter().max(), Some(&'w'));
```

We could also find out how many items are in the iterator using `.count()` but its worth noting this does consume the
iterator, and that _most_ collections allow you to get their size directly from

```rust
let v = vec!['H', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'];

assert_eq!(v.iter().count(), v.len());
```

#### ⚠️ Warning!

Methods like `sum` and `product` do a simple `+` or `*` respectively, which means that if the result overflows, the 
_best_ thing that can happen is your program panics. For more robust (but slower) code you may want to implement the
operation yourself using `fold` which is an iterator method we'll talk about later.

Furthermore, methods like `sum`, `product`, `min`, `max` and many more, expect an iterator to have an end to give you a
final result, but it is possible to create infinite iterators. For example, the function `repeat("hi")` will just 
continue to produce a reference to the string slice `"hi"` forever.

```rust
use std::iter::repeat;

let mut banana_phone = repeat("ring");

assert_eq!(banana_phone.next(), Some("ring"));
assert_eq!(banana_phone.next(), Some("ring"));
assert_eq!(banana_phone.next(), Some("ring"));

// It never ends
assert_eq!(banana_phone.next(), Some("ring"));

// Calling something like `.max()` on this iterator will cause an infinite loop
# // Hahahaha, now its in your head 😈
```

### Applying a Process over each item

One of the most common uses for Iterators is process a set of Items one at a time. There are a number of methods on the
Iterator trait (that themselves return new Iterators) that are really helpful for this.

You can take one iterator and exclude Items based on the result of a predicate using the `.filter(P)`. For example, we
could take a range of numbers, and filter out all odd numbers like this:

```rust
// Many iterator methods return a new iterator which is great for chaining
let mut iter = (1..=10).filter(|n| n % 2 == 0);

assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(4));
assert_eq!(iter.next(), Some(6));
assert_eq!(iter.next(), Some(8));
assert_eq!(iter.next(), Some(10));
assert_eq!(iter.next(), None);
```

Another great way to process Iterators one Item at a time is to take that Item and transform it in some way. We can use
pass a function into the `.map()` method that receives the item and returns a new value. If that value is of a different
type, the Iterator you get back will also be of that new type:

```rust
let mut iter = (1..=3) // An Iterator where Item is i32
    .map(|n| format!("This is item number {n}")); // New Iterator where Item is String

assert_eq!(iter.next(), Some("This is item number 1".to_string()));
assert_eq!(iter.next(), Some("This is item number 2".to_string()));
assert_eq!(iter.next(), Some("This is item number 3".to_string()));
assert_eq!(iter.next(), None);
```

Sometimes the process you apply to an item might itself result in an `Option`, and rather than having an iterator of
`Options` you may want to discard `None`s and unwrap the `Ok`, this is where `.filter_map()` is really handy.

In the example below we use `.checked_add` which returns an `Option` with an `Ok` so long as the result is inbounds. By
combining this with filter_map, we'll get only the items that were Some, and those items will be unwrapped. 

```rust
let mut iter = (1..=u8::MAX)
    .filter_map(|n| n.checked_add(250u8));

assert_eq!(iter.next(), Some(251));
assert_eq!(iter.next(), Some(252));
assert_eq!(iter.next(), Some(253));
assert_eq!(iter.next(), Some(254));
assert_eq!(iter.next(), Some(255));
assert_eq!(iter.next(), None);
```

This not only saves us from having to deal with doubly wrapped options from `next` (eg `Some(Some(255))`) but entirely 
removes the items from the iterator. See this example comparing map and filter map: 

```rust
assert_eq!((1..=u8::MAX).map(|n| n.checked_add(250u8)).count(), 255);
assert_eq!((1..=u8::MAX).filter_map(|n| n.checked_add(250u8)).count(), 5);
```

Another way to reduce how many items we want to deal with in an iterator is by using `.take(n)` and `.skip(n)`.

We can end an iterator earlier by only taking a certain number of items from it with `.take(n)` or we can skip over a 
number of items with `.skip(n)` before resuming the iterator from that point. 

```rust
let v = vec![1, 2, 3, 4, 5, 6];

let iter_take = v.iter().take(3);
let iter_skip = v.iter().skip(3);

assert_eq!(iter_take.collect::<Vec<_>>(), vec![&1, &2, &3]);
assert_eq!(iter_skip.collect::<Vec<_>>(), vec![&4, &5, &6]);
```

As with most iterators, you can chain them:

```rust
# let v = vec![1, 2, 3, 4, 5, 6];
# 
assert_eq!(v.iter().skip(1).take(4).collect::<Vec<_>>(), vec![&2, &3, &4, &5]);
```

An Iterator method we used earlier, `.enumerate()`, allows us to add an index to our Iterator by changing the type of
the iterator `T` to a tuple: `(usize, T)`. This can be really handy in combination with other iterators when the
position in an iterator is important. For example, lets say we want to filter every other item out of a `Vec`. We can
do that by chaining together several of the Iterators we've just learned.

```rust
let v1 = vec!["This", "sentence", "is", "not", "shorter"];

let v2: Vec<_> = v1.into_iter()
    .enumerate()
    .filter(|(i, _)| i % 2 == 0) // Use the index added by enumerate to skip odd items
    .map(|(_, s)| s) // Turn the iterator (usize, T) back into T
    .collect();

assert_eq!(v2, vec!["This", "is", "shorter"]);
```

Although, any time you see a `filter` and a `map` next to each other, you might be able to abbreviate this. Booleans can
be turned into `Option`s with `.then_some()`:

```rust
let v1 = vec!["This", "sentence", "is", "not", "shorter"];

let v2: Vec<_> = v1.into_iter()
    .enumerate()
    .filter_map(|(i, s)| (i % 2 == 0).then_some(s))
    .collect();

assert_eq!(v2, vec!["This", "is", "shorter"]);
```

More Iterator Traits
--------------------

There's a few more traits you may want to be aware of when making your own iterators:

`IntoIterator` can be implemented on any type that can be turned into an `Iterator`. One place you may find yourself
using it is on newtypes.

```rust
{{#include iterators/src/bin/albums.rs:Albums}}

{{#include iterators/src/bin/albums.rs:IntoIterator}}

fn main() {
{{#include iterators/src/bin/albums.rs:UseIntoIterator}}
}
```

`FromIterator` allows you to turn an Iterator into another type, usually through the `.collect()` method on `Iterator`s

```rust
{{#include iterators/src/bin/albums.rs:Albums}}

{{#include iterators/src/bin/albums.rs:FromIterator}}

fn main() {
{{#include iterators/src/bin/albums.rs:UseFromIterator}}
}
```

Finally, the last two traits you should be aware of are `DoubleEndedIterator` and `ExactSizeIterator`. The Iterators 
returned from collections are all both of these (to my surprise, even the `Iter` structs used for `LinkedList` and 
`BinaryHeap` are `DoubleEndedIterator`).

`ExactSizeIterator` can tell you the size of the iterator _without_ consuming it, using the `.len()` method (if you need
to see if the iterator is empty, you should us `.is_empty()` instead).

```rust
let v = vec![1, 2, 3, 4, 5];

let iter = v.into_iter();

assert_eq!(iter.len(), 5); // iter still exists after this
assert_eq!(iter.count(), 5); // iter is consumed
```

`DoubleEndedIterator` allows you to reverse the order of an Iterator with `.rev()`.

```rust
let v = vec![1, 2, 3];

let mut iter = v.into_iter().rev();

assert_eq!(iter.next(), Some(3));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), None);
```

Next Chapter
------------

We've now covered all of what I'd describe as the core, synchronous language features (at least... I hope). We're going
move on to Threads in the next chapter, discuss what they are and some of the most important and useful tools to use
when working with them.

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
