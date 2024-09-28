Introduction to Traits
======================

In the last chapter we created a state machine for our Cat, but we were left with several problems.

1. We couldn't access anything about the Cat from inside our State.
2. The behaviours didn't seem generally applicable. Would `Hangry<Human>` make loud noises and bite someone? Mostly,
   probably not.

Traits can help us solve those problems.

> Note: This chapter uses code from the previous chapter, make sure you have the code from that chapter ready to go.

Example Trait: `ToString`
-------------------------

Traits describe common behaviour between types that implement (`impl`) the trait. For example, have you noticed that 
lots of types have a method called `to_string()`, including numbers, string slices (`&str`) and even strings? This is
because there is a trait called `ToString` that describes the function header for a method called `to_string()` and all
of these types implement that trait.

This is what ToString looks like in the Rust standard library (sans comments and annotations):

```rust
pub trait ToString {
   fn to_string(&self) -> String;
}
```

Any type can implement this trait to provide the `to_string()` method.

We can use the `ToString` trait to create a generic function where we accept data of some type that could be literally
anything, and in the list of generic parameters we use a "Trait Bound" to restrict the types that can be used to only
those that implement the `ToString` trait.

In the example below, we use the generic `S` but we use "bounding" to say that whatever `S` is, it _must_ implement
`ToString`. We can then be sure that whatever goes into our generic function it _must_ have the `to_string()` method, so
it's safe to rely on it being there. If it doesn't implement `ToString` you'll get a compiler error (this should show
up in your IDE before you get as far as compiling though). AS it happens, a _lot_ of built-in types already implement
`ToString`.

```rust
fn say_hello<S: ToString>(could_be_anything: S) {
    println!("Hello {}!", could_be_anything.to_string());
}

fn main() {
    say_hello("Yuki");               // &str
    say_hello(String::from("Yuki")); // String
    say_hello(10u8);                 // u8 
    // say_hello(Vec::new());        // Vec doesn't impl ToString, this won't compile 
}
```

We can also implement `ToString` on our own types. Imagine we have a *\*cough\**
[poorly designed](https://www.kalzumeus.com/2010/06/17/falsehoods-programmers-believe-about-names/) Person type with a
first and last name. We can implement `ToString` to turn the user into a string which combines their name. You can
run this example to see that it works with our previous function

```rust
struct Person {
   first: String,
   last: String,
}

impl ToString for Person {
   fn to_string(&self) -> String {
      // Here we use the format macro to create a combined string from the first
      // and last names. This works almost identically to the various `println!`
      // macros but creates a String on the heap and returns it
      format!("{} {}", &self.first, &self.last)
   }
}
# 
# fn say_hello<S: ToString>(could_be_anything: S) {
#     println!("Hello {}!", could_be_anything.to_string());
# }
# 
# fn main() {
#     let daniel = Person { first: "Daniel".to_string(), last: "Mason".to_string() };
#     say_hello(daniel); 
# }
```

> ⚠️ **Important:** You actually _shouldn't_ implement `ToString`. I use it here because it's very slightly easier to
> understand that what you _should_ do, which is implement the trait `Display`. We'll cover this at the end of the 
> chapter when the reason why is easier to understand.

It's worth noting that in order to use methods associated with a trait, the trait must be in scope. We don't have to do
this ourselves because `ToString` is part of the [Rust prelude](https://doc.rust-lang.org/std/prelude/), a collection
of types and traits that are always available in Rust. Often when people create libraries they'll make their own prelude
module that contains the most commonly used types and traits so that you can import the entire prelude module (eg
`use rayon::prelude`, which we'll talk more about in the ecosystem section of the book) rather than having to import a
lot of items individually.

`ToString` is one of many traits that are built into the Rust standard library, and we'll talk more about some of the
other traits available to you in the future. For now though, we're going to build our own!

`Animal`s
---------

Let's start by tackling the first problem, not having access to the `Cat`'s data inside the States. 

We're going to make an `Animal` trait to represent the behaviour of any animal.

We'll also do a little reorganising while we're at it.

The idea here is that all animals will implement the `Animal` trait, then we'll have some known behaviour.

First lets create an animal module. In `main.rs` add `mod animal` and then create the file `animal/mod.rs`.

Let's move `cat.rs` to `animal/cat.rs` so that it's a submodule of `animal`. Finally, don't forget to add `pub mod cat;`
to `animal/mod.rs` and to update your use statement in `main.rs` to `animal::cat::Cat`.

We're now ready to make our trait.

In `animal/mod.rs`, underneath `pub mod cat;`, let our new `Animal` trait:

```rust,no_run
// File: animal/mod.rs
pub trait Animal {
    fn get_name(&self) -> &str;
}
```

With trait methods, we don't _have_ to define any behaviour (though we can), we only need to tell Rust how the method
will be used. In this case we define a method called `get_name` which will take a reference to the data this is
implemented for, and will return a string slice. We also don't need to specify that the method is public as Traits are
Rust's equivalent of Interfaces, everything listed is assumed to be public.

So, let's implement this for `Cat`.

In `cat.rs` we'll add the implementation. As with implementations for types we start with `impl <TRAIT_NAME>` but with
traits we follow it up with `for <TYPE>`. So our impl block should look like this:

```rust,no_run
# // Prevent mdbook wrapping everything in a main function
# fn main() {}
#
# // This should be in your mod/animal.rs
# trait Animal {
#     fn get_name(&self) -> &str;
# }
#
# mod cat {
use super::Animal;
# 
# pub struct Cat {
#     name: String,
# }
#
# impl Cat {
#     pub fn new(name: String) -> Self { // ...
#         Self { name }
#     }
# 
#     pub fn get_name(&self) -> &str {
#         &self.name
#     }
# }
#

impl Animal for Cat {
    fn get_name(&self) -> &str {
        &self.name
    }
}
# }
```

You might have noticed that we now have _two_ methods for Cat called `get_name()`, one in `impl Cat`, and one in
`impl Animal for Cat`. That's actually ok, but is indicative of a code smell. What happens if we want to add more
functionality to the getter? We'd have to remember to update both. It'd be better to call the underlying
`Cat::get_name` from `Animal::get_name`, but how do we do that?

Have you noticed that when calling methods with the dot syntax, eg, `yuki.get_name()`, even though the methods first
argument is `&self` (or similar), we don't actually pass anything in here, this argument is skipped when calling. This
is because when we call a method with the dot syntax, we call it on a specific instance, so Rust, like many similar 
languages, can infer the value of `self` (or `this` in some languages) to be the instance the method was called on.

We can also call the method directly and manually pass in the value of `self`. For example, in the method
`Animal::get_name` we could call the `Cat` method of the same name, manually passing in `self`. This lets Rust know that
it should call the `Cat` implementation of `get_name`. Now the behaviour of `Animal::get_name` for `Cat` will always be
the same as `Cat::get_name` even if we change the later method in the future.

```rust,no_run
# // Prevent mdbook wrapping everything in a main function
# fn main() {}
#
# // This should be in your mod/animal.rs
# trait Animal {
#     fn get_name(&self) -> &str;
# }
#
# mod cat {
use super::Animal;
# 
# pub struct Cat {
#     name: String,
# }
#
# impl Cat {
#     pub fn new(name: String) -> Self { // ...
#         Self { name }
#     }
# 
#     pub fn get_name(&self) -> &str {
#         &self.name
#     }
# }
#

impl Animal for Cat {
    fn get_name(&self) -> &str {
        Cat::get_name(self)
    }
}
# }
```
For each state (`Mischievous`, `Hangry`, `Eepy`), we can add a Trait Bound so that the generic `A` must be a type that
has implemented the `Animal` trait. We can do this in the generics list as we did before. For example, `Mischievous` 
would look like this:

```rust,no_run
# fn main() {}
# trait Animal {
#     fn get_name(&self) -> &str;
# }
pub struct Mischievous<A: Animal> {
    animal: A,
}
```

Update all of you other states (`Hangry`, and `Eepy`) to match.

Now that we know that whatever is in each state's `animal` field must implement the `Animal` trait, we can treat it as
such in any implementation code for those states. Just remember that for generic `impl`s, it is the `impl` that
specifies the generic, so we need to make sure we add the Trait Bound there, then we can update our describe to use the
trait (here I've used the `format!` macro which is like `println!` but produces a `String`):

```rust,no_run
# fn main() {}
# trait Animal {
#     fn get_name(&self) -> &str;
# }
pub struct Mischievous<A: Animal> {
    animal: A,
}

impl<A: Animal> Mischievous<A> {
    // Other methods ...

    pub fn describe(&self) -> String {
        format!(
            "{} is trying to break into a wardrobe by pulling on exposed clothing",
            self.animal.get_name()
        )
    }
}
```

Update all of your States to use `self.animal.get_name()` and, assuming your `main.rs` still looks like the below, you
should get your output with your cats name!

```rust
# pub mod animal {
#    // animal/mod.rs
#    pub trait Animal {
#       fn get_name(&self) -> &str;
#    }
# 
#    pub mod cat {
#       // animal/cat.rs
#       use crate::state::mischievous::Mischievous;
# 
#       use super::Animal;
# 
#       pub struct Cat {
#          name: String,
#       }
# 
#       impl Cat {
#          pub fn new(name: String) -> Mischievous<Self> {
#             Mischievous::new(Self { name })
#          }
# 
#          pub fn get_name(&self) -> &str {
#             &self.name
#          }
#       }
# 
#       impl Animal for Cat {
#          fn get_name(&self) -> &str {
#             Cat::get_name(self)
#          }
#       }
#    }
# }
# 
# pub mod state {
#    pub mod eepy {
#       // state/eepy.rs
#       use crate::animal::Animal;
# 
#       use super::mischievous::Mischievous;
# 
#       pub struct Eepy<A: Animal> {
#          animal: A,
#       }
# 
#       impl<A: Animal> Eepy<A> {
#          pub fn new(animal: A) -> Self {
#             Eepy { animal }
#          }
# 
#          pub fn sleep(self) -> Mischievous<A> {
#             Mischievous::new(self.animal)
#          }
# 
#          pub fn describe(&self) -> String {
#             format!(
#                "Look at the precious baby {} sleeping 😍",
#                &self.animal.get_name()
#             )
#          }
#       }
# 
#    }
#    
#    pub mod hangry {
#       // state/hangry.rs
#       use crate::animal::Animal;
# 
#       use super::eepy::Eepy;
# 
#       pub struct Hangry<A: Animal> {
#          animal: A,
#       }
# 
#       impl<A: Animal> Hangry<A> {
#          pub fn new(animal: A) -> Self {
#             Hangry { animal }
#          }
# 
#          pub fn feed(self) -> Eepy<A> {
#             Eepy::new(self.animal)
#          }
# 
#          pub fn describe(&self) -> String {
#             format!(
#                "Being loud doesn't work, {} chooses violence and attacks!",
#                &self.animal.get_name()
#             )
#          }
#       }
# 
#    }
#    pub mod mischievous {
#       // state/mischievous.rs
#       use crate::animal::Animal;
# 
#       use super::hangry::Hangry;
# 
#       pub struct Mischievous<A: Animal> {
#          animal: A,
#       }
# 
#       impl<A: Animal> Mischievous<A> {
#          pub fn new(animal: A) -> Self {
#             Mischievous { animal }
#          }
# 
#          pub fn forget_to_feed(self) -> Hangry<A> {
#             Hangry::new(self.animal)
#          }
# 
#          pub fn describe(&self) -> String {
#             format!(
#                "{} is trying to break into a wardrobe by pulling on exposed clothing",
#                self.animal.get_name()
#             )
#          }
#       }
# 
#    }
# }
# 
// main.rs
use animal::cat::Cat;

fn main() {
  let mischievous_yuki = Cat::new("Yuki".to_string());
  println!("{}", mischievous_yuki.describe());
  println!();
  
  let hangry_yuki = mischievous_yuki.forget_to_feed();
  println!("{}", hangry_yuki.describe());
  println!();
  
  let sleepy_yuki = hangry_yuki.feed();
  println!("{}", sleepy_yuki.describe());
  println!();
  
  let mischievous_yuki = sleepy_yuki.sleep();
  println!("{}", mischievous_yuki.describe());
  println!();
}
```

So that's our first problem solved! We can now access the `Cat`'s data through the `Animal` trait.

Making more flexible `Animal`s
------------------------------

Now that we can read details from the underlying `Cat` object, lets start to think about how we can expand this 
functionality out to other types of animals... starting with the most dangerous of animal.

Start by adding `pub mod human;` to `animal.mod`.

Then create `animal/human.rs` and pop this inside:

```rust
# // Prevent mdbook wrapping everything in a main function
# fn main() {}
# pub mod animal {
#     // animal/mod.rs
#     pub trait Animal {
#         fn get_name(&self) -> &str;
#     }
# }  
# pub mod state {
#     pub mod mischievous {
#     // state/mischievous.rs
#         use crate::animal::Animal;
#
#         pub struct Mischievous<A: Animal> {
#             animal: A,
#         }
#
#         impl<A: Animal> Mischievous<A> {
#             pub fn new(animal: A) -> Self {
#                 Mischievous { animal }
#             }
#         }
#     }
# }
#
// File: animal/human.rs
use animal::Animal;
use state::mischievous::Mischievous;

pub struct Human {
    name: String
}

impl Human {
    pub fn new(name: String) -> Mischievous<Self> {
       Mischievous::new(Self { name })
    }
}

impl Animal for Human {
    fn get_name(&self) -> &str {
       &self.name
    }
}
```

Your `animal/mod.rs` need to expose both of its submodules publicly.

```rust,ignore
// File: animal/mod.rs

pub mod cat;
pub mod human;
```

Finally, lets update our main function, and run the program to make sure everything is working.

```rust
# pub mod animal {
#    // animal/mod.rs
#    pub trait Animal {
#       fn get_name(&self) -> &str;
#    }
# 
#    pub mod cat {
#       // animal/cat.rs
#       use crate::state::mischievous::Mischievous;
# 
#       use super::Animal;
# 
#       pub struct Cat {
#          name: String,
#       }
# 
#       impl Cat {
#          pub fn new(name: String) -> Mischievous<Self> {
#             Mischievous::new(Self { name })
#          }
# 
#          pub fn get_name(&self) -> &str {
#             &self.name
#          }
#       }
# 
#       impl Animal for Cat {
#          fn get_name(&self) -> &str {
#             Cat::get_name(self)
#          }
#       }
#    }
# 
#    pub mod human {
#       // animal/human.rs
#       use crate::state::mischievous::Mischievous;
# 
#       use super::Animal;
# 
#       pub struct Human {
#          name: String,
#       }
# 
#       impl Human {
#          pub fn new(name: String) -> Mischievous<Self> {
#             Mischievous::new(Self { name })
#          }
#       }
# 
#       impl Animal for Human {
#          fn get_name(&self) -> &str {
#             &self.name
#          }
#       }
#    }
# }
# 
# pub mod state {
#    pub mod eepy {
#       // state/eepy.rs
#       use crate::animal::Animal;
# 
#       use super::mischievous::Mischievous;
# 
#       pub struct Eepy<A: Animal> {
#          animal: A,
#       }
# 
#       impl<A: Animal> Eepy<A> {
#          pub fn new(animal: A) -> Self {
#             Eepy { animal }
#          }
# 
#          pub fn sleep(self) -> Mischievous<A> {
#             Mischievous::new(self.animal)
#          }
# 
#          pub fn describe(&self) -> String {
#             format!(
#                "Look at the precious baby {} sleeping 😍",
#                &self.animal.get_name()
#             )
#          }
#       }
# 
#    }
#    
#    pub mod hangry {
#       // state/hangry.rs
#       use crate::animal::Animal;
# 
#       use super::eepy::Eepy;
# 
#       pub struct Hangry<A: Animal> {
#          animal: A,
#       }
# 
#       impl<A: Animal> Hangry<A> {
#          pub fn new(animal: A) -> Self {
#             Hangry { animal }
#          }
# 
#          pub fn feed(self) -> Eepy<A> {
#             Eepy::new(self.animal)
#          }
# 
#          pub fn describe(&self) -> String {
#             format!(
#                "Being loud doesn't work, {} chooses violence and attacks!",
#                &self.animal.get_name()
#             )
#          }
#       }
# 
#    }
#    pub mod mischievous {
#       // state/mischievous.rs
#       use crate::animal::Animal;
# 
#       use super::hangry::Hangry;
# 
#       pub struct Mischievous<A: Animal> {
#          animal: A,
#       }
# 
#       impl<A: Animal> Mischievous<A> {
#          pub fn new(animal: A) -> Self {
#             Mischievous { animal }
#          }
# 
#          pub fn forget_to_feed(self) -> Hangry<A> {
#             Hangry::new(self.animal)
#          }
# 
#          pub fn describe(&self) -> String {
#             format!(
#                "{} is trying to break into a wardrobe by pulling on exposed clothing",
#                self.animal.get_name()
#             )
#          }
#       }
#    }
# }
# 
// main.rs
use animal::cat::Cat;
use animal::human::Human;

fn main() {
    let mischievous_yuki = Cat::new("Yuki".to_string());
    println!("{}", mischievous_yuki.describe());
 
    let mischievous_daniel = Human::new("Daniel".to_string());
    println!("{}", mischievous_daniel.describe());
}
```

Notice that we barely had to change anything to add humans to our code, how cool is that!

But there's still an issue... my mischievous state doesn't tend to have me breaking into wardrobes by pulling on exposed
clothing... I have a opposable thumb.

In fact, when I'm in a mischievous mood, I probably don't behave the same as other humans, I probably don't behave the
same as you when you're feeling mischievous.

Optional Homework
-----------------

Can you change the code so that each states behaviours are defined when the structs are instantiated? To do this you
will need to:
- modify the `Human` and `Cat` structs
- add methods to the `Animal` trait
- and then implement those methods for each struct

If you get stuck, I've implemented the code below, just hit the eye icon. Please note that a limitation of this book
means all the code is in one place, you should split your modules into files so that it's easier to manage and work
with.

```rust
# pub mod animal {
#    // animal/mod.rs
#    pub trait Animal {
#       fn get_name(&self) -> &str;
#       fn get_behaviour_mischievous(&self) -> &str;
#       fn get_behaviour_hangry(&self) -> &str;
#       fn get_behaviour_eepy(&self) -> &str;
#    }
# 
#    pub mod cat {
#       // animal/cat.rs
#       use crate::state::mischievous::Mischievous;
# 
#       use super::Animal;
# 
#       pub struct Cat {
#           name: String,
#           behaviour_mischievous: String,
#           behaviour_hangry: String,
#           behaviour_eepy: String,
#       }
# 
#       impl Cat {
#          pub fn new(
#              name: String,
#              behaviour_mischievous: String,
#              behaviour_hangry: String,
#              behaviour_eepy: String,
#          ) -> Mischievous<Self> {
#              Mischievous::new(Self { 
#                  name,
#                  behaviour_mischievous,
#                  behaviour_hangry,
#                  behaviour_eepy,
#              })
#          }
#       }
# 
#       impl Animal for Cat {
#          fn get_name(&self) -> &str {
#             &self.name
#          }
#
#          fn get_behaviour_mischievous(&self) -> &str {
#             &self.behaviour_mischievous
#          }
#
#          fn get_behaviour_hangry(&self) -> &str {
#             &self.behaviour_hangry
#          }
#
#          fn get_behaviour_eepy(&self) -> &str {
#             &self.behaviour_eepy
#          }
#       }
#    }
# 
#    pub mod human {
#       // animal/human.rs
#       use crate::state::mischievous::Mischievous;
# 
#       use super::Animal;
# 
#       pub struct Human {
#           name: String,
#           behaviour_mischievous: String,
#           behaviour_hangry: String,
#           behaviour_eepy: String,
#       }
# 
#       impl Human {
#           pub fn new(
#               name: String,
#               behaviour_mischievous: String,
#               behaviour_hangry: String,
#               behaviour_eepy: String,
#           ) -> Mischievous<Self> {
#               Mischievous::new(Self {
#                   name,
#                   behaviour_mischievous,
#                   behaviour_hangry,
#                   behaviour_eepy,
#               })
#           }
#       }
# 
#       impl Animal for Human {
#          fn get_name(&self) -> &str {
#              &self.name
#          }
#
#          fn get_behaviour_mischievous(&self) -> &str {
#              &self.behaviour_mischievous
#          }
#
#          fn get_behaviour_hangry(&self) -> &str {
#              &self.behaviour_hangry
#          }
#
#          fn get_behaviour_eepy(&self) -> &str {
#              &self.behaviour_eepy
#          }
#       }
#    }
# }
# 
# pub mod state {
#    pub mod eepy {
#       // state/eepy.rs
#       use crate::animal::Animal;
# 
#       use super::mischievous::Mischievous;
# 
#       pub struct Eepy<A: Animal> {
#           animal: A,
#       }
# 
#       impl<A: Animal> Eepy<A> {
#          pub fn new(animal: A) -> Self {
#              Eepy { animal }
#          }
# 
#          pub fn sleep(self) -> Mischievous<A> {
#              Mischievous::new(self.animal)
#          }
# 
#          pub fn describe(&self) -> String {
#              format!("{} is {}", self.animal.get_name(), self.animal.get_behaviour_eepy())
#          }
#       }
# 
#    }
#    
#    pub mod hangry {
#       // state/hangry.rs
#       use crate::animal::Animal;
# 
#       use super::eepy::Eepy;
# 
#       pub struct Hangry<A: Animal> {
#          animal: A,
#       }
# 
#       impl<A: Animal> Hangry<A> {
#          pub fn new(animal: A) -> Self {
#              Hangry { animal }
#          }
# 
#          pub fn feed(self) -> Eepy<A> {
#              Eepy::new(self.animal)
#          }
# 
#          pub fn describe(&self) -> String {
#              format!("{} is {}", self.animal.get_name(), self.animal.get_behaviour_hangry())
#          }
#       }
# 
#    }
#    pub mod mischievous {
#       // state/mischievous.rs
#       use crate::animal::Animal;
# 
#       use super::hangry::Hangry;
# 
#       pub struct Mischievous<A: Animal> {
#          animal: A,
#       }
# 
#       impl<A: Animal> Mischievous<A> {
#          pub fn new(animal: A) -> Self {
#             Mischievous { animal }
#          }
# 
#          pub fn forget_to_feed(self) -> Hangry<A> {
#             Hangry::new(self.animal)
#          }
# 
#          pub fn describe(&self) -> String {
#              format!("{} is {}", self.animal.get_name(), self.animal.get_behaviour_mischievous())
#          }
#       }
#    }
# }
# 
# // main.rs
# use animal::cat::Cat;
# use animal::human::Human;
# 
# fn main() {
#     let mischievous_yuki = Cat::new(
#         "Yuki".to_string(),
#         "trying to break into a wardrobe by pulling on exposed clothing".to_string(),
#         "being loud, it doesn't work so he chooses violence".to_string(),
#         "half a sleep, look at the precious baby 😻".to_string(),
#     );
#  
#     let mischievous_daniel = Human::new(
#         "Daniel".to_string(),
#         r#"pretending to sneak up on his partner for a hug quietly saying "sneak sneak""#.to_string(),
#         "looking at food delivery apps".to_string(),
#         "watching TV he's seen a million times before to wind down".to_string(),
#     );
# 
#     println!("{}", mischievous_yuki.describe());
#     println!("{}", mischievous_daniel.describe());
#     
#     let hangry_yuki = mischievous_yuki.forget_to_feed();
#     let hangry_daniel = mischievous_daniel.forget_to_feed();
#     println!("{}", hangry_yuki.describe());
#     println!("{}", hangry_daniel.describe());
#
#     let sleepy_yuki = hangry_yuki.feed();
#     let sleepy_daniel = hangry_daniel.feed();
#     println!("{}", sleepy_yuki.describe());
#     println!("{}", sleepy_daniel.describe());
# }
// Run me or look at my code using the hover icons
```

`Display`
---------

As I mentioned earlier, we shouldn't actually implement `ToString`, we should implement `Display`. In fact, none of the
internal types I mentioned (numbers, string slices, strings, etc) implement `ToString` but do in fact implement 
`Display`.

Let's start looking at the trait itself:

```rust
# use std::fmt::{Formatter, Result};
#
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
```

As you can see, its already more complex than `ToString`. It takes an additional parameter of type `Formatter` 
(specifically `std::fmt::Formatter`), and instead of returning a string, it returns a `Result` (specifically 
`std::fmt::Result`).

Luckily, we don't actually have to worry about any of this ourselves as there is a macro called `write!` that deals with
it all for us.

To change our `ToString` implementation for `Person` to `Display`, in addition to changing the trait name and method,
we can simply swap the `format!` macro for `write!` and pass the formatter as the first part of the macro.

```rust
# fn main() {}
#
# struct Person {
#    first: String,
#    last: String,
# }
# 
use std::fmt; // Easier to use the fmt module directly due to `Result` already existing in scope

impl fmt::Display for Person {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{} {}", &self.first, &self.last)
   }
}
```

Hang on though, if none of these types are implementing `ToString`, how did we use them in a function with a `ToString`
trait bound?

Well, they don't implement `ToString` directly, there is a
[generic implementation of `ToString`](https://doc.rust-lang.org/1.81.0/src/alloc/string.rs.html#2555) 
for all types that implement `Display`.

Over simplified (there's still more to the `Display` trait I don't want to cover _yet_, but check the link for the full
code) it looks like this:

```rust,ignore
# use std::fmt;
# 
impl<T: fmt::Display> ToString for T {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        let mut formatter = fmt::Formatter::new(&mut buffer);
        self.fmt(&mut formatter).expect("a Display implementation returned an error unexpectedly");
        buffer
    }
}
```

Having gone through the rest of the chapter this hopefully makes some sense. We're implementing `ToString` for the
generic `T` where `T` already has `Display`. We can then create the string using the display method of that type.

Because those built in types already have `Display`, they get `ToString` for free. Once you've implemented `Display` for
`Person` to, you not only won't need `ToString` any more, you'll find that `ToString` if you leave you're `ToString`
implementation in, you can't compile your code because it now conflicts with this other implementation.

So why do both `Display` and `ToString` exist, especially if everything with `Display` gets a free `ToString`
implementation? The answer might surprise you! ... But it's non-trivial so I'll save it for much further into the book,
however I will give you a hint, it's _something_ to do with memory.

Next Chapter
------------

In the next chapter we'll continue to explore Traits by looking at some of the more commonly used ones available in the
Rust standard library. This will also allow us to cover some Trait features we haven't seen so far, including associated
types!
