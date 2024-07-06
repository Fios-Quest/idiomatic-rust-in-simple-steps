Traits
======

Traits are powerful tools that allow us to describe how data types, can be used. Traits are typically applied to
multiple data types to provide a common interface between across them. Before we get to that though, we have to start
with how we apply behaviour to data types.

Impl
----

Once you've created your own data type, you'll want to start using it with functions, but this can get a bit messy. Take
this example:

```rust
// Our User data type
struct User {
    name: String,
    fur_color: String,
}

// A function that takes our User type
fn to_string(user: User) -> String {
    let User { name, fur_color } = user;
    format!("User name: {name}\nUser fur color: {fur_color}")
}

// Usage
fn main() {
    let yuki = User {
        name: "Yuki".to_string(),
        fur_color: "White".to_string(),
    };
    println!("{}", to_string(yuki));
}
```

We have a nicely named function that does what it says, taking a User and turning it into a string, however, because
there is no function overloading in Rust (you can't have multiple functions with the same name but different parameter
lists), we can only have one `to_string` function in scope at any time.

There are a few ways around this: in the case where the types are similar enough, you might be able to use a generic,
you could "rename" the function using `as` (we'll talk about that more in the section on modules), or you could rename
the function to something even more specific such as `user_to_string`...

Or, you could make the function part of the implementation of the type itself using an `impl` block. This will make the
function a part of the `User` type itself, and can be used to make functions called from `User` or on an instantiated
object like `yuki`.

Let's make an impl block and move our functionality in there:

```rust
struct User {
    name: String,
    fur_color: String,
}

impl User {
    fn to_string(&self) -> String {
        let User { name, fur_color } = user;
        format!("User name: {name}\nUser fur color: {fur_color}")
    }
}

fn main() {
    let yuki = User {
        name: "Yuki".to_string(),
        fur_color: "White".to_string(),
    };
    println!("{}", yuki.to_string());
}
```

You'll notice that the function signature changed from `fn to_string(user: User) -> String` to
`fn to_string(&self) -> String`. The `&self` parameter is a special reference to the object on which the function is
called. Functions that belong to instantiated data types like this are also called methods. We don't need to provide
the type here as the type of `self` is implicit.

The same rules for passing variables in functions also apply to `self`:
- `&self` is an immutable reference to self
- `&mut self` is an mutable reference to self
- `self` passes the ownership of self into the method, meaning it will be consumed unless returned

You can also make functions that are available on the uninstantiated type, as in, they are not available on our `user`
variable, but they _are_ available on the `User` type. These are often called static methods. A common example for such 
a method is for instantiating the data type, which is particularly useful if the type has non-public properties.

```rust
struct User {
    name: String,
    fur_color: String,
}

impl User {
    fn new(name: String, fur_color: String) -> Self {
        User {
            name,
            fur_color,
        }
    }
  
    fn to_string(self) -> String { // ...
#         let User { name, fur_color } = self;
#         format!("User name: {name}\nUser fur color: {fur_color}")
#     }
}

fn main() {
    let yuki = User::new("Yuki".to_string(), "White".to_string());
    println!("{}", yuki.to_string());
}
```
 
The function `new` is called from the `User` type, and returns `Self`, this is a special keyword meaning the type that
is being used for the call, which in this case is `User`. So, when we call `User::new` with the parameters for `name`
and `fur_color`, it will return a new `User` object with those fields filled in. You'll also notice that when the
properties of the object are the same as the variables being used to set them, we don't need to add the colon, eg, you
don't need to do `name: name`, just putting `name` is fine.

You can also create "methods", which are functions that can be called on the instantiated data. We do this by making the
first parameter `self`, `&self` or `&mut self`. We'll talk more about the difference between these in the
[generic functions](#generic-functions) section below.

For now, we'll replicate the existing behaviour of our `to_string` function into the `User` implementation:


We moved the function into the impl block, changed the first parameter from `user: User` to just `self`, we don't need
to specify the type. We also changed the way we call the function from `to_string(yuki)` to `yuki.to_string()`.



Traits
------

`impl` is cool, but we can take it a step further and define behaviour that can be implemented for multiple types.

For example, lets say that we have a similar type to our `User` type called `Admin`.

```rust,noplayground
struct User {
    name: String,
    fur_color: String,
}

struct Admin {
    name: String,
}
```

We might want both `User` and `Admin` to have the method `to_string`. We can define it in a trait.

```rust
trait ExampleToString {
    fn to_string(self) -> String;
}
```

We don't need to define the body of the function (though you can, if you want to provide some default behaviour), we
just need to describe its properties and return type. We can then implement that trait for a given type using
`impl <trait> for <type>`.


```rust
# struct User {
#     name: String,
#     fur_color: String,
# }
# 
# impl User {
#     fn new(name: String, fur_color: String) -> Self {
#         User {
#             name,
#             fur_color,
#         }
#     }
# }
# 
# trait ExampleToString {
#     fn to_string(self) -> String;
# }
#
impl ExampleToString for User {
    fn to_string(self) -> String {
        let User { name, fur_color } = self;
        format!("User name: {name}\nUser fur color: {fur_color}")
    }
}
#
# fn main() {
#     let yuki = User::new("Yuki".to_string(), "White".to_string());
#     println!("{}", yuki.to_string());
# }
```

Now we've moved the method from `impl User` to `impl ExampleToString for User` we can still access the method on `yuki`
with `yuki.to_string()`. We can also implement the same trait for `Admin`.

```rust
# struct Admin {
#     name: String,
# }
# 
# impl Admin {
#     fn new(name: String) -> Self {
#         Admin { name }
#     }
# }
# 
# trait ExampleToString {
#     fn to_string(self) -> String;
# }
#
impl ExampleToString for Admin {
    fn to_string(self) -> String {
        let Admin { name } = self;
        format!("Admin name: {name}")
    }
}
#
# fn main() {
#     let indra = Admin::new("Indra".to_string());
#     println!("{}", indra.to_string());
# }
```

Why is this better than just implementing the `to_string` function on to `User` and `Admin`, if anything, this is more
work right? Well, the cool thing about this is that we can use the trait as a trait guard in generics, which we'll see
in just a moment.

Generic Functions
-----------------
