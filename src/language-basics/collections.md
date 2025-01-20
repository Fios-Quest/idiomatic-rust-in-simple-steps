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
let a1 = [1; 5];
assert_eq!(a1, [1, 1, 1, 1, 1]);

// Because the contents of the array is `Copy`, the array is `Copy`.
// This means it will use copy semantics when passed to the function
demonstrate_some_mutation(a1);

// So we can still use the original variable, _and_ it hasn't been mutated.
assert_eq!(a1, [1, 1, 1, 1, 1]);
}
```

Slices
------

Obviously, passing exactly sized arrays around isn't particularly useful in most real world circumstances. So the first
dynamically sized collection we should talk about is the "slice". 

You can think of a slice as a view or window into a series of contiguous data `[T]`. The fact that it's a view of some 
other type hopefully indicates to you that this is a reference type, i.e. `&[T]` or `&mut [T]` for mutable slices.

The simplest way to get a slice is to reference an array. 

```rust
let arr: [u32; 5] = [1, 2, 3, 4, 5];
let slice: &[u32] = &arr;
```

When using slices, you don't need ot specify their size, that information is encoded into the data at runtime, meaning
you can work with slices of arbitrary size.

Vectors
-------


### VecDequeue

### Linked Lists

HashMap
-------

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