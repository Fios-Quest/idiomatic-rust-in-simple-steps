Common Traits
=============

In the previous chapter we introduced the concept of traits and made our own `Animal` trait to fix some problems left
over from the chapter before that!

The Rust standard library itself provides a huge number of traits that you can implement on your own types, and in this
chapter we're going to discuss what I think are the most important to be aware of, whether that's because you'll want
to implement them yourself, you'll want to consume types that implement them, or they have interesting knock on effects
you should be aware of.

This chapter is broken into sections:
- **Markers** - these can be considered intrinsic traits
- **Derivables** - traits for which the functionality is so easy to implement there are easy tools to automate it 
- **Error Handling** - Traits related to handling
- **Converters** - traits that allow you to change one type into another
- **Referencing and Dereferencing** - traits that allow you to treat data as a different type without conversion
- **Other** - Things that didn't fit nicely into the other categories

Two notable absences to this chapter are `Iterator` and `IntoIterator`. They are both very common traits that you will
use pretty much all the time, _but_ there's so much to them, they deserve their own chapter.

Markers
-------

Markers are special traits that describe intrinsic properties of types, that is they relate to what you might call the
core essence of the type.

### Sized

We're starting with a weird one here. You never need to implement this yourself, but you may choose to manually opt out
of it.

So, anything that is of known size at compile time is consider to be `Sized`. For example, a `u8` has size 8 bits*, 
therefore it is sized. In fact, except for `str` (which you can't use outside its reference form anyway) all primitives
are sized. Any compound type you create from only `Sized` types is also considered to be `Sized`.  

> ℹ️ Actually, `u8` has a minimum size of 8 bits, however, its size at compilation may not be 8 bits. This _generally_
> shouldn't impact your decisions about the types you use, but if you're doing embedded programming you _may_ need to 
> check.
> 
> **Fun fact**, `bool` is, as far as Rust cares, a 1 bit data type, however LLVM doesn't have a concept of a 
> 1 bit data type, so uses `i8` for Rust code instead.

One place you will see `Sized` a lot is that due to a quirk in Rusts design, generic types are assumed to be `Sized`.
For this reason you will regularly see the trait bound `?Sized` which means that the concrete type (the one used to fill
in the generic) may or may not be `Sized`. Unlike any other trait bound, this has a widening effect on the number of
types that can be used within the generic.

For example, in the last chapter, I mentioned that I was printing a simplified version of `ToString` implementation for
all type that implement `Display`. This was because I left out the `?Sized` trait bound, so the `ToString` generic
implementation actually looks more like this:

```rust,ignore
impl<T: Display + ?Sized> ToString for T {
    // ...
}
```

The `+` means the type `T` must abide both trait bounds, which includes `Display` but also may or may not be `Sized`.
Without the `?Sized` trait bound, `T` would be assumed to be `Sized`.

You do not need to implement `Sized` for traits yourself, however some types may explicitly implement `!Sized` to 
prevent their type being treated as `Sized`. For now don't worry too much beyond when you need to add `?Sized` trait
bounds to your generics.

### Copy

The `Copy` marker trait means that the data the type contains can be copied, however, "copy" has a very specific meaning
in Rust which means that all the data can be exactly copied as is. Again, for all primitives (except `str`), as well as
all types built only from these types and exist on the stack, this is achievable, however, for something like `String`,
the data can not be safely copied. `String` is a smart pointer that points to a location on the Heap. If we copied the
smart pointer, then we'd have two pointers pointing to the same piece of memory, and if either of them went out of scope
that would cause them to clean up the memory on the heap that the other pointer still points to.

If you wanted to duplicate some string data, or something else inside a smart pointer, then you need to "clone" the data
instead, which we'll discuss below.

One awesome thing `Copy` does is it changes how the language itself works.

Because `Copy` can only apply to things on the Stack, and because copying into memory you already own is cheap, Rust
won't move ownership, and will use what are called "Copy Semantics" instead of "Move Semantics". This means, unlike
normal, when you reassign a variable, or pass it to a function, if the variable has the `Copy` trait, you can still
use the original variable after.

So ordinarily we can't do something like this, you'll get a compile time error. Try running this:

```rust,compile_fail
let x = "String is not Copy".to_string();
let y = x;
print!("y owns the str {y}"); 
print!("x no longer owns the str {x}");
```

However, for types that do implement `Copy` that does still work thanks to Copy Semantics:

```rust
let mut x = 42;
let y = x;
print!("x still owns the value {x}, and so does y {y}");
// As its a copy, we can change one without changing the other
x += 10;
print!("we've added 10 to x, which is now {x}, but y is still {y}");
```

You can implement `Copy` directly, though you must also implement `Clone` which we'll discuss later, and since both
traits are derivable, its very rare you'd ever do it manually.

### Send / Sync

We haven't talked about concurrent programming yet, however, you might have heard that Rust is extremely safe and
efficient compared to many other languages that allow you to kick off potentially multiple processes that might all be
trying to read and write to data at the same time. 

Much of that safety comes from these two marker traits, `Send` and `Sync`.

`Send` is used when data can be safely "sent" between threads. This might be achieved through the use of a channel or
similar. Again, we'll talk about this more in the future, so don't worry what this means just yet, however, when
something is "sent" from one thread to another, it moves ownership, like when you pass a variable to another function.

`Sync` is used when a _reference_ to data can be safely sent from one thread to another, i.e.:
- `T` is `Send` if `T` can be SENT safely from one thread to another
- `T` is `Sync` if `&T` can be safely used across multiple threads SYNCHRONOUSLY (at the same time)

This is perhaps easiest to explain with a type that isn't `Sync`, the `Rc<T>` generic. `Rc` is Rust's most basic
reference counting type. You give it some data to hold, and pass clones of the container around. The `Rc` owns the data
but keeps count of how many places using it there are. That count is not atomic and so two threads could attempt to
change the value at the same time. This means that it is not `Sync`. 

We'll talk a lot more about threaded programming later in the book so don't worry if this doesn't make sense yet, in 
fact, `Send` and `Sync`, like `Sized`, are auto-derived. This means you don't even have to worry about implementing them
for your own types so long as your types are entirely constructed from other types that are Send and/or Sync, the Rust
compiler knows that your type is Send and/or Sync too


Derivables
----------

Apart from `Sized`, `Send` and `Sync`, most traits _need_ to be manually opted in to, however, for some traits, the 
behaviour is so simplistic that the trait can be derived. For _most_ derivable Rust traits there is a requirement that 
each child of your type implements the trait you're attempting to implement yourself.

To derive a trait we use the derive attribute.

> Attributes can be defined either inside or outside the item they are for, however, like
> [Documentation](./documentation.md), unless the attribute is being in some way applied to the whole file (for example,
> as a module), we exclusively use external attributes that come before the item they apply to. Like Documentation we
> use an exclamation mark to differentiate the two
>
> ```rust,ignore
> #![internal_attribute]
> 
> #[external_attribute]
> fn item_with_external_attribute_applie() {}
> ```

The derive attribute itself, looks a bit like a function, but it takes a list of what _looks_ like traits but are 
actually what we call "Derive Macros":

```rust,ignore
#[derive(FirstTrait, SecondTrait, etc)]
struct StrucToGetDerivedBehaviours {
   // ...
}
```

Not every trait has a Derive Macro meaning not all traits are derivable. You can write your own Derive Macros too
(though this is a very advanced form of meta programming we probably won't cover in this book), and many people do to
provide custom derive macros for traits provided in their own libraries.

### Debug

`Debug` is an extremely useful utility Trait that creates a default way to write out types to things like stdout/stderr.

When printing a `Debug` value, we use `{:?}` for a positional marker, or you can put it after the name of a variable, 
eg `{name:?}`.

```rust
#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}

# fn main() {
// --- Example use ---
    
let cat = Cat {
    name: "Yuki".to_string(),
    age: 15
};

println!("{:?}", cat);
println!("or {cat:?}");
# }
```

Ironically perhaps, you should try to avoid using `Debug` for debugging, that's what a debugger is for, head back to our
[Hello World](../getting-started/hello-world.md) chapter if you need a reminder.

The `Debug` macro though is very useful for logging, though be careful to not leak private information this way, this
might be where you want to implement `Debug` manually.

What `Debug` is needed for, is things like the `assert_eq!` macro, mainly used in testing. If you `assert_eq!` two 
values, and they're not equivalent, the test suite will want to print the values to the screen. We'll show this more
when we talk about the equivalence traits in the next section.

`Debug` works very similarly to `Display` taking a formater as a parameter.

```rust,ignore
impl fmt::Debug for MyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ...
    }
}
```

You might be worried about making sure your implementation of the `Debug` trait behaves similarly to official/derived
implementations, well that's where the formatter gets _really_ cool, providing a ton of different tools that help you
build a well-structured output. We won't go into that here, but you can see more in the
[official `Debug` documentation](https://doc.rust-lang.org/std/fmt/trait.Debug.html).

### PartialEq / Eq

`Eq` and `PartialEq` are Rust's equivalency traits, that's right, not equality.

What's the difference, what does equivalence mean and why are there two traits?

Allow me to answer those questions with another question: Is `0` equivalent to `-0`. Mathematically, zero is neither 
positive nor negative, so these are identical, but inside a floating point number, the binary representation is 
different.

Speaking of floating points, in binary representation its possible to represent that something is Not a Number (NaN).
However, should two NaNs, even if they have the same binary representation, be considered as the same value when you
can get there in different ways? Probably not.

For the most part in Rust, we're only concerned with Partial Equivalence `PartialEq`, this is what allows us to compare
values with the `==` operator. Given what we've just discussed, consider the code below before you run it, what do you
think the output will be?

```rust
# fn main () {
if 0.0 == -0.0 {
    println!("0.0 IS equivalent to -0.0");
} else {
    println!("0.0 is NOT equivalent to -0.0");
}

if f64::NAN == f64::NAN {
    println!("NaN IS equivalent to NaN");
} else {
    println!("NaN is NOT equivalent to NaN");
}
# }
```

You can derive `PartialEq` so long as all the parts of your type also implement `PartialEq`, or you can implement it
yourself. Implementing it yourself can be really handy if you have a structure where some fields _can_ be different but
still be considered the same overall "thing". The official Rust book uses ISBNs as an example, though you might also
want this kind of behaviour for aliased user information or something similar.

`PartialEq` has two methods, `eq` and `ne`. `ne` has a default behaviour so you don't need to define it, but you can see
how in the previous example, being able to make the logic for `ne` different from simple `!x.eq(y)` could be handy.

Lets implement it ourselves below:

```rust
#[derive(Debug)]
struct User {
    id: u64,
    email: String,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
    
    // fn ne(&self, other: &Self) -> bool {
    //    // We'll leave the default logic of `ne`, to be "not eq"
    // }
}

# fn main() {
// --- Example use ---

let yuki = User { id: 1, email: "yuki@example.com".to_string() };
let alias = User { id: 1, email: "other@example.com".to_string() };
// Now that we know Debug and PartialEq we can use assertions!
assert_eq!(yuki, alias);
# }
```

`PartialEq` has even more utility though! It's a generic trait where the generic parameter represents the type for the
"right hand side" or RHS. This generic parameter defaults to being the same type, but we can write code that allows us
to compare the equivalence of different types too!

Taking that User alias example again, what if we had a "root" user type, and an aliased User type.

```rust
#[derive(Debug)]
struct User {
    id: u64,
    email: String,
}

#[derive(Debug)]
struct UserAlias {
    id: u64,
    root_id: u64,
    email: String,
}

impl PartialEq<UserAlias> for User {
    fn eq(&self, other: &UserAlias) -> bool {
        self.id == other.root_id
    }
}

# fn main() {
// --- Example use ---

let yuki = User { id: 1, email: "yuki@example.com".to_string() };
let alias = UserAlias { id: 2, root_id:1, email: "other@example.com".to_string() };
assert_eq!(yuki, alias);
# }
```

So that's `PartialEq`, but what is `Eq`? 

`Eq` doesn't actually provide any additional behaviour, it's an empty trait that can only be applied to types that are
also `PartialEq`. It's purpose _isn't_ to provide functionality but to indicate to you, the software engineer, and
anyone looking at your code, that types have exact equivalence. Those points we made about floating points earlier,
different binary representations having equality, and the same binary representation not being considered equal, are
not `Eq`, which is why `f32` and `f64` do not implement `Eq`.

There's no way for the compiler to guarantee the correct implementation of `Eq` so it's something you need to be mindful
of.

Unlike `PartialEq`, `Eq` is not a generic that can be used with other types (since we're talking about exact
equivalence, this wouldn't make sense).

Earlier we chose to make that `User` type partially equivalent if the id matched. If we instead checked the entire 
object, it could be considered to be exactly equivalent:

```rust
#[derive(Debug)]
struct User {
    id: u64,
    email: String,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.email == other.email
    }
}

impl Eq for User {}

# fn main() {
// --- Example use ---
    
let yuki = User { id: 1, email: "yuki@example.com".to_string() };
let alias = User { id: 2, email: "other@example.com".to_string() };
assert_ne!(yuki, alias);

let other_yuki = User { id: 1, email: "yuki@example.com".to_string() };
assert_eq!(yuki, other_yuki);
# }
```

Of course, in this case, it'd be far easier _and safer_ to use the derived version, which protects us making mistakes in
complex code, or forgetting to check changes we make in our type later:

```rust
#[derive(PartialEq, Eq)]
struct User {
    id: u64,
    email: String,
}
```

### PartialOrd / Ord

As you can imagine, `PartialOrd` and `Ord` have a similar relationship to each other as `PartialEq` and `Eq`, and
indeed:
- `PartialOrd` can only be applied to types with `PartialEq`
- `Ord` can only be applied to types with `Eq` (and `PartialOrd`)

Both `PartialOrd` and `Ord` have a required method each (`partial_cmp` and `cmp` respectively) as well as some methods
with default behaviour. The required methods of each trait use the `Ordering` type which looks roughly like this:

```rust
pub enum Ordering {
    Less = -1,
    Equal = 0,
    Greater = 1,
}
```

`PartialEq` is what gives us our usual greater than (`>`), less than (`<`), greater or equal to (`>=`)  and less than or
equal to (`<=`) behaviour, through the use of the methods `gt`, `lt`, `ge` and `le` respectively, though unless these
methods are implemented, their default behaviour relies on `partial_cmp`, which returns `Option<Ordering>`. 

Again, using floating point numbers, it's easy to see why we use an `Option` on our comparisons. When comparing `NaN`, 
is it greater than, less than, or equal to `NaN`? We can't determine that, so we use the `None` variant to represent
that.

But how does that affect ordering? What do you think the output of the following code will be:

```rust
# fn main() {
// Using the Debug output, see above!
println!("{:?}", f32::NAN.partial_cmp(&f32::NAN));

println!("NaN > NaN  = {:?}", f32::NAN > f32::NAN);
println!("NaN >= NaN  = {:?}", f32::NAN >= f32::NAN);
println!("NaN < NaN  = {:?}", f32::NAN < f32::NAN);
println!("NaN <= NaN  = {:?}", f32::NAN <= f32::NAN);
println!("NaN == NaN  = {:?}", f32::NAN == f32::NAN);
# }
```

One important thing to bear in mind when deriving `PartialOrd` is that although, yes you can do it if all parts of your
type implement `PartialOrd`, when derived on structs, it will first check the ordering of the first field, and only move
on to the next field if the first field is equal.

Eg:

```rust
#[derive(PartialEq, PartialOrd)] // Remember PartialEq is required for PartialOrd
struct Rect {
    width: u64,
    height: u64,
}

// --- Example comparisons ---

# fn main() {
let test_one_lhs = Rect { width: 2, height: 1 };
let test_one_rhs = Rect { width: 1, height: 1000 };
# assert!(test_one_lhs > test_one_rhs);
println!("test one: is lhs great than rhs? - {}", test_one_lhs > test_one_rhs);

let test_two_lhs = Rect { width: 2, height: 1 };
let test_two_rhs = Rect { width: 2, height: 1000 };
# assert!(test_two_lhs < test_two_rhs);
println!("test two: is lhs great than rhs? - {}", test_one_lhs > test_one_rhs);
# }
```

For this reason, it's quite likely that you'd want to implement `PartialOrd` yourself, depending on how you think types
should be compared.

```rust
#[derive(PartialEq)] // Remember PartialEq is required for PartialOrd
struct Rect {
    width: u64,
    height: u64,
}

impl Rect {
    pub fn area(&self) -> u64 {
        self.width * self.height 
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.area().partial_cmp(&rhs.area())
    }
}

// --- This is better, right? ---

# fn main() {
let test_one_lhs = Rect { width: 2, height: 1 };
let test_one_rhs = Rect { width: 1, height: 1000 };
# assert!(test_one_lhs < test_one_rhs);
println!("test one: is lhs greater than rhs? - {}", test_one_lhs > test_one_rhs);

let test_two_lhs = Rect { width: 2, height: 1 };
let test_two_rhs = Rect { width: 2, height: 1000 }; 
# assert!(test_two_lhs < test_two_rhs);
println!("test two: is lhs greater than rhs? - {}", test_one_lhs > test_one_rhs);
# }
```

Finally `Ord` isn't quite the same as `Eq` in that it does provide extra methods;
- `cmp` which is like `partial_cmp` but returns `Ordering` without the `Option`
- `max` which returns the greater of the two values
- `min` which returns the lesser
- `clamp` which will return a value so long as its between two other values, or the closest value that is

Like with `PartialOrd`, `Ord` can be derived but has the same ordering quirk. If we want to implement it ourselves, we
only need to implement `cmp`, and the other methods can use that for their default behaviour. Importantly, when
implementing both `PartialOrd` _and_ `Ord`, the result of `partial_cmp` _must_ match `cmp`, though the compiler has no
way of confirming this for you. The easiest way to handle this is if you need to manually implement `PartialOrd`, simply
call `cmp` and wrap it in an `Option`. Let's do that with our Rect type.

```rust
use std::cmp::Ordering;

// Remember PartialEq is required for PartialOrd, Eq is required for Ord
#[derive(Debug, Eq, PartialEq)]
struct Rect {
    width: u64,
    height: u64,
}

impl Rect {
    pub fn area(&self) -> u64 {
        self.width * self.height
    }
}

impl Ord for Rect {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.area().cmp(&rhs.area())
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

# fn main() {
// --- Example comparison ---
    
let test_one_lhs = Rect { width: 2, height: 1 };
let test_one_rhs = Rect { width: 1, height: 1000 };
# assert_eq!(test_one_lhs.cmp(&test_one_rhs), Ordering::Less);
println!("test one: lhs is {:?} than rhs", test_one_lhs.cmp(&test_one_rhs));

// --- You still need to be careful with default behaviour ---
// --- What do you think happens here? ---

let two_one = Rect { width: 2, height: 1 };
let one_two = Rect { width: 1, height: 2 };
let four_four = Rect { width: 4, height: 4 };
println!("{:?}", four_four.clamp(two_one, one_two));
# }
```

Unlike `PartialEq`, neither `PartialOrd` nor `Ord` are generic, they can only be implemented where both the left hand
side and the right hand side are the same type.

### Clone (and Copy)

`Clone` is a bit like `Copy` in that it allows you to duplicate values, however, where `Copy` is implicitly very cheap,
`Clone` can get away with doing a bit more work.

With `Copy`, we can make a copy of data on that is purely on the stack, however, this restricts us to `Sized` data. This
means, for example, `String` which is a smart pointer to data on the heap, can not implement `Copy`. In order to
duplicate `String` we'd need to request new memory on the Heap to place the data into, then copy the data to the new
location, and create a new smart pointer on the stack to point to it.

Requesting heap memory is considered expensive as you have to wait for the operating system to provide you a location
you can use, so it's really handy to differentiate `Clone` from `Copy`.

Luckily, you don't have to do all of this memory allocation stuff yourself. For any type that is built from other types
that already implement `Clone` you can derive `Clone`. 

```rust
#[derive(Clone, PartialEq, Debug)]
struct MyNewType(String); // String already implements Clone, PartialEq and Debug

// --- Testing clone ---

# fn main() {
let a = MyNewType("Hello, world!".to_string());
let b = a.clone();
assert_eq!(a, b);
# }
```

If you need to implement `Clone` yourself (rare and only required in very specific and advanced circumstances), then you
can do so:

```rust,ignore
struct MyNewType(String);

impl Clone for MyNewType {
    fn clone(&self) -> Self {
        // ...
    }
}
```

Finally, there is an optional method in the `Clone` trait called `clone_from`. Its optional because there is a default
implementation built into the trait itself but, again, allows you to override it in case you want to do something like
provide more efficient memory allocation.

In order to derive `Copy`, not only must your type be made from only other types that implement `Copy`, but your type
must also implement `Clone`.

```rust
#[derive(Copy, Clone, PartialEq, Debug)]
struct MyNewType(u32); // This tuple struct uses a u32 which implements Copy and Clone

// --- Testing copy ---

# fn main() {
let a = MyNewType(1);
let b = a; // Copy is automatic when its available
assert_eq!(a, b);
# }
```

### Default

Many built in types in Rust have a default value. Defaults for numbers are typically zero, while `String`s, `Vec`s and
other collections default to being empty. If your type is built from only types that implement `Default` then you can
derive the behaviour of `Default` for your type to be, essentially, the instantiation of your type with all values set
to _their_ default.

```rust
#[derive(Default, Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person::default();
    assert_eq!(&person.name, "");
    assert_eq!(person.age, 0);
    println!("Default persons name is '{}' and their age is '{}'", person.name, person.age);
}
```

Obviously, this may not always be the desired result, so you can obviously implement the trait directly:

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            name: "Jane Doe".to_string(),
            age: 30,
        }
    }
}

fn main() {
    let person = Person::default();
    assert_eq!(&person.name, "Jane Doe");
    assert_eq!(person.age, 30);
    println!("Default persons name is '{}' and their age is '{}'", person.name, person.age);
}
```

You might be wondering if you can derive `Default` for Enums, or if you have to implement it directly, and you actually
can, using an additional attribute that you apply to the value you want to be the default.

```rust
#[derive(Default, Debug, PartialEq)]
enum SomeEnum {
    Variant1,
    #[default]
    Variant2,
}


fn main() {
    let choice = SomeEnum::default();
    assert_eq!(choice, SomeEnum::Variant2);
}
```

Unfortunately the `default` attribute only works when deriving `Default` for unit enums, which means if your enum 
contains nested types, you will have to implement `Default` manually:

```rust
// The nested types here mean we can't derive default
#[derive(Debug, PartialEq)]
enum SomeEnum {
    Variant1(u32),
    Variant2(String),
}

impl Default for SomeEnum {
    fn default() -> Self {
        SomeEnum::Variant2("Hello".to_string())
    }
}

fn main() {
    let choice = SomeEnum::default();
    assert_eq!(choice, SomeEnum::Variant2("Hello".to_string()));
}
```

### Hash

Hashing as a concept is more complex than we'll go in to here, however, to keep it simple, in Rust there is trait that
describes a type that is `Hash` which means that it can be "hashed", and another trait called `Hasher` which does the
hashing.

You _generally_ don't need to worry too much about this, but it is useful if you want your type to work as a key in a
`HashMap` or similar data structure.

So long as your type is constructed only of other types that implement `Hash`, then you can derive it, though if you 
need more control than that, then you can of course implement the trait methods yourself. This might be useful if you
want to skip over types that can't be hashed _BUT_ when using `Eq`, if `A == B`, then `hash(A) == hash(B)` must also
be true.

> I've actually secretly derived `Hash` on some types in the code examples in this chapter just to test my code behaves
> correctly. See if you can spot them and the assocciated tests!

To derive it yourself simply use the derive attribute, and you'll be good to use it in a `HashMap`:

```rust
#[derive(Hash)]
struct Email(String);
```

Error Handling
--------------

### Display

Before we jump straight into the `Error` trait, lets recap on `Display`. This trait allows us to display information
related to the type that implements it. You get to decide what that information is, but `Display` is pretty broad. Once
you implement it, if you pass a value of your type into a macro like `println!` or `format!`, then `Display` defines
how the type will be rendered.

`Display` only has one method which you must implement, it takes `&self`, and a mutable pointer to a `Formatter` and
returns a `fmt::Result` which is a type alias for `Result<(), fmt::Error>`. The easiest way to implement it is with
`write!` macro which returns this same type, and to `use std::fmt` to slightly simplify the namespacing:

```rust
use std::fmt;

struct MyUnitStruct;

impl fmt::Display for MyUnitStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "My unit struct")
    }
}
```

### Error

The `Error` trait is applied to types that are specifically used to represent something that went wrong during the
execution of the code.

Although `Result`s do not _require_ the `Error` trait be implemented for types in their Error variant, it is definitely
worth doing as error types with the `Error` trait provide a lot of utility for very little effort.

The trait itself has several "provided" methods but none that you need to implement yourself. You're unlikely to want to
alter the provided behaviour of the `Error` trait which means the only thing you need to do is make sure that your
error type _also_ implements `Debug` and `Display`. As we know, `Debug` is usually derivable, so that just leaves 
`Display`. Let's create a custom Error for a fridge to demonstrate how we _might_ do this.

```rust
use std::{fmt, error};

#[derive(Debug)]
enum FridgeError {
    TooHot(f32),
    TooCold(f32),
    PowerFailure,
}

impl fmt::Display for FridgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FridgeError::TooHot(temp) => write!(f, "Fridge temperature recorded at {temp} which is too hot"),
            FridgeError::TooCold(temp) => write!(f, "Fridge temperature recorded at {temp} which is too cold"),
            FridgeError::PowerFailure => write!(f, "A power failure has been detected in the fridge"),
        }
    }
}

impl error::Error for FridgeError {}
```

While we've avoided talking about the wider ecosystem so far, it's worth mentioned there are some _extremely_ powerful
Error libraries that might change the way you work with errors. We will cover these in the Ecosystem part of the book.

Converters
----------

### From / Into

By now you're probably beginning to understand how important types are to Rust, but sometimes, you need to take the data
from one type, and move it to another type. `From` and `Into` are the easiest ways to do this, providing the `from` and
`into` methods respectively. For example, you'll regularly see people turning a string slice into a string in one of
these two ways:

```rust
# fn main() {
let first = String::from("first");
let second: String = "second".into();
println!("{first}, {second}");
# let hidden_example = String::from(second); // String also implements `From<String>`
# }
```

You can immediately see a couple of difference here. In the first example, we don't need to type hint the variable as
its clear that we're creating a `String` from another value. That value can be anything so long as there is a `From<T>`
(in our case an `impl From<&str> for String`) implementation for the type of that value, and String has quite a few
`From` implementations.

In the second example, we call `into` on the string slice however, we need to tell Rust "into what", so we use a type 
hint to say we're changing the reference to a string slice into a String. As with `From`, there could many types you can
turn something into, so long as there is an `Into<T>` (in our case, `impl Into<String> for &str`) for that type.

What's really cool though is you rarely have to implement `Into` yourself. You might have realised that the 
functionality of `impl Into<String> for &str` is probably identical to `impl From<&str> for String`, and Rusts
maintainers realised that too! There is a generic implementation of Into that looks like this:

```rust,ignore
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

We haven't talked about `where` yet, but it's a way of providing type bounds (like when we've used colons in previous 
examples) that's great for when the type bound is a little more complex. This generic implementation simply applies
`Into<U>` for any type where `U` can already be gotten `From<T>`. Simple, but powerful. Because of this however, you
should only ever implement `Into` if you _can't_ implement `From`, which rarely comes up outside of crate scoping which
we'll discuss in the next section of the book.

### TryFrom / TryInto

Sometimes, its just not possible to guarantee that a conversion from one thing into another will actually work. 
`TryFrom` and `TryInto` can help you with possible errors using another feature of traits we haven't discussed,
associated types.

To oversimplify things a little, let's say you're talking to an external system that thinks about pets like this:

```rust
enum PetType {
    Cat,
    Dog,
    Rat, //..
}

struct Pet {
    pet_type: PetType,
    name: String,
}
```

but your system _only_ cares about Cats:

```rust
struct Cat {
    name: String
}
```

We can't `impl From<Pet> for Cat` because not all Pets are Cats. We can use `impl TryFrom<Pet> for Cat` to manage this
for us which is similar to `From` except that it can fail, therefore returns a `Result<Ok, Err>` type where the `Ok`
type is the type we're converting to which is the type we're implementing for and the `Err` type is... hmm, there's no
error type in the generic or what we're implementing for.

It's time to introduce "Associated Types".

TODO: Add link for Associated Types

The official Rust book has a detailed explanation of Associated Types, but I think the best way to think about them is
they are a bit like private static placeholder types. They're similar to generics in that they are defined once and
reused throughout a trait implementation, however, you are not leaving the type up to the person calling the generic
code. 

This loss of flexibility comes with two significant upsides however;

1. The trait is easier to use for the caller (less or no generics)
2. Relatedly, you are reducing the risk area of making mistakes.

```rust
use std::convert::TryFrom; // You do not need to do this since Rust 2021, including for backwards compatability

# #[derive(Debug,PartialEq)]
# enum PetType {
#     Cat,
#     Dog,
#     Rat, //..
# }
# 
# #[derive(Debug)]
# struct Pet {
#     pet_type: PetType,
#     name: String,
# }
# 
# #[derive(Debug)]
# struct Cat {
#     name: String,
# }
#
#[derive(Debug)]
struct NotACatError(Pet);

impl TryFrom<Pet> for Cat {
    type Error = NotACatError;
    
    fn try_from(pet: Pet) -> Result<Cat, Self::Error> {
        if pet.pet_type != PetType::Cat {
            Err(NotACatError(pet))
        } else {
            Ok(Cat { name: pet.name })
        }
    }
}

fn main() {
    let yuki_pet = Pet { pet_type: PetType::Cat, name: "Yuki".into() };
    let yuki_cat_result = Cat::try_from(yuki_pet);
    // This should display "Result: Ok(Cat { name: "Yuki" })"
    println!("Result: {yuki_cat_result:?}");
    
    let lassie_pet = Pet { pet_type: PetType::Dog, name: "Lassie".into() };
    let lassie_cat_result = Cat::try_from(lassie_pet);
    // This should display "Result: Err(NotACatError(Pet { type: Dog, name: "Lassie" }))"
    println!("Result: {lassie_cat_result:?}");
}
```

And yes `TryInto` is automatically provided by Rust for any types that already provide the reverse `TryFrom`
implementation. One thing to note though is, like `into`, you still need to type hint to Rust what the generic parts
are, but because they're now inside a result its a little more verbose:

```rust
# use std::convert::{TryFrom, TryInto}; // You do not need to do this since Rust 2021, including for backwards compatability
#
# #[derive(Debug,PartialEq)]
# enum PetType {
#     Cat,
#     Dog,
#     Rat, //..
# }
#
# #[derive(Debug)]
# struct Pet {
#     pet_type: PetType,
#     name: String,
# }
#
# #[derive(Debug)]
# struct Cat {
#     name: String,
# }
#
# #[derive(Debug)]
# struct NotACatError(Pet);
# 
# impl TryFrom<Pet> for Cat {
#     type Error = NotACatError;
# 
#     fn try_from(pet: Pet) -> Result<Cat, Self::Error> {
#         if pet.pet_type != PetType::Cat {
#             Err(NotACatError(pet))
#         } else {
#             Ok(Cat { name: pet.name })
#         }
#     }
# }
# fn main() {
let yuki_pet = Pet { pet_type: PetType::Cat, name: "Yuki".into() };
let yuki_cat_result: Result<Cat, _> = yuki_pet.try_into();
println!("Result: {yuki_cat_result:?}");
# }
```

Note: that we only need to specify the Ok type of the `Result`, the Error type can be inferred from the `TryFrom` 
implementation, how clever is that! To ask Rust to infer a type, we can use `_`.

Referencing and Dereferencing
-----------------------------

We've talked about this a little bit already but in Rust, rather than having to pass around ownership of a value, you
can instead reference it, while leaving ownership wherever it originated (so long as the reference never outlives the
owned data it points to). Reference's in rust are similar to pointers in other languages that you might have heard of,
in that they are a value which "points at" another location in memory where the actual value is.

Since the reference only points at the data, if you pass it into a function, when the function ends, only the reference
is cleaned up, not the data it was pointing at. Because the reference doesn't own the data it points at, we describe it
as "borrowing" the data. You can have any number of immutable borrows, or a single mutable borrow.

Rust has other kinds of pointers too though, called "Smart Pointers". These are similar to references in that they
"point at" some location in memory, however, they also provide additional functionality to the data. The common example
of this is `String` which is a smart pointer to a string slice that exists on the heap. Not only is it the only way
to manipulate string data, but it also handles things like how that data is freed when the `String` is dropped.


A simpler smart pointer we haven't covered yet is `Box<T>` which _only_ provides the facilities for requesting the 
memory allocation, and freeing it once the `Box` value goes out of scope and is dropped. 

In Rust, it is possible to write code that manipulates memory directly, but it's actually very rare to _need_ to do 
this. We'll cover this later in the `unsafe` chapter which I promise is less scary than it sounds!

The next few traits deal specifically with traits that make references and smart pointers more malleable. 

### Borrow / BorrowMut

`Borrow` allows you to borrow the data of one type as another if its another type, and `BorrowMut` allows you to borrow
that data mutably.

One type that already implements this is `String`. As I mentioned above, `String` is a smart pointer to a string slice
stored on the heap, and it implements `Borrow<str>` to allow us to borrow the data as if it were a string slice type.

```rust
// We need to bring the trait in scope
use std::borrow::Borrow;

fn say_hello(who: &str) {
    println!("Hello, {who}!");
}

fn main() {
    let name = "Yuki".to_string();
    say_hello(name.borrow());
}
```

In this case, instead of borrowing the `String` as `&String` like it normally would, Rust sees that it's being passed 
into a function that accepts a `&str` _and_ that `String` implements `Borrow<str>` so it can use that to provide a
reference to a string slice, `&str`.

There are blanket implementations of both traits, so, for any type `T` you know that `Borrow<T>` and `BorrowMut<T>` are
already implemented, meaning that any value of type `T` can be borrowed as its referenced type `&T`, and any mutable
value of type `T` can be mutably borrowed as `&mut T`.

You can also provide further implementations of `Borrow` yourself allowing you to borrow the same data as if it were a
variety of types, however there are some important restrictions that effectively mean you should only implement borrow
for types where the internal representation remains the same. This means you should never implement borrow that only
returns part of the underlying data. One way to check this is to be sure the hash of a value `v` must be the same as the
hash `v.borrow()`.

A common pattern in Rust is to use wrappers around other types, this is the "new type" pattern. Imagine you have a type
that represents an email address. Obviously a sensible type to store that data in is some kind of string (lets say
specifically a `String` for simplicity), however, there's no validation on creating a String, so how do we know if any
given string contains a valid email. For that we can wrap the string in a "new type" (often written as "newtype").

```rust
use std::borrow::Borrow;
# use std::error::Error;
# use std::fmt;
# use std::ops::Deref;
# use std::hash::{DefaultHasher, Hash, Hasher};
# 
# #[derive(Debug)]
struct InvalidEmailError; // Use the eye icon to see all the error code ->
# // Arguably the error code isn't necessary for this example but now we've explained 
# // it I think its important to do things properly

# impl fmt::Display for InvalidEmailError {
#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
#         write!(f, "{self:?}")
#     }
# }
# 
# impl Error for InvalidEmailError {}
#
# #[derive(Hash, Debug)]
struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Email, InvalidEmailError> {
        if Email::is_valid(&email) {
            Ok(Email(email))
        } else {
            Err(InvalidEmailError)
        }
    }
    
    fn is_valid(email: &str) -> bool {
        // Note: this is oversimplified but is less likely to give false negatives than many approaches
        let at_pos = email.find('@');
        if let Some(at_pos) = at_pos {
            return email.len() >= 3 && at_pos > 0 && at_pos < email.len() - 1;
        }
        false
    }
}

impl Borrow<String> for Email {
    fn borrow(&self)  -> &String {
        &self.0
    }
}

impl Borrow<str> for Email {
    fn borrow(&self)  -> &str {
        &self.0
    }
}

fn test_str(s: &str) {
    println!("{s} is an &str")
}

fn test_string(s: &String) {
    println!("{s} is an &String")
}

fn main() {
    let good_address = "example@example.com";
    let possible_email = Email::new(good_address.to_string());
    let email = possible_email.expect("failed to create email, check result");

    // We can borrow the string slice inside of email
    test_str(email.borrow());
    test_string(email.borrow());
# 
#     // Hello curious reader 👋🏻. When using borrow the hash of the borrowed value must be equal to the hash of the 
#     // original value (as I understand it) 
#     let borrowed_email: &str = email.borrow();
#     assert_eq!(hash(&email), hash(borrowed_email));
# 
#     // These just test if the is_valid logic works, I don't want to look like an idiot now do I? 😅
#     assert!(Email::is_valid("a@b"));
#     assert!(!Email::is_valid("@ab"));
#     assert!(!Email::is_valid("ab@"));
#     assert!(Email::is_valid("example@example.com"));
}
# 
# fn hash<H: Hash + ?Sized>(hashable: &H) -> u64 {
#     let mut hasher = DefaultHasher::new();
#     hashable.hash(&mut hasher);
#     hasher.finish()
# }
```

`BorrowMut` does exactly the same thing but gives you a mutable reference instead. In our `Email` example we could give
a mutable reference to the underlying `String`... but in this case, should we? Allowing mutation of the data inside the
email would bypass the logic of our `Email` type that guarantees the email address is valid.

Now, there's an important caveat to `Borrow` and `BorrowMut`.

- If a type implements borrow, where it's rue that `x == y`, then it must also be true that `x.borrow() == y.borrow()` 
- If your type implements `Eq` then your borrowed type must also implement `Eq`
- Furthermore, if `x > y` then `x.borrow() > y.borrow()`, if `x < y` then `x.borrow() < y.borrow()`, etc
- Finally, if we have a hashing function, if `hash(x) == hash(y)` then `hash(x.borrow()) == hash(y.borrow())`

There are no compiler checks for this, you need to be sure that its true when you implement Borrow and, as you can 
probably guess, `Borrow` really only works for changing the exact binary representation of a value from one type to
another, making it less useful for compound types. What if you only want to borrow _part_ of a type.

### AsRef / AsMut

Sometimes, you might have a type where the internal representation could very cheaply be read and/or manipulated as a
different type. For example `String` exists on the Heap, but the data that lives there is identical to a `str`. This
means that any function that takes a reference to a string slice should really be able to also take a reference to a
`String`. This is represented by the trait implementation `impl AsRef<str> for String`. In fact, what is a string but
an array of `u8`s, and indeed you'll find that there is `impl AsRef<[u8]> for String` too. 

> Note: the AsRef trait generic type does not require an ampersand as this is implied.

Remember earlier we had our `Cat` type which only had a name. We could, if we wanted, implement `AsRef<str>` so that
it can be used in the place of a `&str`:

```rust
struct Cat {
    name: String,
    age: u8,
}

impl AsRef<str> for Cat {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

fn cuddle(who: &str) {
    println!("Cuddle {who}");
} 

fn main() {
    let yuki = Cat { name: "Yuki".into(), age: 15 };
    cuddle(yuki.as_ref());
}
```

Arguably, we could make this code even friendly by changing the `cuddle` to take a generic, and then calling `.as_ref()`
in the function itself. This code looks a little scarier, but once you get used to seeing code like this, you can write
far more flexible and easy to use code. 

```rust
# struct Cat {
#     name: String,
# }
# 
# impl AsRef<str> for Cat {
#     fn as_ref(&self) -> &str {
#         &self.name
#     }
# }
# 
fn cuddle<S: AsRef<str> + ?Sized>(who: &S) {
    println!("Cuddle {}", who.as_ref());
} 

fn main() {
    let yuki = Cat { name: "Yuki".into() };
    cuddle(&yuki);
    let yuki = "Yuki";
    cuddle(yuki); // &str also implements AsRef<str>
}
```

`AsMut` is essentially the same as `AsRef` but for mutable references instead!

### Deref / DerefMut

We mentioned above that when referencing, you create a pointer to another piece of data and when dereferencing you're
moving back from the reference to whatever is underneath. This can be esspecially useful when working with references
to references:

```text
       <--- reference -----        <--- reference -----
&&Cat                        &Cat                        Cat
       --- dereference --->        --- dereference --->
```

We also talked a bit about smart pointers which are not references but a way to wrap data with additional functionality.
To get inside a smart pointer, we use the `Deref` trait, this is why `String` can be used as if it were a `str`.

```text
        <--- Smart Pointer -----
String                            str
        ----- dereference ----->
```

When a smart pointer wraps a mutable type (remember `str` is not itself mutable) then you can also implement `DerefMut`
_but_ you need to think carefully about when it's appropriate to do this.

Let's return to our `Email` type, it makes sense to allow our `Email` to be used as an immutable String, so lets
implement `Deref` for it:

```rust
use std::ops::Deref;
# use std::error::Error;
# use std::fmt;
# 
# #[derive(Debug)]
struct InvalidEmailError; // Use the eye icon to see all the error code ->
# // Arguably the error code isn't necessary for this example, but now we've explained 
# // it above, I think it's important to do things properly

# impl fmt::Display for InvalidEmailError {
#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
#         write!(f, "{self:?}")
#     }
# }
# 
# impl Error for InvalidEmailError {}
#
# #[derive(Debug)]
struct Email(String);

impl Email {
    // ...
    pub fn new(email: String) -> Result<Email, InvalidEmailError> {
        if Email::is_valid(&email) {
            Ok(Email(email))
        } else {
            Err(InvalidEmailError)
        }
    }
    
    pub fn is_valid(email: &str) -> bool {
        // Note: this is oversimplified but is less likely to give false negatives than many approaches
        let at_pos = email.find('@');
        if let Some(at_pos) = at_pos {
            return email.len() >= 3 && at_pos > 0 && at_pos < email.len() - 1;
        }
        false
    }
}

impl Deref for Email {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn test_str(s: &str) {
    println!("{s} is an &str")
}

fn main() {
    let good_address = "example@example.com";
    let possible_email = Email::new(good_address.to_string());
    let email = possible_email.expect("failed to create email, check result");

    // We can deref the Email into a string slice simply by referencing it... yes, I know, it's weird
    test_str(&email);
    
    // We can even pass the email type to its own validation function now!
    assert!(Email::is_valid(&email));
# 
#     // I kept these tests in case I change anything by mistake 
#     assert!(Email::is_valid("a@b"));
#     assert!(!Email::is_valid("@ab"));
#     assert!(!Email::is_valid("ab@"));
#     assert!(Email::is_valid("example@example.com"));
}
```

Let's think about this differently though, what if instead of dereferencing to a str, we dereferenced to a `String`,
_and_ we allowed mutability. Our `Email` type here does some rudimentary validation, if we allowed mutability of the
inner `String`, we allow people to change the email to be invalid, and lose the benefit of the `Email` type.

```rust
use std::ops::{Deref, DerefMut};
# use std::error::Error;
# use std::fmt;
# use std::hash::{DefaultHasher, Hash, Hasher};
# 
# #[derive(Debug)]
struct InvalidEmailError; // Use the eye icon to see all the error code ->
# // Arguably the error code isn't necessary for this example, but now we've explained 
# // it above, I think it's important to do things properly

# impl fmt::Display for InvalidEmailError {
#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
#         write!(f, "{self:?}")
#     }
# }
# 
# impl Error for InvalidEmailError {}
#
# #[derive(Debug)]
struct Email(String);

impl Email {
    // ...
#     pub fn new(email: String) -> Result<Email, InvalidEmailError> {
#         if Email::is_valid(&email) {
#             Ok(Email(email))
#         } else {
#             Err(InvalidEmailError)
#         }
#     }
#     
#     pub fn is_valid(email: &str) -> bool {
#         // Note: this is oversimplified but is less likely to give false negatives than many approaches
#         let at_pos = email.find('@');
#         if let Some(at_pos) = at_pos {
#             return email.len() >= 3 && at_pos > 0 && at_pos < email.len() - 1;
#         }
#         false
#     }
}

impl Deref for Email {
    // Note that DerefMut requires Deref _and_ uses it's target so it must be the same type
    type Target = String;

    // ...
#     fn deref(&self) -> &Self::Target {
#         &self.0
#     }
}
impl DerefMut for Email {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let good_address = "example@example.com";
    let possible_email = Email::new(good_address.to_string());
    let mut email = possible_email.expect("failed to create email, check result");
    
    match Email::is_valid(&email) {
        true => println!("{} is a valid email", email.deref()),
        false => println!("{} is NOT a valid email", email.deref()),
    }
    
    email.remove(7);
    
    match Email::is_valid(&email) {
        true => println!("{} is a valid email", email.deref()),
        false => println!("{} is NOT a valid email", email.deref()),
    }
# 
#     // I kept these tests in case I change anything by mistake 
#     assert!(Email::is_valid("a@b"));
#     assert!(!Email::is_valid("@ab"));
#     assert!(!Email::is_valid("ab@"));
#     assert!(Email::is_valid("example@example.com"));
}
```

Other
-----

### Drop

Rust is _very_ good at cleaning up after itself, especially when you use the standard library:
- If your variable allocate heap memory, that memory is released when the variable that owns it goes out of scope
- If you open a file to read or write, it's closed when the file handler goes out of scope
- If you start a TCP connection, its ended when the handler goes our of scope

The Rust standard library is achieving this with the `Drop` trait.

You can implement the drop trait yourself:

```rust
struct UnitStruct;

impl Drop for UnitStruct {
    fn drop(&mut self) {
        println!("UnitStruct was dropped")
    }
}

fn main() {
    println!("In main");
    {
        println!("In inner scope");
        println!("Creating UnitStruct");
        let unit_struct = UnitStruct;
        println!("Leaving inner scope");
    }
    println!("Leaving main");
}
```

When a variable goes out of scope, if it implements the `Drop` trait, then the functionality on that trait is called,
which allows you to write cleanup code for the type implementing the trait. Depending on the type of programming you do
you may not need to think about this trait very much... _except_, there is one thing worth considering.

Each of the examples I gave above is "blocking". That means that the program will have to wait until whatever the `drop`
method of the `Drop` trait needs to do is complete before continuing. You may want to be mindful of this when you allow
things to go out of scope, and be aware of what any library code you're consuming might be doing.

Most of the time this isn't worth worrying too much about, however, if you do find you want to very precisely control
when variables are dropped and have any `Drop` functionality acted on, then let me introduce you to my all-time
favourite function `std::mem::drop`. Here it is in full:

```rust
pub fn drop<T>(_x: T) {}
```

Yeah, that's not a mistake. It has one generic variable and no function body. Remember that variables in Rust are owned
by the function they exist in, and when they leave that function they're dropped. The intention of this function is that
at the exact moment you want to cause a variable to be cleaned up, you pass ownership of that variable into this 
function, the function immediately ends, and, if the variable has a `Drop` implementation, then that code is run then 
and there.

Next Chapter
------------

Now that we've learned about the `Error` trait, in the next chapter we'll dive deeper into error handling, including
learning a whole new operator that makes working with errors in Rust simply sublime.
