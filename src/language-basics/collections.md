Collections
===========

The Rust standard library gives you several ways to store a collection of items that are all the same type. Some of them
you'll use quite a lot, like `Vec` and `HashMap`, but there's a few more that have interesting features that can be
useful in specific circumstances.

This chapter will focus on the collections primarily on the ones you'll use the most, but we'll also cover all the other
collections provided by the Rust library too. We'll also mostly focus on structure and what you might choose them for
but most of how we use them will come in the next chapter.

Arrays
------

Arrays are... technically not collections. Or at least, they are quite different to the other collections in one key
way.

They're `Sized`!

While other collections can have elements added to or removed from them, Arrays are always the same size. This means
that they can exist on the stack, which further means they can be `Copy` (so long as the type stored in the array is
also `Copy`).

We can create arrays in two ways, either by defining a list of expressions, or by using an expression and a length:

```rust
let a1 = [1, 2, 3, 4, 5]; // A list of values
let a2 = [1 - 0, 1 + 1, 9 / 3]; // A list of expressions
let a3 = [1 + 2; 5]; // Fill an array with the result of a single expression, evaluated once

assert_eq!(a1, [1, 2, 3, 4, 5]);
assert_eq!(a2, [1, 2, 3]);
assert_eq!(a3, [3, 3, 3, 3, 3]);
```

You can access arrays using square brackets and the index of the entry you want to edit, however, if

You can pass arrays into functions so long as you know the exact length of the array. If the contents of the array is
`Copy` then the array will also be `Copy` and will be passed with Copy Semantics instead of Move Semantics

```rust
fn demonstrate_some_mutation(mut arr: [usize; 5]) {
    // Because the array is passed using copy semantics, it doesn't matter if the original variable was mutable.
    // Also, because we know the array _must_ have five elements, it's safe to directly access elements we know exist
    arr[0] = 100;
    assert_eq!(arr, [100, 1, 1, 1, 1]);
}

# fn main() {
// Reminder: Simply passing a1 to a function that takes an array of `usize`s is enough to infer the type 
let a1 = [1; 5];
assert_eq!(a1, [1, 1, 1, 1, 1]);

// Because the contents of the array is `Copy`, the array is `Copy`.
// This means it will use copy semantics when passed to the function
demonstrate_some_mutation(a1);

// So we can still use the original variable, _and_ it hasn't been mutated.
assert_eq!(a1, [1, 1, 1, 1, 1]);
# }
```

Note: This won't compile because the a2 is the wrong size
```rust,compile_fail
fn demonstrate_some_mutation(mut arr: [usize; 5]) {
    // ...
#     arr[0] = 100;
#     assert_eq!(arr, [100, 1, 1, 1, 1]);
}
# fn main() {
let a2 = [1; 4];
demonstrate_some_mutation(a1);
# }
```

Slices
------

Obviously, passing exactly sized arrays around isn't particularly useful in most real world circumstances. So the first
dynamically sized collection we should talk about is the "slice". 

You can think of a slice as a view or window into a series of contiguous data `[T]`. The fact that it's a view of some 
other type hopefully indicates to you that this is a reference type, i.e. `&[T]` or `&mut [T]` for mutable slices.

The simplest way to get a slice is to reference an array. 

```rust
# fn main() {
let arr: [u32; 5] = [0, 1, 2, 3, 4];
let slice: &[u32] = &arr;

assert_eq!(slice, &[0, 1, 2, 3, 4]);
# }
```

You can also use range notation, [which we've discussed before](./control-flow.md#range), including open ranges, which
we didn't discuss at the time. The way to think about this is, where `X..Y`:

- if X is specified, the slice begins before the `X`th element
- if X is not specified, the slice begins at the start of the collection being sliced
- if Y is specified, the slice ends before the `Y`th element
- if Y is preceded by an equals, the slice ends after the `Y`th element
- if Y is not specified, the slice ends at the end of the collection being sliced

```rust
let array: [u32; 5] = [0, 1, 2, 3, 4];

assert_eq!(&array[1..3],  &[1, 2]);       // X is specified, slice begins before the `X`
assert_eq!(&array[..3],   &[0, 1, 2]);    // no X, slice begins at the start
assert_eq!(&array[1..3],  &[1, 2]);       // Y is specified, slice ends before the `Y`
assert_eq!(&array[1..=3], &[1, 2, 3]);    // =Y , slice ends after the `Y`
assert_eq!(&array[1..],   &[1, 2, 3, 4]); // no Y, slice ends at the end
```

When using slices, you don't need ot specify their size, that information is encoded into the data at runtime, meaning
you can work with slices of arbitrary size. Bear in mind though that they are references, so you _may_ need to use
lifetimes to keep track. We discussed [lifetimes in the functions chapter](./functions.md#lifetimes), however as a quick
reminder, if the reference is a kite, lifetimes are the string that tie it back to the owning data. 

In the functions chapter we discussed a function for splitting a `String` which returned two `&str` or "string slice
references". That's right, `&str` is another, special, kind of slice. Here's that code again, and here's some things to
note that will hopefully make a lot more sense after the last few chapters:
- `String` implements `Deref` targeting `str` so we can get a string slice just by referencing `String`
- The lifetime `'a` is attached to `yuki` enters `split` through `input` and is tied to the return parameters `left` and
  `right`.
- The same range notation is used to create the slices as above
- In the "`found_at`" branch, open ranges are used to capture the beginning and end for `left` and `right` respectively
- In the "`else`" branch, the completely open range creates a slice the full length of the collection, while the slice 
  that starts at `input.len()` is a zero length slice that starts before the element that _would_ be after the final 
  element (i.e. it starts after the final element) and runs to the end (i.e. the same place as it starts).

```rust
fn split<'a>(input: &'a str, sub_string: &str) -> (&'a str, &'a str) {
    if let Some(found_at) = input.find(sub_string) {
      (&input[..found_at], &input[found_at + sub_string.len()..])
    } else {
      (&input[..], &input[input.len()..])
    }
}

# fn main() {
let yuki = "yuki".to_string();
let sub_string = "uk".to_string();

let (left, right) = split(&yuki, &sub_string);

assert_eq!(yuki, "yuki");
assert_eq!(left, "y");
assert_eq!(right, "i");
# 
# // Here's some test code to make sure I'm telling you the right think
# let yuki = "yuki".to_string();
# let sub_string = "daniel".to_string();
# 
# let (left, right) = split(&yuki, &sub_string);
# 
# assert_eq!(yuki, "yuki");
# assert_eq!(left, "yuki");
# assert_eq!(right, "");
# }
```

Hopefully code like this is starting to make a lot more sense!

Vectors
-------

`Vec` (short for Vector) is similar to an array (and can be dereferenced as an array slice), but unlike array, Vec can
grow in size. `Vec` is a generic type (`Vec<T>`) with no trait bound, meaning you use them with any type.

> So here's something weird. `Vec<T>` has no trait bound at all... not even `?Sized`, but you can have a Vec of unsized
> items, eg, 

There are several ways to instantiate `Vec`s, and which way is best can vary depending on how you're going to use them.

The fact that `Vec`s are dynamically sized means they need to exist on the Heap, and so, your program, under the hood,
will request an amount of heap memory. If your vector exceeds the amount of memory that is currently available to it,
code inside the `Vec` type will automatically request a new, larger, portion of memory from the operating system, copy
the current data into that new location, then free the memory it used to hold, all automatically without you needing to
do anything. This process however is expensive, and you should do what you can to avoid it. 

With that in mind, you should try to start with a vector large enough to contain as much as you think is reasonable, 
using the `::with_capacity(capacity: usize)` constructor. This will construct an empty `Vec` with at least (but possibly
not exactly) the capacity you requested. Note that capacity and length are not the same thing in Rust. You can get the
number of items of data currently in the vector with `.len()` and its capacity with `.capacity()`.

```rust
let example: Vec<i32> = Vec::with_capacity(10);
assert_eq!(example.len(), 0);
assert!(example.capacity() >= 10);
```

If you're not worried about the potential costs of resizing your vector, and you already have some data that you want to
instantiate, you can use the `vec!` macro.

```rust
let example = vec![0, 1, 2, 3];
assert_eq!(example.len(), 4);
```

Usually you'll make Vectors mutable, and they provide a huge array (pun intended) of useful methods, but here are some
of the basics.

To add elements to the end of a vector we use the `.push(t: T)` method, and to remove them from the end of the vector
we use the `.pop()` method which returns an `Option<T>`, since the vector may be empty.

```rust
let mut v = Vec::with_capacity(2);
v.push("Hello");
v.push("World!");

// v as it is now
assert_eq!(v, vec!["Hello", "World!"]);

// popping returns an option containing the last element of the vector (if there are no items the Option will be None)
assert_eq!(v.pop(), Some("World!"));

// popping an item from the vector modifies the vector so it no longer contains the last item
assert_eq!(v, vec!["Hello"]);
```

If you're used to arrays and vectors in other languages, you _can_ index directly into an array in Rust in the same way
that you can in other languages _but_ you generally shouldn't. If you try to access an element out of bounds (eg, if you
have 3 items in your vector, and try to access the fifth), your program will panic.

```rust,should_panic
let v = vec!["Hello", "World!"];
assert_eq!(v[0], "Hello");
assert_eq!(v[1], "World!");
let _ = v[2]; // Panics!
```

Instead, Vec provides `.get()` and `.get_mut()` which allow return an `Option` containing either an immutable or mutable
reference to an item inside the vector. This is much safer as the program will not halt if there is no item at the given
index, you'll simply get a `None`.

```rust
let v = vec!["Hello", "World!"];
assert_eq!(v.get(0), Some(&"Hello"));
assert_eq!(v.get(1), Some(&"World!"));
assert_eq!(v.get(2), None);
```

`.get_mut()` will return a mutable reference to the element inside the Vec, _but_ the way we use it... is a little
weird:
```rust
let mut v = vec!["Hello".to_string()];
if let Some(hello) = v.get_mut(0) {
    assert_eq!(hello, &mut "Hello".to_string());
    hello.push_str(", World!");
    assert_eq!(v, vec!["Hello, World!".to_string()]);
}
```

`.get()` and `.get_mut()` will also allow you to create an array slice if you give it a `Range` instead.

```rust
let mut v = vec![0, 1, 2, 3, 4, 5];

// Note the weird syntax as `get` returns an array slice, not an array
assert_eq!(v.get(2..), Some(&[2, 3, 4, 5][..])); 
assert_eq!(v.get(..2), Some(&[0, 1][..]));
assert_eq!(v.get(6..), Some(&[][..]));

// You can even edit values inside the returned slice
if let Some(mut inner) = v.get_mut(2..) {
    inner[0] += 10; // Actually element 2!
};

assert_eq!(v, vec![0, 1, 12, 3, 4, 5]);
```

Finally, we can also create slice using square brackets and this actually works without panicing:

```rust
let v = vec![0, 1, 2, 3, 4, 5];

// This would panic:
// assert_eq!(v[6], 0);
// But this doesn't:
assert_eq!(v[6..], []); 
```

#### A note on ownership

When you put a variable into a `Vec`, or any other collection, unless that variable is copy you are moving ownership
into the collection. Using methods like `get` will give you a reference to the data, but the only way to get ownership
back is to either clone in (and take the potential memory and runtime hit), or to remove to use a method that removes
the element from the collection, like `pop` in `Vec`. We'll discuss similar methods for other collections as we go.

### VecDequeue

`VecDeque` is very similar to `Vec` however, where in `Vec` you can only add and remove items from the end, `VecDeque`
also allows you to add and remove items from the front!

```rust
use std::collections::VecDeque;

let mut v = VecDeque::from([0, 1, 2, 3, 4, 5]);

v.push_back(6);
v.push_front(-1);

assert_eq!(v, [-1, 0, 1, 2, 3, 4, 5, 6]);
assert_eq!(v.pop_front(), Some(-1));
assert_eq!(v.pop_front(), Some(0));
assert_eq!(v.pop_back(), Some(6));
assert_eq!(v, [1, 2, 3, 4, 5]);

```

### Linked Lists

It's very rare to actually need a full `LinkedList`, and for performance reasons, you should try to avoid them where 
possible. `Vec` and `VecDeque` will almost always beat `LinkedList` in both speed and memory efficiency if all you want
to do is add items to the end of a list (or, in the case of `VecDeque` to the front).

Where `LinkedList`s are useful though, is when splitting and merging your collection is a core feature you will be
heavily reliant on.

```rust
use std::collections::LinkedList;

let mut list = LinkedList::new();
list.push_back(1);
list.push_back(3); // ohps! forgot 2!
list.push_back(4);
list.push_back(5);

// This gets us a vec use for comparison.
// Don't worry about this syntax yet, we'll explain it in the next chapter!
assert_eq!(list.iter().copied().collect::<Vec<_>>(), &[1, 3, 4, 5]);

// We can inject the missing number like this
let mut right = list.split_off(1);
list.push_back(2);
list.append(&mut right);

assert_eq!(list.iter().copied().collect::<Vec<_>>(), &[1, 2, 3, 4, 5]);
# 
# // Weirdly, the append method doesn't consume the other linked list but it does empty it
# // This might be useful if you are juggling values linked lists that you want to keep ownership of 
# assert_eq!(right.iter().copied().collect::<Vec<_>>(), &[]);
```

HashMap
-------

A HashMap is, simply, a key value lookup table. The key can be a value of any type, so long as that type implements the
`Hash` trait (see the [previous chapter](./common-traits.md#hash)).  Hashing the key results in a u64 that is used to
create the lookup table. There's more details on how hashing works in the official book, including how to create a
`HashMap` with a different hashing algorithm, but that's beyond the scope of IRISS.
ToDo: Add link to the book

Similar to `Vec`s, `HashMap`s can be initialised in a few ways, the main three you're likely to use are:

```rust
use std::collections::HashMap;

// Create an empty hashmap with some arbitrary capacity
let mut hashmap: HashMap<String, String> = HashMap::new();

// Create a hashmap with _at least_ this capacity (prevents reallocation if you know 
// the largest your hashmap will ever be)
let mut hashmap_with_capcity = HashMap::with_capacity(1);

// You usually won't have to specifically type the HashMap so long as Rust can infer
// the types by what you're inserting into it.
hashmap_with_capcity.insert(
    "Key".to_string(), // Can be anything with Hash but all keys for a hashmap need to be the same type
    "Value".to_string(), // Can be anything at all
);

// Create a hashmap with initial values from an array of tuples (K, V) where K: Hash
let mut hashmap_from_array = HashMap::from([
    ("String is Hash".to_string(), "First value".to_string()),
    ("Another Key".to_string(), "Another value".to_string()),
]);
```

To access data you've stored in your hashmap, there's a few handy methods:
- `.get(key: &K)` and `.get_mut(key: &K)` will get references 

### BTreeMap

### BinaryHeap

Sets
----

### HashSet

### BTreeSet


- vectors
  - talk about Vec, mention VecDeque, quick mention Linked Lists
- hashmap
  - talk about hashmap, mention btreemap
- sets
  - quick mention that hashmap and btreemap have variants for sets 
- heaps
  - 