What next
=========

We've reached the end of Idiomatic Rust in Simple Steps. So what next?

Other Learning Resources
------------------------

If you'd like to dive into other learning resources, there's a lot of really amazing stuff out there.

The official book, [The Rust Programming Language] is available in both living digital and "dead tree" versions. If you
got all the way through IRISS and didn't look at the official book yet... how? Why? It's amazing!

For the more hands on, I can't recommend [Rustlings] enough. It mirrors the official book pretty closely and gives you
exercises to work through as you go. Similarly, [Tour of Rust] is another guide with embedded Rust Playground examples
so that you can play with the things you've just learned.

There are a ton of Rust YouTubers (including [Fio's Quest], obviously), but my top recommendations for people
specifically looking to learn are [No Boilerplate] and [Let's Get Rusty]. Also, a special mention to [Chris Biscardi]
who does really great guides on all things [Bevy], but also routinely runs through [Advent of Code] problems, solving
them with Rust.

If you know of any other good guides, please let me know! You can even send me a PR for the book on GitHub!

Start building
--------------

But the best way to learn Rust is to start building. Rust is one of the few languages that work in just about any field.
What do you want to build? 

To give you some food for thought, you can build command line apps, cross-platform GUI apps, full stack web apps,
embedded microcontroller apps, machine learning tools, networking tools, libraries that can be consumed by other
programs and video games. The list is essentially endless.

Early examples for me were; a CSV to Json converter using [Serde], a web server for reading specific details from a
WordPress database using [Actix Web] and [Diesel], a 250,000 cell game of life in [WASM], and a command line flash card
database using [Clap] and [Sqlx].

Think of something you're missing in your life, ideally something that's limited in scope and achievable, and try 
building it.

Cargo, Crates, and docs.rs
--------------------------

Very little of what we build is actually built from scratch. My goal with Idiomatic Rust in Simple Steps was to teach
Rust without getting distracted with third party libraries (which, even the official book gets a little distracted with
sometimes).

But now we're past that and ready to get really stuck in to everything the ecosystem has to offer.

Like most modern languages, Rust has a default package manager called [crates.io]. Here you'll find a wealth of
libraries for just about every use you can imagine, whether you're building for tiny embedded microcontrollers or
data center scale, distributed, GPU-powered, AI tools. In Rust parlance, we call external libraries "crates".

When you create a project in Cargo, it will create a `Cargo.toml` manifest file. By adding libraries from [crates.io]
to your `[dependencies]` (or `[dev-dependencies]`) section in your manifest file, Cargo will automatically download them
for you, and you'll be able to access them in your software.

Documentation for libraries can almost always be found on [docs.rs] (usually linked from the crate's page on 
[crates.io]).

Over to you
-----------

Where you go next is up to you, but I'd honestly love to hear about it. Fio's Quest has a [Discord], and a community of
wonderful, supportive people. Let us know what you're up to! 

[The Rust Programming Language]: https://doc.rust-lang.org/book/
[Rustlings]: https://rustlings.rust-lang.org/
[Tour of Rust]: https://tourofrust.com
[Fio's Quest]: https://www.youtube.com/@FiosQuest
[No Boilerplate]: https://www.youtube.com/@NoBoilerplate
[Let's Get Rusty]: https://www.youtube.com/@letsgetrusty
[Chris Biscardi]: https://www.youtube.com/c/chrisbiscardi
[Bevy]: https://bevy.org/
[Advent of Code]: https://adventofcode.com/

[Serde]: https://serde.rs/
[Actix Web]: https://actix.rs/
[Diesel]: https://diesel.rs/
[WASM]: https://www.rust-lang.org/what/wasm
[Clap]: https://docs.rs/clap
[Sqlx]: https://docs.rs/sqlx

[crates.io]: https://crates.io/
[docs.rs]: https://docs.rs/

[Discord]: https://fios-quest.com/discord/