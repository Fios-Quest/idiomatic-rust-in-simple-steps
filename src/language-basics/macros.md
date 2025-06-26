Macros
======

Macro's let us do meta programming in Rust. This allows us to treat our code as data, manipulating it, expanding it, and
creating new code.

Over this chapter we'll learn how to do three things with macros:

1. Generate boilerplate code to mitigate repeating ourselves
2. Create pseudo-functions that can take any number of parameters
3. Create a very basic domain specific language (DSL)

There are two types of macro in Rust, `macro_rules!` and `proc macro`s. We won't be dealing with `proc macro`s in this
book, but they are what allow you to create custom Derive macros (like `#[derive(Clone)]`), and custom attributes like
`#[YourAttriburte]`. They also let you make the same function style macros we'll be making with `macro_rules!` but can
unlock even more power!

Anatomy of `macro_rules!`
-------------------------

`macro_rules!` is, itself, a macro, providing its own DSL that allows you to create more macros. This lets things get
very powerful and, honestly, very weird. Let's take it slow.

The general layout of `macro_rules!` looks like this:

```rust,compile_fail
// We invoke the `macro_rules!` macro usually at the module level rather than in
// a function
macro_rules! <macro_name> {
    // A list of function-like code blocks with brackets a match pattern
    // potentially including "metavariables". Each of these blocks defines a
    // single rule that is matched based on the pattern and stores matching
    //  "metavariables" for use in the macro.  Don't worry, we''ll explain all
    // of this very soon.
    ( <match_pattern> ) => {
        // curly braces surround the macro body. This is used to generate code
        // at the invocation site of the macro.
    };
    // You can have more rules but they need to have a different pattern of
    // metavariables to match against.
    ( <match_pattern> ) => {
        // different rules can do completely different things, and can even
        // call the macro again recursively
    };
}
```

When you invoke your macro it works as a sort of replacement, generating new code to exist at that point, but rather
than it being a copy-paste, `macro_rules!` works on the Abstract Syntax Tree of your program making it much safer and
more fully featured.

Hello, macro!
-------------

We'll start by making a hello world macro that produces a string.

```rust
macro_rules! hello {
    () => { String::from("Hello, world") };
}

fn main() {
    assert_eq!(hello!(), "Hello, world".to_string());
}
```

Let's break it down. As we said above, immediately after `macro_rules!` we provide the name of the macro we're creating,
in this case `hello`. Our first draft won't match anything between the brackets, so we leave those empty. We then have
an arrow, followed by some curly brackets surrounding what our macro will generate.

Our `hello` macro simply creates a string containing `"Hello, world"` at the site where the macro is called (in this
case inside of an `assert_eq!` macro).

This type of macro _could_ be useful if you have a block of code you need to repeat but don't want to put it in a
function, but let's be honest, that's not very likely.

Let's upgrade our macro to match a pattern.


```rust,editable
macro_rules! hello {
    (this must be present) => { String::from("Hello, world") };
}

fn main() {
    assert_eq!(hello!(this must be present), "Hello, world".to_string());
    // assert_eq!(hello!(this wont compile), "Hello, world".to_string());
}
```

What? What?! This is obviously madness, what kind of parameters are we passing to this macro?

The key to understanding the power of macros is that they _don't_ take parameters. The thing in the brackets at the
start of each rule is a pattern, and that pattern can be _almost_ anything. The contents of the macro's invocation is
broken up into something called a token tree, which we'll talk about in the next section. Here, `this must be present`
is considered a token tree made of the tokens: `this`, `must`, `be`, `present`.

We can invoke different rules based on the matched pattern.

```rust,editable
macro_rules! hello {
    (world) => { String::from("Hello, world") };
    (yuki) => { String::from("Hello, yuki") };
}

fn main() {
    assert_eq!(hello!(world), "Hello, world".to_string());
    assert_eq!(hello!(yuki), "Hello, yuki".to_string());
    // assert_eq!(hello!(this wont compile), "Hello, world".to_string());
}
```

We obviously can't write out every possible thing that we might want match on (what if we want to be able to say "hello"
to lots of different people) so we can capture tokens into metavariables.


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

First, we added a metavariable, and you'll immediately notice this looks nothing like a normal function parameter in
Rust. 

In `macro_rules!`, we can parameterise toekns into "metavariables" which are preceded by a dollar symbol, followed by a
colon, and what's called a fragment-specifier (sometimes refered to as a designator).

Fragment-specifiers are a bit like types but are specific to how we think about how Rust classifies tokens trees. We
can't specify "str" here, but we can specify that it's a `literal`, which is any raw value, such as a string slice, a
number, a boolean, etc.

You might wonder what will happen if our macro gets a literal thats not a `str` and the answer is it won't compile and
the person who passed in the non-`str` will get an error relating the the `.push_str` method on `String`.

There are a number of different fragment-specifiers, some of which overlap with each other, we'll go over more of them
later in the chapter.

The second change we've made here is that inside of the code block... we've added _another_ block.

The reason for this is that when we invoke the macro, Rust pretty much does a drop in replacement of the code block at
the point that you place the macro. If we didn't have the extra brackets, when we use the macro in our `assert_eq!`, our
code would look to Rust as if it were this:

```rust,compile_fail
# fn main() {
assert_eq!(
    let mut output = String::from("Hello, ");
    output.push_str("Yuki");
    output,
    "Hello, Yuki".to_string()
);
}
```

This doesn't work because `assert_eq!`, which is also a macro, expects its parameters to be expressions (represented
by the framgent-specifier `:expr`).

In Rust an expression is a token tree that produces a value. So `String::from("Hello, ")` is an expression, but
`let mut output = String::from("Hello, ");` is not. Blocks of code surrounded by `{ ... }` are expressions though
because they have a value, even if the value is the unit type `()`. When we wrap our macro in curly brackets then, and
have the output as the final line, our code block becomes a single expression the value of which is the `output`.

This means that when we add those extra curly brackets to our macro, the generate code now looks like this, which is
valid!

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
    #
}
```

Expressions in Rust are particularly useful as they have a type and a value, just like variables, allowing you to use
them inside other expressions.

Let's go deeper and add another rule. Let's bring back our original behaviour for an empty `hello!` macro:

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

This is fine, but we're repeating ourselves a little bit. In case we might want to change our greeting later, lets not
have `"Hello, "` twice. To maintain consitency we can call our macro from inside our macro!

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

We're nearly there now, but I think our hello macro is missing one critical feature; what if I want to greet lots of
people?

We can "repeat" patterns inside macros by surrounding them with `$(...)` followed by either a `?`, a `+`, or a `*`.
Similar to regex rules:

- `?` means the content is repeated zero or one times
- `+` means one or more times
- and `*` means zero or more times

Specifically with `+` you can add a seperator to the repeat pattern by placing it before the `+`. This token can be
almost anything except the repeat symbols, or token used for delimiters, eg: `$(...),+` or `$(...);+` or even
`$(...)~+` are all fine.

Repeats can be used to match metavariables multiple times, and to repeat code generation for each used repeated
metavariable. When the repeat pattern is used in code generation it will repeat for each combination of metavariables
used within it.

We already have zero and one metavariable dealt with, so we want a branch in our macro that takes two or more inputs:

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
    ($name:literal, $($rest:literal),+) => {
        {
            let mut output = hello!($name);
            $(
                output.push_str(" and ");
                output.push_str($rest);
            )+;
            output
        }
    }
}

fn main() {
    assert_eq!(hello!(), "Hello, world".to_string());
    assert_eq!(hello!("Yuki"), "Hello, Yuki".to_string());
    assert_eq!(
        hello!("Yuki", "Daniel", "Indra"),
        "Hello, Yuki and Daniel and Indra".to_string()
    );
}
```

Our new rule looks a bit like the previous one, but now there's a comma after `$name:literal` and then a repeat pattern.

The repeat pattern contains a metavariable, `$rest:literal`, which will be used to store all metavariables passed to
the macro after the first. It uses a `+` to show that there must be at least one additional metavariable, but there may
be many. There's one more quirk here though, the `,` that would separate the metavariables is outside the repeat
brackets but before the `+`. With repeats you _can_ specify seperators this way, but it only works for `+`. We'll come
back to this.

In the body of the macro, we initialise our output in much the same way as we do in the version with no inputs, by
calling the hello macro with the first metavariable. We then have another repeat pattern that contains the `$rest`
metavariable. Because we have a repeated metavariable inside a repeated block, this block will be repeated for every
`literal` that `$rest` matched to.

If we were to unwrap the code generated for the final test, it would look something like this:

```rust
    assert_eq!(
    {
        let mut output = String::from("Hello, ");
        output.push_str("Yuki");
        output.push_str(" and ");
        output.push_str("Daniel");
        output.push_str(" and ");
        output.push_str("Indra");
        output
    },
    "Hello, Yuki and Daniel and Indra".to_string()
);
```

Hopefully you're probably starting to see why writing a quick macro can really cut down on repeated boilerplate code,
and we're really only making a quick toy macro to demonstrate the power they provide!

You might be wondering if we can use repeats to reduce the number of arms we have. We unfortunately can't do things
like treat the first or last element of a repeat differently using macro repeats *cough*foreshadowing*cough* but we
can merge the second and third arms using a `*`.

```rust
macro_rules! hello {
    () => { hello!("world") };
    ($name:literal $(, $other:literal)*) => {
        {
            let mut output = String::from("Hello, ");
            output.push_str($name);
            $(
                output.push_str(" and ");
                output.push_str($other);
            )*;
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

You'll notice that the `,` after `$name:literal` has moved inside the repeat pattern, and the `,` being used as a
separator has been dropped. This is because the `*` repeat pattern doesn't support seperators, but we can simply say
that the repeating pattern starts with a `,`.

Ok, so I wasn't quite lying about not being able to treat the first and last differently with macro repeats, we can't
do it with _just_ macro repeats, BUT, we can work around that with very low-cost language features like slices.

```rust
macro_rules! hello {
    ($($names:literal),*) => {
        {
            // We split the names out directly into an array. This is done at
            // compile time so doesn't require any heap allocations
            let names = [$($names, )*];

            // We get an iterator over the array. By precisely specifying the
            // type of the iterator here we can avoid Rust not knowing what to
            // do if the iterator is empty.
            use std::iter::Peekable;
            use std::slice::Iter;
            let mut names_iter: Peekable<Iter<&str>> = names.iter().peekable();

            // We initialise our string as before.
            let mut output = String::from("Hello, ");
            // If there are no metavariables were passed then the array will be
            // empty so we'll use our default value
            output.push_str(names_iter.next().unwrap_or(&"world"));

            // We'll loop until no more items are in the iterator
            while let Some(next_name) = names_iter.next() {

                // By looking ahead to see if there's more items we can now use
                // gramatically correct seperators
                match names_iter.peek() {
                    Some(_) => output.push_str(", "),
                    None => output.push_str(" and "),
                }

                output.push_str(next_name);

            };

            // Finally we'll add an exclamation mark for funsies!
            output.push_str("!");
            output
        }
    }
}

fn main() {
    // Note, we've update our tests with the new and improved output!
    assert_eq!(hello!(), "Hello, world!".to_string());
    assert_eq!(hello!("Yuki"), "Hello, Yuki!".to_string());
    assert_eq!(
        hello!("Yuki", "Daniel", "Indra"),
        "Hello, Yuki, Daniel and Indra!".to_string()
    );
}
```

Being able to quickly compose macros like this can save us a lot of time when repeating the same code over and over.

Fragment-Specifiers
-------------------

The way Rust macros work is that they view what you write inside them through the eyes of the Abstract Syntax Tree. This
means that what it sees aren't words or characters but nodes within a structure with semantic meaning. Metavariables 
then can be typed based on how Rust view the thing you passed it, and a single metavariable could be viewed in a number
of ways.

For example, the smallest discrete item in an AST is a Token though we normally don't think about individual tokens but
token trees (`tt`). Parts of a token tree might form an expression (`expr`), basically any token tree that forms a
value, is an expression. An expression might assign its value to a variable using more tokens, and that variable is both
a token and an `ident`. The span of tokens that assign the value of the expression to the ident forms a statement that
is not an expression, even though all expressions are also statements.

```rust
/* rust */      let  hello    =   String  ::  from (  "Hello"   )  ;
// token tree:  |tt| | tt|   |tt| | tt | |tt| |tt| |    tt      | |tt|
//                                                   |  tt   |
// ident:           |ident|
// literals:                                         |literal|
// expressions:                   |             expr             |
//                                                   | expr  |
// statements:  |                       stmt                       |
//                                |             stmt             |
//                                                   | stmt  |
```

Usefully DRY
------------

> ℹ️ I've slightly altered the code in this section to not rely on third party crates, such as
> [Uuid](https://crates.io/crates/uuid) and [paste](https://crates.io/crates/paste). If you're comfortable with crates
> then bellow is a permalink straight to the `storage` crate of the Job Application repository where you'll find the
> real examples. For example, if you look in the `storable` module, you'll find test macros defined in the `property`
> module which are consumed in the `object` module.
>
> [https://github.com/Fios-Quest/job-tracker/tree/c1eba63311ff954de0d80cdd9f55984051c620ef/storage/src/](https://github.com/Fios-Quest/job-tracker/tree/c1eba63311ff954de0d80cdd9f55984051c620ef/storage/src/storable)

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
a big believer in unit tests so lets look at how that works with a test using `Role` as an example.:

```rust
# pub trait HasCompany {
    # fn get_company_id(&self) -> u128;
    #
}
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

That's fine, but we're going to be doing this for every item that implements that trait, as well as for every
implementation of every other trait. Every time we add a new storable item we'll have to add tests for its
implementation.

The way I got around this was, first I created a trait allowing me to create test instances of the types I want to test,
then I created macros that use that trait to run the test:

```rust
// This trait exists in a central location
pub trait TestHelper: Sized {
    // Aside: I'm _actually_ using anyhow for Result which is more flexible
    fn new_test() -> Result<Self, String>;
}

// Each test macro sits alongside the trait it creates tests for
macro_rules! test_has_company_id {
    ($test_name:ident, $storable:ident) => {
        #[test]
        fn $test_name() {
            let storable = $storable::new_test()
                .expect("Could not create storable");
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

While this is a _very_ simple example, there are more complex examples in the Job Tracker like the ones that manage the
act of storing and recalling these storable objects.

Domain Specific Languages
-------------------------

Ever wanted to write your own language?

We're going to get a little bit silly here, but Domain Specific Languages (DSLs) can be incredibly useful for
conceptualising code in meaningful ways. For example, JSX is a DSL for writing React.

This:

```jsx
const heading = (
    <h1 className="example">
        Hello, world!
    </h1>
);
```

Is undeniably easier to understand for web developers who are outputting HTML than writing:

```javascript
const heading = React.createElement(
    'h1',
    {className: 'example'},
    'Hello, world!'
);
```

So, I promised silly, lets write our own DSL... a Brain Fudge interpreter.

The programming language Brain Fudge (which is not actually called Brain Fudge) was created by Urban Müller in 1993. The
language is what's known as an "esoteric" language which is, generally, a fully functional language that you would never
actually want to use. Often they're considered jokes, but Brain Fudge actually lets us write real programs with just
eight instructions. This makes it (almost, foreshadowing again) ideal for creating a full DSL with little effort.

The language operates on theoretically infinite array sequential memory initialised to `0`. You start with a pointer
pointing to the first cell in memory and then process instructions that allow you to move the pointer, modify the data
at that point in memory and either output or input data at the current pointer location.

This is what the instructions do:

- `>` increments the pointer position, moving it to the next position in memory
- `<` decrements the pointer position, moving it to the previous position in memory
- `+` increments the value at the current position in memory
- `-` decrements the value at the current position in memory
- `.` outputs the value at the current position in memory
- `,` takes one byte of input and stores it in memory (we won't use this in this example though)
- `[` and `]` contain a loop that repeats the contained code. Each time the loop begins the value at the current
  position is checked, and the loop is then skipped if the value is 0.

That sounds easy enough, right... well, here's Hello World in Brain Fudge.

```text
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```

Don't panic! We can break the instructions down rather easily. Trust that this outputs Hello World (with a new line) and
let's see if we can make it do that.

We're going to use two macros. First let's create a macro that initialises the program.

```rust
macro_rules! brain_fudge {
    ($($token:tt)+) => {
        {
            let mut memory = vec![0u8];
            let mut pointer = 0_usize;
            let mut output: Vec<u8> = Vec::new();

            // todo: breaking up the token tree

            output.into_iter().map(char::from).collect::<String>()
        }
    };
}
```

Let's break it down:

- `$($token:tt)+` is the input to our interpreter. We're using the `tt` fragment-specifier which means that our
  repeating metavariable `$token` represents a token tree. Tokens are any discrete item in a programming language. For
  example, the Rust code `let hello = String::from("Hello, world!");` can be represented as a token tree like this:
  ![Token Tree example](macros/TokenTreeLight.svg)
  As it happens `>`, `<`, `+`, `-`, `.`, `,`, `[` and `]` are all tokens in Rust so this should work well... (even more
  foreshadowing).
- `memory` is going to be our programs' memory. We're using a Vec with a single initialised value of `0` under the
  assumption that even the smallest program requires one word of memory. We'll expand the Vec as necessary. Not
  necessarily the most time effective but it'll be ok. For our memory we're using `u8` to represent one word. You can
  use larger words if you like but different programs might function differently depending on what word size is used and
  how overflows are handled (more on that later).
- `pointer` points to the current position in data
- `output` is where we'll store output data from the program. We're using a Vec<u8> here, but actually any type that has
  a method `.push(u8)` will work.  
- At the end of the macro we take the output Vec of `u8`s we've stored in output and collect it into a string by naively
  considering each byte to be a character. Again, this won't be appropriate for every use case which is why utilising
  `Write` might be better but do you _really_ want to use this DSL properly 😅

So now we need to handle the token stream, but before we do that, lets write some tests. We'll keep it simple for now,
while `++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.`
outputs "Hello, world!\n", so does the following, with only 3 instructions of the 8 possible:

```rust,should_panic
# macro_rules! brain_fudge {
#     ($($token:tt)+) => {
#         {
#             let mut memory = vec![0u8];
#             let mut pointer = 0_usize;
#             let mut output: Vec<u8> = Vec::new();
# 
#             // todo: breaking up the token tree
# 
#             output.into_iter().map(char::from).collect::<String>()
#         }
#     };
# }
# 
# fn main() {
assert_eq!(
    brain_fudge!(
        // H
        ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
        // e
        >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
        // l
        >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
        // l
        >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
        // o
        >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
        //
        >++++++++++++++++++++++++++++++++.
        // W
        >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
        // o
        >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
        // r
        >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
        // l
        >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
        // d
        >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
        // !
        >+++++++++++++++++++++++++++++++++.
        // \n
        >++++++++++.
    ),
    "Hello World!\n"
);
# }
```

So lets work out how to handle `>`, `+` and `.`

We'll create a new helper macro that can handle these tokens by having an arm that matches a token string that starts
with the token we want to handle and passes remaining tokens back to itself. We also need a special arm to handle when
there are no tokens left so we have an endpoint to our recursive calls.

Unlike before, when we create our match arms, we're going to use a semicolon as a separator. The reason for this is
that Brain Fudge uses comma's as part of its syntax (even if we're not using it here). This doesn't actually cause a
problem with matching (even if the first character of your Brain Fudge program is a comma, it still matches based on
position relative to the other commas), but we _can_ use semicolons as separators in our macro which aren't part of the
Brain Fudge language, and it _will_ help readability when we get to the final part of this chapter.

```rust,no_run
macro_rules! brain_fudge_helper {
    // This arm matches +, it adds 1 to the value at the current position We'll
    // use wrapping_add to avoid overflows, so in our interpreter, adding 1 to
    // 255 makes 0.
    ($memory:ident; $pointer:ident; $buffer:ident; + $($token:tt)*) => {
        $memory[$pointer] = $memory[$pointer].wrapping_add(1);
        brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
    };
    // This arm matches >, it adds 1 to the pointer position. This time we're
    // using saturating_add for the specific reason we want to be consistent
    // and don't want to wrap a  usize on -, you'll see why later!
    // We also need to make sure that any time we go outside of the Vec we
    // resize the Vec appropriately and zero memory, we can do this with a
    // quick loop, pushing 0's
    ($memory:ident; $pointer:ident; $buffer:ident; > $($token:tt)*) => {
        $pointer = $pointer.saturating_add(1);
        while $pointer >= $memory.len() {
            $memory.push(0);
        }
        brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
    };
    // This arm matches ., it takes the value at the current pointer and writes
    // it to our output buffer
    ($memory:ident; $pointer:ident; $buffer:ident; . $($token:tt)*) => {
        $buffer.push($memory[$pointer]);
        brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
    };
    // This arm matches there being no Brain Fudge tokens left, it does nothing
    ($memory:ident; $pointer:ident; $buffer:ident; ) => {};
}
```

And update our brain_fudge! macro to call the helper, passing in the program state.

```rust,compile_fail
# macro_rules! brain_fudge_helper {
#     ($memory:ident; $pointer:ident; $buffer:ident; + $($token:tt)*) => {
#         $memory[$pointer] = $memory[$pointer].wrapping_add(1);
#         brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $buffer:ident; > $($token:tt)*) => {
#         $pointer = $pointer.wrapping_add(1);
#         while $pointer >= $memory.len() {
#             $memory.push(0);
#         }
#         brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $buffer:ident; . $($token:tt)*) => {
#         $buffer.push($memory[$pointer]);
#         brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $buffer:ident; ) => {};
# }
# 
macro_rules! brain_fudge {
    ($($token:tt)+) => {
        {
            let mut data = vec![0u8];
            let mut pointer = 0_usize;
            let mut output: Vec<u8> = Vec::new();

            // We update our brain_fudge macro to pass the program state to the
            // helper
            brain_fudge_helper!(data; pointer; output; $($token)+);
            
            output.into_iter().map(char::from).collect::<String>()
        }
    };
}

# fn main() {
assert_eq!(
    brain_fudge!(
      // You know what's hidden here 😅
#         // H
#         ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // e
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // l
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // l
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // o
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         //
#         >++++++++++++++++++++++++++++++++.
#         // W
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // o
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // r
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // l
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // d
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // !
#         >+++++++++++++++++++++++++++++++++.
#         // \n
#         >++++++++++.
    ),
    "Hello World!\n"
);
# }
```

Aaaand, it errors.

```text
error: recursion limit reached while expanding `brain_fudge_helper!`
```

Rust keeps track of how many times we recurse (call a function/macro from the same function/macro), and by default, the
maximum amount of times we can do this is 128. Our macro, using our silly Hello World example, recurses 1120 times!

So, we _could_ avoid recursing by looping through the tokens instead, and that will work for our Hello World... but it
won't work for loops when we come to do that so for now, we're going to play a dangerous game and manually tell Rust
it's fine for it to recurse 2048 times.

The `recursion_limit` attribute applies at the crate level so be careful with this one!

```rust
#![recursion_limit = "2048"]

macro_rules! brain_fudge_helper {
    // ... snip ...
#     ($memory:ident; $pointer:ident; $buffer:ident; + $($token:tt)*) => {
#         $memory[$pointer] = $memory[$pointer].wrapping_add(1);
#         brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $buffer:ident; > $($token:tt)*) => {
#         $pointer = $pointer.wrapping_add(1);
#         while $pointer >= $memory.len() {
#             $memory.push(0);
#         }
#         brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $buffer:ident; . $($token:tt)*) => {
#         $buffer.push($memory[$pointer]);
#         brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $buffer:ident; ) => {};
}

macro_rules! brain_fudge {
    // ... snip ...
#     ($($token:tt)+) => {
#         {
#             let mut data = vec![0u8];
#             let mut pointer = 0_usize;
#             let mut output: Vec<u8> = Vec::new();
#             
#             brain_fudge_helper!(data; pointer; output; $($token)+);
#             
#             output.into_iter().map(char::from).collect::<String>()
#         }
#     };
# }
# 
# fn main() {
    assert_eq!(
        brain_fudge!(
      // You know what's hidden here 😅
#         // H
#         ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // e
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // l
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // l
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // o
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         //
#         >++++++++++++++++++++++++++++++++.
#         // W
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // o
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // r
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // l
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // d
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // !
#         >+++++++++++++++++++++++++++++++++.
#         // \n
#         >++++++++++.
    ),
        "Hello World!\n"
    );
# }
```

Huzzah! We've made a good start. Dealing with `>` and `-` will be easy enough, they're the opposite of what we already
have. More complex is the loop `[`...`]`. Luckily, we aren't dealing with characters, we're dealing with token trees!

In Rust, the bracket pairs `()`, `[]`, and `{}` are all considered tokens that wrap other tokens, so Rust will correctly
handle them in pairs, even when nested. Eg, with the token tree `[+[-]]` Rust will correctly match the first `[` token
with the final `]` rather than the first `]`
with the final `]` rather than the first `]`.

This means to make our loop arm work, we can match against any token tree that starts with a `[`, contains more tokens
which may include more `[]` pairs, matches its ending `]` and is followed by yet more tokens! How cool is that!?

Lets write up the missing arms and run our test against the original Hello World program:

```rust,compile_fail
#![recursion_limit = "2048"]

macro_rules! brain_fudge_helper {
    // Like + but does a wrapping_sub instead 
    ($memory:ident; $pointer:ident; $buffer:ident; - $($token:tt)*) => {
        $memory[$pointer] = $memory[$pointer].wrapping_sub(1);
        brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
    };
    // Like < but does a saturating_sub instead. This is why saturating is
    // potentially better here as we don't want to wrap and have fill a Vec with
    // something like 18,446,744,073,709,551,615 zeros
    ($memory:ident; $pointer:ident; $buffer:ident; < $($token:tt)*) => {
        $pointer = $pointer.saturating_sub(1);
        brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
    };
    // And here's the magic! We match against $loop_statement tokens inside
    // a square bracket pair potentially followed by more tokens. We then loop
    // while the data at the pointer isn't 0, and once it is, move on to the
    // rest of the tokens
    ($memory:ident; $pointer:ident; $buffer:ident; [$($loop_statement:tt)+] $($token:tt)*) => {
        while $memory[$pointer] != 0 {
            brain_fudge_helper!($memory; $pointer; $buffer; $($loop_statement)+);
        }
        brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
    };
    // ... Snip previous arms ...
#     ($memory:ident; $pointer:ident; $buffer:ident; + $($token:tt)*) => {
#         $memory[$pointer] = $memory[$pointer].wrapping_add(1);
#         brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $buffer:ident; > $($token:tt)*) => {
#         $pointer = $pointer.saturating_add(1);
#         while $pointer >= $memory.len() {
#             $memory.push(0);
#         }
#         brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $buffer:ident; . $($token:tt)*) => {
#         $buffer.push($memory[$pointer]);
#         brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $buffer:ident; ) => {};
}
 
macro_rules! brain_fudge {
    // ... Snip ...
#     ($($token:tt)+) => {
#         {
#             let mut data = vec![0u8];
#             let mut pointer = 0_usize;
#             let mut output: Vec<u8> = Vec::new();
# 
#             // We update our brain_fudge macro to pass the program state to the
#             // helper
#             brain_fudge_helper!(data; pointer; output; $($token)+);
#             
#             output.into_iter().map(char::from).collect::<String>()
#         }
#     };
}

# fn main() {
assert_eq!(
    brain_fudge!(
        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
    ),
    "Hello World!\n"
);
# // keeping the old test to make sure we don't have a regression
# assert_eq!(
#     brain_fudge!(
#         // H
#         ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // e
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // l
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // l
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // o
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         //
#         >++++++++++++++++++++++++++++++++.
#         // W
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // o
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // r
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // l
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // d
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // !
#         >+++++++++++++++++++++++++++++++++.
#         // \n
#         >++++++++++.
#     ),
#     "Hello World!\n"
);
# }
```

And when we run this... it doesn't work again 🤦🏻‍♂️

The exact error we get is:

```text
67 |         ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
   |                                                          ^^ no rules expected this token in macro call
   |
```

Why is it pointing at `>>`? We have a match on `>`.

Well here's the problem with using tokens for our DSL. Rust considers `>>` to be a single token. Specifically it's a
"right shift" operator. Tokens in Rust can be multiple characters. Here are our problem tokens and they mean in each
language:

| token | Brain Fudge                        | Rust                         |
|-------|------------------------------------|------------------------------|
| `..`  | output the current value twice     | range literal                |
| `>>`  | increment pointer twice            | right shift                  |
| `<<`  | decrement pointer twice            | left shift                   |
| `->`  | decrement value, increment pointer | function/closure return type |
| `<-`  | decrement pointer, decrement value | unused but reserved          |

Soooo... we need to take care of these special cases, unfortunately. Luckily, while `>>` is a right shift token, `> >`
_is_ two greater than tokens. Tokens can be seperated by whitespace and will still match the `tt` fragment-specifier,
all we need to do is split the token and pass it back into the macro

```rust
#![recursion_limit = "2048"]

macro_rules! brain_fudge {
    // snip
#     ($($token:tt)+) => {
#         {
#             let mut data = vec![0u8];
#             let mut pointer = 0_usize;
#             let mut output = Vec::new();
#             
#             brain_fudge_helper!(data; pointer; output; $($token)+);
#             
#             output.into_iter().map(char::from).collect::<String>()
#         }
#     };
}

macro_rules! brain_fudge_helper {
    // ... Snip existing tokens ...
#     ($memory:ident; $pointer:ident; $buffer:ident; + $($token:tt)*) => {
#         $memory[$pointer] = $memory[$pointer].wrapping_add(1);
#         brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $buffer:ident; - $($token:tt)*) => {
#         $memory[$pointer] = $memory[$pointer].wrapping_sub(1);
#         brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $buffer:ident; > $($token:tt)*) => {
#         $pointer = $pointer.saturating_add(1);
#         while $pointer >= $memory.len() {
#             $memory.push(0);
#         }
#         brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $buffer:ident; < $($token:tt)*) => {
#         $pointer = $pointer.saturating_sub(1);
#         brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $buffer:ident; . $($token:tt)*) => {
#         $buffer.push($memory[$pointer]);
#         brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $buffer:ident; [$($loop_statement:tt)+] $($token:tt)*) => {
#         while $memory[$pointer] != 0 {
#             brain_fudge_helper!($memory; $pointer; $buffer; $($loop_statement)+);
#         }
#         brain_fudge_helper!($memory; $pointer; $buffer; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $buffer:ident; ) => {};

    // Special "token" cases
    ($memory:ident; $pointer:ident; $buffer:ident; >> $($token:tt)*) => {
        brain_fudge_helper!($memory; $pointer; $buffer; > > $($token)*);
    };
    ($memory:ident; $pointer:ident; $buffer:ident; << $($token:tt)*) => {
        brain_fudge_helper!($memory; $pointer; $buffer; < < $($token)*);
    };
    ($memory:ident; $pointer:ident; $buffer:ident; .. $($token:tt)*) => {
        brain_fudge_helper!($memory; $pointer; $buffer; . . $($token)*);
    };
    ($memory:ident; $pointer:ident; $buffer:ident; <- $($token:tt)*) => {
        brain_fudge_helper!($memory; $pointer; $buffer; < - $($token)*);
    };
    ($memory:ident; $pointer:ident; $buffer:ident; -> $($token:tt)*) => {
        brain_fudge_helper!($memory; $pointer; $buffer; - > $($token)*);
    };
}

# fn main() {
assert_eq!(
    brain_fudge!(++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.),
    "Hello World!\n"
);
# // keeping the old test to make sure we don't have a regression
# assert_eq!(
#     brain_fudge!(
#         // H
#         ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // e
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // l
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // l
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // o
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         //
#         >++++++++++++++++++++++++++++++++.
#         // W
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // o
#         >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // r
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // l
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // d
#         >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
#         // !
#         >+++++++++++++++++++++++++++++++++.
#         // \n
#         >++++++++++.
#     ),
#     "Hello World!\n"
# );
# }
```

And we just created an interpreter for another language inside Rust! That's kind of wild, right?!

Homework
--------

I've stopped setting homework, but I thought I'd set a little challenge for anyone who wants to do it.

Can you edit our `brain_fudge!` macro to work with programs that take input via the `,` token. To do this I recommend
making the following change to the main macro, assuming types with Read for `$input` and Write for `$output`:

```rust
macro_rules! brain_fudge {
     ($input:ident, $output:ident, $($token:tt)+) => {
        {
          // That's all you get!
        }
     };
}
```

If you need help, the code below shows the test for a ROT13 Brain Fudge program and has the answer to the homework
hidden if you want to reveal it

```rust
# #![recursion_limit = "2048"]
# 
# macro_rules! brain_fudge {
#     ($input:ident; $output:ident; $($token:tt)+) => {
#         {
#             use std::io::{Read, Write};
#             
#             let mut memory = vec![0u8];
#             let mut pointer = 0_usize;
# 
#             brain_fudge_helper!(memory; pointer; $input; $output; $($token)+);
#         }
#     };
# }
# 
# macro_rules! brain_fudge_helper {
#     ($memory:ident; $pointer:ident; $input:ident; $output:ident; + $($token:tt)*) => {
#         $memory[$pointer] = $memory[$pointer].wrapping_add(1);
#         brain_fudge_helper!($memory; $pointer; $input; $output; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $input:ident; $output:ident; - $($token:tt)*) => {
#         $memory[$pointer] = $memory[$pointer].wrapping_sub(1);
#         brain_fudge_helper!($memory; $pointer; $input; $output; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $input:ident; $output:ident; > $($token:tt)*) => {
#         $pointer = $pointer.saturating_add(1);
#         while $pointer >= $memory.len() {
#             $memory.push(0);
#         }
#         brain_fudge_helper!($memory; $pointer; $input; $output; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $input:ident; $output:ident; < $($token:tt)*) => {
#         $pointer = $pointer.saturating_sub(1);
#         brain_fudge_helper!($memory; $pointer; $input; $output; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $input:ident; $output:ident; . $($token:tt)*) => {
#         $output.push($memory[$pointer]);
#         brain_fudge_helper!($memory; $pointer; $input; $output; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $input:ident; $output:ident; , $($token:tt)*) => {
#         $memory[$pointer] = $input.next().unwrap_or(0);
#         brain_fudge_helper!($memory; $pointer; $input; $output; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $input:ident; $output:ident; [$($loop_statement:tt)+] $($token:tt)*) => {
#         while $memory[$pointer] != 0 {
#             brain_fudge_helper!($memory; $pointer; $input; $output; $($loop_statement)+);
#         }
#         brain_fudge_helper!($memory; $pointer; $input; $output; $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $input:ident; $output:ident; ) => {};
#     // Special "token" cases
#     ($memory:ident; $pointer:ident; $input:ident; $output:ident; >> $($token:tt)*) => {
#         brain_fudge_helper!($memory; $pointer; $input; $output; > > $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $input:ident; $output:ident; << $($token:tt)*) => {
#         brain_fudge_helper!($memory; $pointer; $input; $output; < < $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $input:ident; $output:ident; .. $($token:tt)*) => {
#         brain_fudge_helper!($memory; $pointer; $input; $output; . . $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $input:ident; $output:ident; <- $($token:tt)*) => {
#         brain_fudge_helper!($memory; $pointer; $input; $output; < - $($token)*);
#     };
#     ($memory:ident; $pointer:ident; $input:ident; $output:ident; -> $($token:tt)*) => {
#         brain_fudge_helper!($memory; $pointer; $input; $output; - > $($token)*);
#     };
# }
# 
# fn main() {
    let input_string = String::from("FiosQuest");
    let mut input = input_string.bytes();
    let mut output = Vec::new();
    brain_fudge!(
      input; 
      output;
      ,[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>++++++++++++++<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>>+++++[<----->-]<<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>++++++++++++++<-[>+<-[>+<-[>+<-[>+<-[>+<-[>++++++++++++++<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>>+++++[<----->-]<<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>++++++++++++++<-[>+<-]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]>.[-]<,]
    );
    let output_string: String = output.into_iter().map(char::from).collect();
    assert_eq!(&output_string, "SvbfDhrfg");
    println!("{}", output_string);
# }
```

