Common Traits
=============

In the previous chapter we introduced the concept of traits and made our own Animal trait to fix some problems left over
from the chapter before that!

Rust provides a huge number of traits that you can implement on your own types, and in this chapter we're going to 
discuss what I think are the most important to be aware of, whether that's because you'll want to implement them 
yourself, you'll want to consume types that implement them, or they have interesting knock on effects you should be 
aware of.

This chapter is broken into three main parts:
- Markers - these can be considered intrinsic traits
- Derivables - traits who's functionality is so easy to 
- Converters - traits that allow you to change one type to another
- Iterators - traits that allow you to step through lists of things
- Other - Things that didn't fit nicely into the other categories

Markers
-------

### Sized

Anything that is of known size at compile time is consider to be `Sized`. For example, a `u8` has size 8 bits*, 
therefore it is sized. All primitives are sized, with the exception of `str`, which you can't use outside of its 
reference form anyway.

> ℹ️ Actually, u8 has a minimum size of 8 bits, however, what its size at compilation may not be 8 bits. This 
> _generally_ shouldn't impact your decisions about the types you use, but if you're doing embedded programming you
> _may_ need to check... but probably not. Fun fact, boolean is, as far as Rust cares, a 1 bit data type, however LLVM
> doesn't have a concept of a 1 bit data type, so uses i8 for Rust code instead.

One place you will see `Sized` a lot is that due to a quirk in Rusts design, generic types are assumed to be `Sized`.
For this reason you will regularly see the trait bound `?Sized` which means that it may or may not be `Sized`. Unlike
any other trait bound, this has a widening effect on the number of types that can be used within the generic.

For example, in the last chapter, I mentioned that I was printing a simplified version of Display. This was because I
left out the `?Sized` trait bound, so Display actually looks more like this:

```ignore
impl<T: Display + ?Sized> ToString for T {
    // ...
}
```

The `+` means the type `T` must abide both trait bounds, which includes `Display` but also may or may not be `Sized`.
Without the `?Sized` trait bound, `T` would be assumed to be `Sized`.

### Copy

The `Copy` marker trait means that the data the type contains can be copied, however, "copy" has a very specific meaning
in Rust which means that all the data can be exactly copied as is. Again, for all primitives (except `str`), as well as
all types built only from these types and exist on the stack, this is achievable, however, for something like `String`,
the data can not be safely copied. `String` is a smart pointer that points to a location on the Heap. If we copied the
smart pointer, then we'd have two pointers pointing to the same piece of memory, and if either of them went out of scope
that would cause them to clean up the memory on the heap that the other pointer still points to.

If you wanted to duplicate some string data, or something else inside a smart pointer, then you need to "clone" the data
instead, which we'll discuss below.

Copy is derivable, which we'll discuss below too.

### Send / Sync

We haven't talked about concurrent programming yet, however, you might have heard that Rust is extremely safe and
efficient compared to many other languages that allow you to kick off potentially multiple processes that might all be
trying to read and write to data at the same time. 

Much of that safety comes from these two marker traits, `Send` and `Sync`.

`Send` is used when data can be safely "sent" between threads. This might be achieved through the use of a channel or
similar. Again, we'll talk about this more in the future, so don't worry what this means just yet, however, when
something is "sent" from one thread to another, it moves ownership, like when you pass a variable to another function.

`Sync` is used when a reference to data can be safely sent from one thread to another, i.e. `T` is `Sync` is `&T` is
`Send`. This is perhaps easiest to explain with a type that isn't `Sync`, the `Rc<T>` generic. `Rc` is Rust's most basic
reference counting type. You give it some data to hold, and pass clones of the container around. This 



Derivables
----------

### Clone

### Default

### Eq / PartialEq

### Ord / PartialOrd

Converters
----------

### From / Into

### TryFrom / TryInto

### AsRef / AsMut

Iterators
---------

### Iterator

### IntoIterator

### FromIterator

Other
-----

### Drop

