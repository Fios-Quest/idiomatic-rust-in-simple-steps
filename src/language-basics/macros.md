Macros
======

Macro's let us do meta programming in Rust. This allows us to treat our code as data, manipulating it, expanding it, and
creating new code.

Over this chapter we'll learn how to do three things with macros:

1. Generate boilerplate code to mitigate repeating ourselves
2. Create a very basic domain specific language (DSL)
3. Create pseudo-functions that can take any number of parameters

There are two types of macro in Rust, `macro_rules!` and `proc macro`s. We won't be dealing with `proc macro`s in this
book, but they are what allow you to create custom Derive macros (like `#[derive(Clone)]`), and custom attributes like
`#[YourAttriburte]`. They also let you make the same function style macros we'll be making with `macro_rules!` but can
unlock even more power!

Anatomy of `macro_rules!`
-------------------------

`macro_rules!` is, itself, a macro, providing its own DSL that allows you to create more macros. This lets things get
very powerful and, honestly, very weird. Let's take it slow.

We'll start by making a hello world macro.

```rust
macro_rules! hello {
    () => { String::from("Hello, world") };
}

fn main() {
    assert_eq!(hello!(), "Hello, world".to_string());
}
```

Let's break it down. Immediately after `macro_rules!` we provide the name of the macro we're creating, in this case
`hello`. We then have some curly brackets surrounding the definition of the macro. 

Inside the definition, we have some empty brackets, this tells `macro_rules` that our macro does not take any
parameters. After that we have an arrow and the code block, that will become our generated code.

This type of macro _could_ be useful if you have a block of code you need to repeat but don't want to put it in a
function. Let's upgrade our macro with a parameter, I'm also going to .

```rust
macro_rules! hello {
    ($name:literal) => { 
        { 
            let mut output = String::from("Hello, ");
            output.push_str($name);
            output
        }
    };
}

fn main() {
    assert_eq!(hello!("Yuki"), "Hello, Yuki".to_string());
}
```

Things got a little bit weird here, right? Lets step through our changes.

First, we added a parameter, but you'll immediately notice this looks nothing like a normal function parameter in Rust.

In macro rules, parameters are preceded by a dollar symbol, followed by a colon, and what's called a designator.

Designators are a bit like types but are specific to how we think about the component parts of a language. We can't
specify "str" here, but we can specify that it's a literal, which is any raw value, such as a string slice, a number, a
boolean, etc.

-- Add more about designators here --

The second change we've made here is that inside of the code block... we've added _another_ block.

The reason for this is that when we _use_ the macro, Rust pretty much does a drop in replacement of the code block at
the point that you place the macro. If we didn't have the extra brackets, when we use the macro in our code would
look to Rust as if it were this:

```rust,compile_fail
# fn main() {
assert_eq!(
    let mut output = String::from("Hello, ");
    output.push_str("Yuki");
    output,
    "Hello, Yuki".to_string()
);
# }
```

This doesn't work because `assert_eq!`, which is also a macro, expects its parameters to be expressions (`:expr`). By
adding the brackets, we turn multiple statements into an expression.

```rust
# fn main() {
assert_eq!(
    {
        let mut output = String::from("Hello, ");
        output.push_str("Yuki");
        output
    },
    "Hello, Yuki".to_string()
);
# }
```

Expressions in Rust are particularly useful as they have a type and a value, just like variables, allowing you to use
them inside other expressions.

Before we get too much deeper, let's talk about why macro's have this weird code block that wraps the rest of the macro
definition, rather than just having something like `macro_rules! hello() { String::from("Hello, world")}`.

`macro_rules!` can do pattern matching over the arguments you pass into your macro. This means we can create macros
that can take wildly different inputs. Let's bring back our original behaviour for an empty `hello!` macro:


```rust
macro_rules! hello {
    () => { String::from("Hello, world") };
    ($name:literal) => { 
        { 
            let mut output = String::from("Hello, ");
            output.push_str($name);
            output
        }
    };
}

fn main() {
    assert_eq!(hello!(), "Hello, world".to_string());
    assert_eq!(hello!("Yuki"), "Hello, Yuki".to_string());
}
```

Lets make one more improvement that will help us maintain consistency. We can call our macro from inside our macro.
Just in case we want to change our greeting later, lets not have `"Hello, "` twice, from our macro with no arguements
call our hello macro with the name filled in.

```rust
macro_rules! hello {
    () => { hello!("world") };
    ($name:literal) => { 
        { 
            let mut output = String::from("Hello, ");
            output.push_str($name);
            output
        }
    };
}

fn main() {
    assert_eq!(hello!(), "Hello, world".to_string());
    assert_eq!(hello!("Yuki"), "Hello, Yuki".to_string());
}
```

Now I think our hello macro is missing one critical feature; what if I want to greet lots of people?

We can tell our macro that we expect a repeating value by surrounding an argument with `$(...),` followed by either a
`?`, a `*`. Similar to regex rules:

- `?` means the arguement is repeated zero or one times
- `+` means one or more times
- and `*` means zero or more times

We already have zero and one parameters dealt with, so we want a branch in our macro that takes two or more inputs:

```rust
macro_rules! hello {
    () => { hello!("world") };
    ($name:literal) => { 
        { 
            let mut output = String::from("Hello, ");
            output.push_str($name);
            output
        }
    };
    ($name:literal, $($other:literal),+) => {
        {
            let mut output = hello!($name);
            $(
                output.push_str(" and ");
                output.push_str($other);
            )+;
            output
        }
    }
}

fn main() {
    assert_eq!(hello!(), "Hello, world".to_string());
    assert_eq!(hello!("Yuki"), "Hello, Yuki".to_string());
    assert_eq!(hello!("Yuki", "Daniel", "Indra"), "Hello, Yuki and Daniel and Indra".to_string());
}
```

Being able to quickly compose macros like this can save us a lot of time when repeating the same code over and over.

Usefully DRY
------------

> Note: I've slightly altered the code in this section to not rely on third party crates, such as Uuid and paste. If
> you're comfortable with crates then feel free to follow the links to the actual code to see what the real code looks
> like.

The example we've run through to build up our understanding of how macro's work is very abstract and not very useful,
so I wanted to go over a quick example of how I've started using Macro's.

In the [Job Tracker](https://github.com/Fios-Quest/job-tracker/) app I've been building with the help of folks in the
chat of my [streams](https://www.youtube.com/playlist?list=PLW2L8KbM0O7Z2KroHNNBWY1UApqmeiyqe), I've leaned heavily
into composing my types from Traits to form common behaviour.

For example, at time of writing, I allow the user to create `Company`s, `Role`s, and `Flag`s. `Role`s and `Flag`s
belong to `Company`s so those types implement the following trait:

```rust
pub trait HasCompany {
    fn get_company_id(&self) -> u128;
}
```

The trait itself does not provide any code, so each item that implements this code must decide on its behaviour. I'm
a big believer in unit tests so lets look at how that works. The `Role` type, trait implementation and test look roughly
like this:

```rust
# pub trait HasCompany {
#     fn get_company_id(&self) -> u128;
# }
# 
#[derive(Clone, Debug)]
pub struct Role {
    pub id: u128,
    pub company_id: u128,
    pub name: String,
}

impl HasCompany for Role {
    fn get_company_id(&self) -> u128 {
        self.company_id
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_role_get_company_by_id() {
        let role = Role {
            id: 1234,
            company_id: 5678,
            name: "Test company".into(),
        };
        assert_eq!(role.get_company_id(), 5678);
    }
}
```

That's fine, but we're going to be doing this for every item that implements that trait, as well as every other trait
each item implements. Every time we add a new storable item we'll have to add tests for its implementation.

The way I got around this was, first I created a trait allowing me to create test instances of the types I want to test,
then I created macros that use that trait to run the test:

```rust
pub trait TestHelper: Sized {
#     // I'm actually using anyhow for errors
    fn new_test() -> Result<Self, String>;
}

macro_rules! test_has_company_id {
    ($test_name:ident, $storable:ident) => {
        #[test]
        fn $test_name() {
            let storable = $storable::new_test().expect("Could not create storable");
            assert!(storable.get_company_id() > 0);
        }
    };
}
```

By implementing the trait for each type that I want to test, I can add tests trivially like this:

```rust
# pub trait TestHelper: Sized {
#     fn new_test() -> Result<Self, String>;
# }
# 
# macro_rules! test_has_company_id {
#     ($test_name:ident, $storable:ident) => {
#         #[test]
#         fn $test_name() {
#             let storable = $storable::new_test().expect("Could not create storable");
#             assert!(storable.get_company_id() > 0);
#         }
#     };
# }
# 
# pub trait HasCompany {
#     fn get_company_id(&self) -> u128;
# }
# 
# #[derive(Clone, Debug)]
# pub struct Role {
#     pub id: u128,
#     pub company_id: u128,
#     pub name: String,
# }
# 
# impl HasCompany for Role {
#     fn get_company_id(&self) -> u128 {
#         self.company_id
#     }
# }
# 
impl TestHelper for Role {
    fn new_test() -> Result<Role, String> {
        Ok(Role {
            id: 1234,
            company_id: 5678,
            name: "Test company".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    test_has_company_id!(test_role_get_company_by_id, Role);
}
```

> Aside: while this is a _very_ simple example, there are more complex examples in the job track like the ones that
> manage the act of storing and recalling these storable objects.

