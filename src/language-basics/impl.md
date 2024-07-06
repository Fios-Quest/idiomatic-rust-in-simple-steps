# Giving types functionality

Next we're going to learn how to add functionality to data by modelling a common emotional pattern followed by my cat, 
Yuki.

## Yuki States

We'll model 3 emotional states of my Cat, give him behaviours unique to each state, and allow him to transition between
those states, but only in a specific

![YukiStateMachine.svg](./impl/YukiStateMachine.svg)

So what is the behaviour we want to model in these states?

### Mischievous

- We'll initialise Yuki in the Mischievous state because that's how he wakes up.
- In this state he'll make a lot of fun little noises
- He'll also try to get up to various naughtiness
- If we forget to feed him he'll get Hangry

### Hangry (Hungry and Angry)

- While hangry, he only really makes one noise, a desperate plea to "pay attention and do your job"
- He'll get hyper focused on you getting your attention and may choose violence
- Once he's eaten, he'll get Eepy

### Eepy (Sleepy)

- Once he's eaten, he'll get sleepy, he won't make any noises he'll just settle down in one of his beds
- Once he's slept, he'll get back to his mischeivous state

## Making a Cat

Let's create a new project with `cargo new yuki-state-machine` and open it in our IDE of choice.

For this project we're going to need to organise our code a bit. We've talked a little bit about modules before, they're
Rust's way of organising and grouping code. We've created modules to contain tests, that we've then excluded from our 
build. You can create them with the `mod` keyword, then a name, then either:
- a block of code surrounded by curly brackets
- a file called the same thing as the module (e.g. `mod my_module` and a file called `my_module.rs`)
- a directory called the same thing as the module and a file called `mod.rs` (e.g. `mod my_module` and a file called
  `my_module/mod.rs`)

We're going to use all of these techniques in this project, though only the latter two today.

To organise our cat code away from the main binary, lets create a cat module inside our main file, so it looks something
like this:

```rust,ignore
// File: main.rs
mod cat;

fn main() {
    println!("Hello, world!");
}
```

If you're using VSCode, RustRover or similar, you might be able to use the context menu to create `cat/mod.rs` by 
putting the cursor over `cat` and pressing activating the context actions (in VSCode that's `Ctrl`/`⌘` + `.`, in
IntelliJ products like RustRover it's `Alt`/`⌥` + `enter`). If you aren't able to do that, create a directory called
`cat` in your `src` folder, then create a `mod.rs` folder in there.

Inside our `mod.rs` file lets create a structure to hold a cat, we'll make it public by putting the word `pub` in front
of the struct.

```rust
// File: cat/mod.rs
pub struct Cat {
    name: String,
}
```

We can access the `Cat` struct either by giving a full reference to the `Cat` struct in its module, `cat::Cat` or by
using the `use` keyword. However, you'll find we can't actually _create_ the structure.

```rust,compile_fail
# // This would be in your `cat/mod.rs`, I need to put it here to make the code work in mdbook
# mod cat {
#   pub struct Cat {
#     name: String,
#   }
# }
# 
// File: main.rs
mod cat;

use cat::Cat;

fn main() {
    Cat { name: "Yuki".to_string() }
}
```


```rust
enum YukiState {
    Mischievous,
    Hangry,
    Eepy,
}
```


