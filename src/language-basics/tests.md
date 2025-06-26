Tests
=====

Tests. Are. Awesome.

They are my favourite bit of software engineering. I'm sure many of you can relate, but I don't tend to trust humans 
telling me I'm doing well at my job. When I make a test pass though, oh that dopamine hits good.

A good test makes sure that the thing we're building does what we think it should do.

Anecdotally I recently interviewed at a company where I professed my love of tests, and they told me flatly they don't
write tests, they "move fast". Later in the interview they admitted they were having morale issues because their
engineers were constantly getting called out of hours to fix things.

So, it begs the question: are you moving fast if you're writing software that doesn't work?

Software engineers are not paid to write software, we're paid to solve problems. Tests are what make sure we solved the
problem and by automating our tests they make sure we don't accidentally "unsolve" it further down the line.

The testing Pyramid
-------------------

There are many ways to test software, these largely fall into three categories that make up what we call the testing
pyramid.

<div class="pyramid">
<style scoped>
.pyramid code {
    text-align: center;
}
</style>

```text
            ↑         /----------\        |           
            |        / End-to-End \       |           
  more      |       /--------------\      |    less   
expensive   |      /   Integration  \     |  expensive
   use      |     /------------------\    |    use    
  less      |    /        Unit        \   |    more   
            |   /----------------------\  ↓           
```
</div>

It's a pyramid to indicate that, although all tests are important, those lower down the pyramid should be laying the
foundation for the others. Not only should you have more of them, but they will provide the greatest feeling of safety
and security.

### End-to-End Tests

E2E tests are designed to make sure that the user of the software you've created can complete full user 
journeys, for example, can the user open, edit and save a file in a desktop application or can a user add an item to
a shopping cart and checkout of an ecommerce store. End-to-End tests are the slowest form of test and lack depth.

### Integration Tests

Integration tests check that the code you control works correctly with code that your program depends on that you
don't control. This would include things like databases or other data stores, web apis, library apis, etc. Integration
tests are also used if your software produces a public API. You write tests to check that people using your software in
its more natural state. Because of the communication component to these tests, these tests are also quite slow.

### Unit Tests

Unit tests test a single unit of functionality. These tests are the simplest, fastest and should make up the bulk
of your testing. These tests are so important, that it is best practice to write them before you write the code 
you're going to test.

For this book, we're only going to cover Unit Tests. That isn't to say that Integration Tests and End-to-End Tests
aren't important, they absolutely are, and there are many good guides out there. But, it is to say, that Unit Tests are
_so_ important, that they significantly impact how we communicate about Rust code and particularly libraries that we
might use, and they'll change the way we talk about Rust in this book going forward.

Introduction to Modules
-----------------------

Unlike many languages, in Rust, tests live with the code that they're testing. To explain this we need to talk about
how code in Rust is organised with Modules.

A Module is simply a container for other things, functions, type definitions, other modules, etc. You could think of it
like a physical container, though you can nest any number of containers together. The contents of the module are private
to that module unless explicitly marked as public with the `pub` keyword.

We define a module with the `mod` keyword and a name. There are then three ways to define what's inside that module:
1. With a directory named the same thing as the module which contains the file `mod.rs`, eg `my_module/mod.rs`
2. With a file in the same directory named the same thing as the module, eg `my_module.rs`
3. Inside curly brackets, eg `mod my_module { ... }`

If the module exposes anything publicly, you can then reference them with the path to the module and the name of the
thing you're referencing separated by double colons. Sound familiar? It should, this is how we've been accessing Rust's
standard library. For example, the `stdin` function is inside the `io` module, which itself is available inside the
`std` library.

We access that function using `std::io::stdin()`. We can also use the `use` keyword to simplify this a bit, for example:

```rust,noplayground
use std::io::stdin; // Full name here

fn main() {
    let _ = stdin(); // No need to use the full name here
}
```

Test Modules
------------

In Rust, we typically create a test module near the code that is being tested. Let's say we want to test some of the
functions we wrote in the last chapter (I've renamed them slightly below).

First we start by creating a module to test these functions in the same file as the functions exist

```rust,noplayground
fn split_at(input: &str, at: usize) -> (&str, &str) {
    // ...
#     let up_to = std::cmp::min(at, input.len()); // Prevent out of bounds
#     (&input[..up_to], &input[up_to..])
}

fn split_around<'a>(input: &'a str, sub_string: &str) -> (&'a str, &'a str) {
    // ...
#   if let Some(found_at) = input.find(sub_string) {
#     (&input[..found_at], &input[found_at + sub_string.len()..])
#   } else {
#     (&input[..], &input[input.len()..])
#   }
}

fn split_around_many<'a>(input: &'a str, sub_string: &str, collection: &mut Vec<&'a str>) {
    // ...
#     if let Some(found_at) = input.find(sub_string) {
#         let end_pos = found_at + sub_string.len();
#         collection.push(&input[..found_at]);
#         split_around_many(&input[end_pos..], sub_string, collection);
#     } else {
#         collection.push(&input);
#     }
}

mod tests {
    // empty for now
}
```

As long as nothing in the `tests` module is used in your main program it shouldn't appear in your final binary, however,
this isn't good enough. There's a risk we might make a mistake, but even without that, the module will still be 
processed by the compiler in order to do things like type checking. We only care about this module when we're running
our tests and Rust provides us a way to tell it that, the 
[`#[cfg(...)]` attribute](https://doc.rust-lang.org/reference/conditional-compilation.html).

Attributes are one of Rust's many meta programming tools which we'll cover more in the future at increasing difficulty
levels. For now, the `cfg` attribute allows us to tell the Rust Compiler (`rustc`) _when_ we want to compile something.
There are many, many ways to use conditional compilation, but for tests it's pretty simple, we only want the module
compiled when we're building tests and `cfg` has a "predicate" to identify this simply called `test`.

We use `cfg` to only build our tests module when we're building for tests like this:

```rust,noplayground
# fn split_at(input: &str, at: usize) -> (&str, &str) {
#     let up_to = std::cmp::min(at, input.len()); // Prevent out of bounds
#     (&input[..up_to], &input[up_to..])
# }
# 
# fn split_around<'a>(input: &'a str, sub_string: &str) -> (&'a str, &'a str) {
#   if let Some(found_at) = input.find(sub_string) {
#     (&input[..found_at], &input[found_at + sub_string.len()..])
#   } else {
#     (&input[..], &input[input.len()..])
#   }
# }
# 
# fn split_around_many<'a>(input: &'a str, sub_string: &str, collection: &mut Vec<&'a str>) {
#     if let Some(found_at) = input.find(sub_string) {
#         let end_pos = found_at + sub_string.len();
#         collection.push(&input[..found_at]);
#         split_around_many(&input[end_pos..], sub_string, collection);
#     } else {
#         collection.push(&input);
#     }
# }
# 
#[cfg(test)]
mod tests {
    // still empty
}
```

Writing Tests
-------------

Now we're ready to write our first test.

A test is simply a function that we mark with another attribute `#[test]`.

Let's quickly write a broken test to make sure things are working.

```rust,noplayground
#[cfg(test)]
mod tests {
    
    #[test]
    fn test_split_at() {
        assert!(false, "Intentionally failing a test to show how they work")
    }
}
```
[Link to tests](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d62bd5aa150a9f0bdab23a98b655fc78)

> Note: currently mdbook, the tool we're using to create the IRISS book, does not support running tests natively. 
> Instead, we'll provide a permalink to the relevant code in Rust Playground. Inside Rust Playground, click the three 
> dots next to Run, and choose Test

The `assert!()` macro takes either one or two parameters. The first parameter which is not optional is a boolean value,
or something that evaluates to a boolean. If this boolean is false, then the assertion will cause a panic, and the test
will fail (unless it's expected to panic, more on that later).

The second, optional parameter allows us to annotate the assertion, which can help us more easily determine which (if
any) assertion failed in a test that might have multiple assertions. You'll find that people don't use this as much, I'm
guilty of this too, but I do recommend making an effort to describe each specific assertion. The people you work with,
as well as future you, will appreciate the effort.

There are three main assert macros:
- `assert!(<boolean value>, <optional message>)` asserts value is true or panics with optional message
- `assert_eq!(<left>, <right>, <optional message>)` asserts left is equal to right or panics with optional message
- `assert_ne!(<left>, <right>, <optional message>)` asserts left is NOT equal to right or panics with optional message

There are a couple of restrictions with the assert macros. Values used must implement `PartialEq` and `Debug`. Most 
built in types already implement both, and we'll talk about how to implement them for your own types in the Traits
chapter.

> You can also find more assert macros for specific types, in Rusts experimental tool chain, and in other libraries. 
> There are even libraries specifically built for enhancing your tests but these are out of scope for this book.

To run tests in our project we use `cargo test`. In the case of the above we should see the following:

```text
running 1 test
test tests::test_split_at ... FAILED

failures:

---- tests::test_split_at stdout ----
thread 'tests::test_split_at' panicked at src/lib.rs:6:9:
Intentionally failing a test to show how they work
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::test_split_at

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Testing our code
----------------

Let's move on to writing our actual tests, we'll start with "split_at".

Before we can write a test, `split_at` is not part of the `tests` module, so we need to make it available inside. We can
do that with the `use` statement in one of two ways, either `use super::split_at` or `use super::*`. The `super` keyword
simply means the module above this one, which for your unit tests should be the module to you're writing tests for. We
can either bring just the one function in, or we can bring in everything available in that scope. The idiom here is that
your module ideally shouldn't be so complicated that you can't bring in everything, so it's usually safe to
`use super::*`.

```rust,noplayground
fn split_at(input: &str, at: usize) -> (&str, &str) {
    // ...
#     let up_to = std::cmp::min(at, input.len()); // Prevent out of bounds
#     (&input[..up_to], &input[up_to..])
}
 
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_split_at() {
        let input = "Hello, world!";
        let (split_left, split_right) = split_at(input, 3);
        assert_eq!(split_left, "Hel", "First 3 characters");
        assert_eq!(split_right, "lo, world!", "Rest of input");
    }
}
```

[Link to tests](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c690b50e621e51a775b9ecf2019ee663)

Congratulations, we now have our first working test! If you mess with the assertions, you can see how the optional
message helps us find the broken assertion faster. And if you read the optional message, it tells us the expected 
behaviour... I think those of you who regularly USE non-english languages will see where the expectation doesn't meet 
the behaviour. 

Let's write another test for `split_at`:

```rust,noplayground
fn split_at(input: &str, at: usize) -> (&str, &str) {
    // ...
#     let up_to = std::cmp::min(at, input.len()); // Prevent out of bounds
#     (&input[..up_to], &input[up_to..])
}
 
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_split_at() {
        // ...
#         let input = "Hello, world!";
#         let (split_left, split_right) = split_at(input, 3);
#         assert_eq!(split_left, "Hel", "First 3 characters");
#         assert_eq!(split_right, "lo, world!", "Rest of input");
    }
    
    #[test]
    fn test_split_at_multibyte() {
        let input = "こんにちは世界！";
        let (split_left, split_right) = split_at(input, 3);
        assert_eq!(split_left, "こんに", "First 3 characters");
        assert_eq!(split_right, "ちは世界！", "Rest of input");
    }
}
```

[Link to tests](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=cf9de81995dc46c807e4d08e9cc4c75b)

This is why setting our expectations of functionality in plain, natural language, is so important!

We've explained in the test that we expected to split at the nth character NOT the nth byte. Now that we know this
doesn't match the expectations, we should fix our function.

Don't worry too much about this next bit yet, but if you'd like an explanation of the fix

In order to do this, we need to find the byte number where our number of characters is met. Looking at the 
documentation for [str](https://doc.rust-lang.org/std/primitive.str.html#method.chars) we can find there's a method
for creating an iterator of chars. For small strings, this feels like an acceptable way to find the new place to split 
our string slice. We'll also use a method called
[`take`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take) that we can use to effectively only take the
first X characters. We'll then `map` over each item remaining in the iterator to get its size in bytes, before summing
the number of bytes to get our new split point.

We no longer need the bounds check because, if the `at` is greater than the length of the string in characters, our
`chars` iterator will end before we reach the `take` limit.

```rust,noplayground
fn split_at(input: &str, at: usize) -> (&str, &str) {
    let byte_count = input.chars().take(at).map(|c| c.len_utf8()).sum();
    (&input[..byte_count], &input[byte_count..])
}
 
#[cfg(test)]
mod tests {
    // ...
#     use super::*;
#     
#     #[test]
#     fn test_split_at() {
#         let input = "Hello, world!";
#         let (split_left, split_right) = split_at(input, 3);
#         assert_eq!(split_left, "Hel", "First 3 characters");
#         assert_eq!(split_right, "lo, world!", "Rest of input");
#     }
#     
#     #[test]
#     fn test_split_at_multibyte() {
# 
#         let input = "こんにちは世界！";
#         let (split_left, split_right) = split_at(input, 3);
#         assert_eq!(split_left, "こんに", "First 3 characters");
#         assert_eq!(split_right, "ちは世界！", "Rest of input");
#     }
}
```

[Link to tests](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=09d99c5316173b306ad23c4f938a0b54)

Now this test works too!

Let's quickly write the tests for `split_around` and `split_around_many`:

```rust,noplayground
fn split_at(input: &str, at: usize) -> (&str, &str) {
    // ...
#     let byte_count = input.chars().take(at).map(|c| c.len_utf8()).sum();
#     (&input[..byte_count], &input[byte_count..])
}

fn split_around<'a>(input: &'a str, sub_string: &str) -> (&'a str, &'a str) {
    // ...
#   if let Some(found_at) = input.find(sub_string) {
#     (&input[..found_at], &input[found_at + sub_string.len()..])
#   } else {
#     (&input[..], &input[input.len()..])
#   }
}

fn split_around_many<'a>(input: &'a str, sub_string: &str, collection: &mut Vec<&'a str>) {
    // ...
#     if let Some(found_at) = input.find(sub_string) {
#         let end_pos = found_at + sub_string.len();
#         collection.push(&input[..found_at]);
#         split_around_many(&input[end_pos..], sub_string, collection);
#     } else {
#         collection.push(&input);
#     }
}
 
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_split_at() {
        // ...
#         let input = "Hello, world!";
#         let (split_left, split_right) = split_at(input, 3);
#         assert_eq!(split_left, "Hel", "First 3 characters");
#         assert_eq!(split_right, "lo, world!", "Rest of input");
    }
    
    #[test]
    fn test_split_at_multibyte() {
        // ...
#         let input = "こんにちは世界!";
#         let (split_left, split_right) = split_at(input, 3);
#         assert_eq!(split_left, "こんに", "First 3 characters");
#         assert_eq!(split_right, "ちは世界!", "Rest of input");
    }
    
    #[test]
    fn test_split_around() {
        let input = "Hello, world!";
        let (split_left, split_right) = split_around(input, ", ");
        assert_eq!(split_left, "Hello", "First 3 characters");
        assert_eq!(split_right, "world!", "Rest of input");
    }

    #[test]
    fn test_split_around_multibyte() {
        let input = "こんにちは世界！";
        let (split_left, split_right) = split_around(input, "世界");
        assert_eq!(split_left, "こんにちは", "First 3 characters");
        assert_eq!(split_right, "！", "Rest of input");
    }
    
    #[test]
    fn test_split_around_many() {
        let input = "The quick brown fox jumped over the lazy dog";
        let mut collection = Vec::new();
        split_around_many(input, " ", &mut collection);
        assert_eq!(collection, vec![
            "The",
            "quick",
            "brown",
            "fox",
            "jumped",
            "over",
            "the",
            "lazy",
            "dog",
        ]);
    }
    
    #[test]
    fn test_split_around_many_multibyte() {
        let input = "The quick brown キツネ jumped over the lazy 犬";
        let mut collection = Vec::new();
        split_around_many(input, " ", &mut collection);
        assert_eq!(collection, vec![
            "The",
            "quick",
            "brown",
            "キツネ",
            "jumped",
            "over",
            "the",
            "lazy",
            "犬",
        ]);
    }
}
```

[Link to tests](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1192fc71d8bf6be488c241a0a7f0a827)

Note that we didn't need to update the other functions for multibyte because we're specifically looking for a substring
that either exists or doesn't.

Now, what if I told you: we just did all of this backwards 😲

Test Driven Development
-----------------------

So now, hopefully, you're eager to write a load of code and then write a load of tests, but wait!

As we alluded to at the top of chapter, and again halfway through, the point of tests isn't to test that your code does
what you _think_ it does, it's to make sure it does what it's _supposed_ to do.

The best way to achieve this is to work out what your code is supposed to do, then write the test, then write the code.
This is called Test Driven Development (TDD).

Let's try some TDD. We'll create a function that checks if a given string is a palindrome (a word that's the same 
forwards and backwards).

We'll start by writing our test:

```rust,noplayground
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("kayak"));
        assert!(is_palindrome("racecar"));
        assert!(!is_palindrome("wood"));
    }
}
```

[Link to tests](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6f6409b54d8e8d3af13086a02030c3f4)

This won't compile though, so in order to run our test (even though it won't work), we need to write the function. We
don't want to write any code inside it yet though, so we'll use the `todo!()` macro.

```rust,noplayground
fn is_palindrome(_input: &str) -> bool {
    todo!("Implement the palindrome checker");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_palindrome() {
        // ...
#         assert!(is_palindrome("kayak"));
#         assert!(is_palindrome("racecar"));
#         assert!(!is_palindrome("wood"));
    }
}
```

[Link to tests](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=64e08f5b84aaf37520679327ae0eb015)


We use the `todo!` macro to state we are intending to come back and fix this code soon. It works even in our function 
that's supposed to return a boolean because Rust recognises that the todo macro will kill the program, and therefore the
function will never return.

We've also used an underscore on the front of the `_input` parameter just to let Rust know that _we_ know that parameter
isn't used yet (otherwise it'll warn us about it).

Let's think about our implementation. The string slice type doesn't have a reverse method built in to it and even if it
did, that would require allocating memory. Instead, lets use the chars iterator like we did earlier, we'll create two
iterators, reverse one of them, then zip them together. If every character matches its counterpart then the string is
a palindrome.

```rust,noplayground
fn is_palindrome(input: &str) -> bool {
    let forward = input.chars();
    let backward = forward.clone().rev();
    forward.zip(backward).all(|(f, b)| f == b)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("kayak"));
        assert!(is_palindrome("racecar"));
        assert!(!is_palindrome("wood"));
    }
}
```

[Link to tests](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=29b7f57a396eef9d06d0d0c501bd3e8c)

> ℹ️ Curiously, cloning an iterator does not necessarily cause a memory allocation. In this case we're safe, but it can
> be worth checking these things when speed and efficiency are important.

And now our test passes! But, uh-oh, when we send the code to be reviewed by a peer, they point out "racecar" isn't a 
word. They do think that "race car" (with a space) should be considered a palindrome, so we update our test, but now it
fails.

```rust,noplayground
fn is_palindrome(input: &str) -> bool {
    // ...
#     let forward = input.chars();
#     let backward = forward.clone().rev();
#     forward.zip(backward).all(|(f, b)| f == b)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("kayak"));
        assert!(is_palindrome("race car"));
        assert!(!is_palindrome("wood"));
    }
}
```

[Link to tests](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=55ca613fc48720fe1b2ab48370ccf866)

Now we broke our test, lets fix the code. This one is easy, we just ignore anything that's not a letter or a number.
We can do this by adding a filter to the iterator.

```rust,noplayground
fn is_palindrome(input: &str) -> bool {
    let forward = input.chars().filter(|c| c.is_alphanumeric());
    let backward = forward.clone().rev();
    forward.zip(backward).all(|(f, b)| f == b)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("kayak"));
        assert!(is_palindrome("race car"));
        assert!(!is_palindrome("wood"));
    }
}
```

[Link to tests](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=0b7ab806e298c477989d64dd899985b3)

And we've fixed the code. The person reviewing the code is happy, so it goes out to customers, but someone complains.
Their name is Anna, which is an anagram. We add it to the test:

```rust,noplayground
# fn is_palindrome(input: &str) -> bool {
#     let forward = input.chars().filter(|c| c.is_alphanumeric());
#     let backward = forward.clone().rev();
#     forward.zip(backward).all(|(f, b)| f == b)
# }
# 
# #[cfg(test)]
# mod tests {
#     use super::*;
#     
#[test]
fn test_is_palindrome() {
    assert!(is_palindrome("kayak"));
    assert!(is_palindrome("race car"));
    assert!(!is_palindrome("wood"));
    assert!(is_palindrome("Anna"));
}
# }
```

[Link to tests](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=62e10f1d7a723de4637d560c5153219c)

Capital letters are a little more complex as an uppercase character might be the same for multiple lowercase characters.
When we call `.to_lowercase()` on a character in Rust, it will return an iterator for each character that could
conceivably be turned into that uppercase character. If we map over each character and use `.to_lowercase()` then we 
have an iterator of iterators of characters. We can flatten this out with the `.flatten()` method to turn it back into 
an iterator of characters. Because we use `.rev()` after this point, it should still work with strings that contain
characters that could have multiple lowercase counterparts.

```rust,noplayground
fn is_palindrome(input: &str) -> bool {
    let forward = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase())
        .flatten();
    let backward = forward.clone().rev();
    forward.zip(backward).all(|(f, b)| f == b)
}
# 
# #[cfg(test)]
# mod tests {
#     use super::*;
#     
#     #[test]
#     fn test_is_palindrome() {
#         assert!(is_palindrome("kayak"));
#         assert!(is_palindrome("race car"));
#         assert!(!is_palindrome("wood"));
#         assert!(is_palindrome("Anna"));
#     }
# }
```
[Link to tests](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=e012e666c1862768da409a7995116cfe)

This function still isn't perfect, but it works for the given test conditions.

> If you want to continue developing this function to include things like diacritics, please do! But, you will need to
> start using external crates which is out of scope of this section of the book.

How many tests should a good test writer write if a good test writer could write good tests?
--------------------------------------------------------------------------------------------

The perennial of questions when it comes to testing: How many tests should you write?

The important thing here is "coverage". Do you have a test that "covers" each line of code.

Take for example this really silly function:

```rust,noplayground
fn is_positive(num: i32) -> bool {
    if num > 0 {
        true
    } else {
        true
    }
}
```

If we only test this function by giving it a number greater than 0, we'll only "cover" the line first branch of the
`if`, we miss the fact there is a mistake in the second branch. 

So what percent coverage should you aim for?

Anecdotally, when I was creating my own API framework in PHP, I decided I wanted to get 100% coverage, that is, every
line should have a test that hits it. The very last line that was uncovered was:

```php,ignore
        }
```

I wondered if it was worth the effort, decided it was. The reason this was the last uncovered line was that it was part
of a nested `if`. I'd tested what happens if you went into both `if`s, what happens if you didn't go into the first, but
not what happens if you went into the first, but not the second.

```php
function example() {
    if first_condition {
        if second_condition {
            return do_something();
        }
    } // <- It was this line
    return do_something_else();
}
```

I wrote the test and... found a bug so severe that I had to rewrite almost a third of the framework. Should have written
the tests first, right?

My personal feelings are that you as an engineer should strive for 100% coverage of your code.

As a manager or engineering lead though, test coverage is a terrible metric. Test coverage doesn't tell you if the test
was any good. If you make arbitrary metrics like this, you're not improving code quality, engineers will write tests 
that meet that metric, but don't fulfil the reason we want tests in the first place which is to answer: "does this
code do what we want it to do?".

As an engineer, it's a matter of pride. Get your code right now, and you won't have to deal with it later. As a leader,
make sure your engineers are encouraged to be the best they can. That means giving them the time to write tests first,
giving them access to the resources they need to learn how to write the best tests, etc.

Homework
--------

We've already let on how you can solve last chapters homework:

```rust,noplayground
fn split_around_many<'a>(input: &'a str, sub_string: &str, collection: &mut Vec<&'a str>) {
    if let Some(found_at) = input.find(sub_string) {
        let end_pos = found_at + sub_string.len();
        collection.push(&input[..found_at]);
        split_around_many(&input[end_pos..], sub_string, collection);
    } else {
        collection.push(&input);
    }
}
```

We only needed one lifetime to represent the link between the input string slice and the string slice inside our vector.

For a cleaner API though you could keep our recursive function private and expose a public function that creates the
vector for us. Because we have control of the vector in this example we can make sure we create a vector with a capacity
that is at least as large as the maximum possible entries in it. This is useful as when you create a new Vector in Rust
it has a default size, and any time you try to add an item to a vector that is already full, Rust will allocate the
memory for a new larger vector in the background, copy the data from the old location to the new location, then free the
memory in the old location.

```rust,noplayground
fn split_around_many_recurse<'a>(input: &'a str, sub_string: &str, collection: &mut Vec<&'a str>) {
    // ...
#     if let Some(found_at) = input.find(sub_string) {
#         let end_pos = found_at + sub_string.len();
#         collection.push(&input[..found_at]);
#         split_around_many_recurse(&input[end_pos..], sub_string, collection);
#     } else {
#         collection.push(&input);
#     }
}

pub fn split_around_many<'a>(input: &'a str, sub_string: &str) -> Vec<&'a str> {
    let mut output = Vec::with_capacity(input.matches(sub_string).count());
    split_around_many_recurse(input, sub_string, &mut output);
    output
}
```

For the homework in this chapter, I would like you to write the tests for, and then implement the code for the following
requirements, one at a time, in this order:

1. Create a function that will reverse the words in an English sentence.
2. If the string starts or ends with whitespace, it should be removed (trimmed) from the returned String.
3. If the string contains more than one sentence, the function should return an error (though for now, that error can
   be the unit type `()`).

Your function will need to allocate memory and should probably have the header:

```rust,ignore
fn reverse_sentence(input: &str) -> Result<String, ()>
```

Here is the documentation on [String](https://doc.rust-lang.org/std/string/struct.String.html) and
[Result](https://doc.rust-lang.org/std/result/enum.Result.html)

Going forward
-------------

As I mentioned, after this chapter we will be talking about Rust differently. This will primarily be by using the assert
macros to state what a value _should_ be.

For example, one of our earlier code examples looks like this:

```rust
let min_byte: u8 = 0b0000_0000;
let max_byte: u8 = 0b1111_1111;
println!("min_byte: {min_byte}"); // 0
println!("max_byte: {max_byte}"); // 255
```

But from here on out it'll look like this:

```rust
let min_byte: u8 = 0b0000_0000;
let max_byte: u8 = 0b1111_1111;
assert_eq!(min_byte, 0);
assert_eq!(max_byte, 255);
```

Now that you've seen the assert macros, this should be cleaner and clearer.
