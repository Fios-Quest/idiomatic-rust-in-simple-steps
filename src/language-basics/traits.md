Traits
======

In the last chapter we created a state machine for our Cat, but we were left with several problems.

- First, we couldn't access anything about the Cat from inside our State.
- Second, the behaviours didn't seem generally applicable. Would `Hangry<Human>` make loud noises and bite someone?
  Mostly, probably not.

Traits can help us solve those problems.

Traits describe common behaviour between types that implement (impl) the trait. For example, have you noticed that lots
of types have a method called `to_string()`, including numbers, string slices (`&str`) and even strings? This is because
there is a trait called `ToString` that describes the function header for a method called `to_string()` and all of these
types implement that trait.

We can use this knowledge to create a generic function where we accept data of some type that could be literally
anything, and in the list of generic parameters we use a "Trait Bound" to restrict the types that can be used.

In the example below, we use the generic `S` but we use a bound to say that whatever `S` is, it _must_ implement
`ToString`. We can then be sure that whatever goes into our generic function it _must_ have the `to_string()` method, so
it's safe to rely on it being there. If it doesn't implement `ToString` you'll get a compiler error (this should show
up in your IDE before you get as far as compiling though).

```rust
fn say_hello<S: ToString>(could_be_anything: S) {
    println!("Hello {}!", could_be_anything.to_string());
}

fn main() {
    say_hello("Yuki");               // &str
    say_hello("Daniel".to_string()); // String
    say_hello(10u8)                  // u8 
    // say_hello(Vec::new());        // Vec does not impl ToString, so this would not compile 
}
```

Animals
-------

Let's start by tackling the first problem, not having access to the `Cat`'s data inside the States. We're going to make
an `Animal` trait to represent the behaviour of any animal, we're also going to do a little reorganising while we're at
it.

First lets create an animal module. In `main.rs` add `mod animal` and then create the file `animal/mod.rs`.

Let's move `cat.rs` to `animal/cat.rs` so that it's a submodule of `animal`. Finally, don't forget to add `pub mod cat;`
to `animal/mod.rs` and don't forget to update your use statement in `main.rs` to `animal::cat::Cat`.

We're now ready to make our trait.

In `animal/mod.rs`, underneath `pub mod cat;`, let add the following:

```rust,no_run
// File: animal/mod.rs
pub trait Animal {
    fn get_name(&self) -> &str;
}
```

With trait methods, we don't have to define any behaviour (though we can), we only need to tell Rust how the method will
be used. In this case we define a method called `get_name` which will take a reference to the data this is implemented
for, and will return a string slice. We also don't need to specify that the method is public as Traits are Rust's
equivalent of Interfaces, everything listed is assumed to be public.

So, let's implement this for `Cat`.

In `cat.rs` we'll add the implementation. As with implementations for types we start with `impl TRAIT_NAME` but with
traits we follow it up with `for TYPE`. So our impl block should look like this:

```rust,no_run
# // Prevent mdbook wrapping everything in a main function
# fn main() {}
#
# // This should be in mod/animal.rs
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

impl Animal for Cat {
    fn get_name(&self) -> &str {
        &self.name
    }
}
# }
```

You might have noticed that we now have _two_ functions for Cat called `get_name()`, one in `impl Cat`, one in
`impl Animal for Cat`. That's ok, we'll come to that. For now, lets finish off the first task by updating out states.

For each state (`Mischievous`, `Hangry`, `Eepy`), we need to add a Trait Bound so that the generic `A` can only be of
type `Animal`. We can do this in the generics list as we did before. For example, `Mischievous` would look like this:

```rust,no_run
# fn main() {}
# trait Animal {
#     fn get_name(&self) -> &str;
# }
pub struct Mischievous<A: Animal> {
    animal: A,
}
```

Now that we know that whatever is in each state's `animal` field must implement the `Animal` trait, we can treat it as
such in any implementation code for those states. Just remember that for generic `impl`s, it is the `impl` that
specifies the generic, so we need to make sure we add the Trait Bound there, then we can update our describe to use the
trait (here I've used the `format!` macro which is like `println!` but produces a `String`):

```rust
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

Update all of your States to use `self.animal.get_name()`, and try rerunning your program, you should get something 
like:

```text
Yuki is trying to break into a wardrobe by pulling on exposed clothing

Being loud doesn't work, Yuki chooses violence and attacks!

Look at the precious baby Yuki sleeping 😍

Yuki is trying to break into a wardrobe by pulling on exposed clothing
```

So that's our first problem solved!

---

Then we'll add a second animal, arguably the most dangerous of them all!

Pop this into `animal/human.rs`.

```rust
// File: animal/human.rs

pub struct Human {
    name: String
}

// impl Human {
//     pub fn new(name: String) -> Mischievous<Self> {
//         todo!()
//     }
// }
```

Your `animal/mod.rs` need to expose both of its submodules publicly.

```rust,ignore
// File: animal/mod.rs

pub mod cat;
pub mod human;
```

Finally, lets update our main function, and run the program to make sure everything is working.
