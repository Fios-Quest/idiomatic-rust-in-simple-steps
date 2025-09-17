What next
=========

We've reached the end of Idiomatic Rust in Simple Steps. So what next?

Cargo
-----

Crates
------

docs.rs
-------

Other Learning Resources
------------------------

If you'd like to dive into other learning resources, there's a lot of great stuff out there.

The official book, [The Rust Programming Language](https://doc.rust-lang.org/book/) is available in both living digital
and "dead tree" versions. If you got all the way through IRISS and didn't look at the official book yet, how? Why? It's
amazing!

For the more hands on, I can't recommend [Rustlings](https://rustlings.rust-lang.org/) enough. It mirrors the official
book pretty closely and gives you exercises to work through as you go. Similarly, [Tour of Rust](https://tourofrust.com)
is another guide with embedded Rust Playground examples so that you can play with the things you've just learned. 

There are a ton of Rust YouTubers (including [Fio's Quest](https://www.youtube.com/@FiosQuest), obviously), but my top
recommendations for people specifically looking to learn are [No Boilerplate](https://www.youtube.com/@NoBoilerplate)
and [Let's Get Rusty](https://www.youtube.com/@letsgetrusty). Also, a special mention to 
[Chris Biscardi](https://www.youtube.com/c/chrisbiscardi) who does really great guides on all things Bevy, but also
routinely runs through [Advent of Code](https://adventofcode.com/) problems, solving them with Rust.

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

[Serde]: https://serde.rs/
[Actix Web]: https://actix.rs/
[Diesel]: https://diesel.rs/
[WASM]: https://www.rust-lang.org/what/wasm
[Clap]: https://docs.rs/clap
[Sqlx]: https://docs.rs/sqlx

