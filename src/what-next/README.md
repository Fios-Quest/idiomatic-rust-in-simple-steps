What next
=========

We've reached the end of Idiomatic Rust in Simple Steps. So what next?

Other Learning Resources
------------------------

If you'd like to dive into other learning resources, there's a lot of really amazing stuff out there.

The official book, [The Rust Programming Language] is available in both living digital and "dead tree" versions. If you
got all the way through IRISS and didn't look at the official book yet... how? Why? It's amazing!

The book goes into far more detail that IRISS, and makes fewer pains to avoid non-idiomatic examples or crates so gets 
to the point more directly. Particularly if you've gone through IRISS already, the official book will be a sinch to get
through and give you a much broader 

For the more hands on, I can't recommend [Rustlings] enough. It's a collection of short exorcises that don't just tell
you how Rust works, but get you to write it. You run it locally, and you can only pass a section when your code works! 
It mirrors the official book pretty closely so working through both at the same time can really support your learning. 

Similarly, [Tour of Rust] is another guide with embedded Rust Playground examples so that you can play with the things
you've just learned.

If you're more into visual learning, there are some great YouTube channels out there (including [Fio's Quest],
obviously). Here are some of my favourites: 

- [No Boilerplate] offers what Tris, the creator, describes as "Fast technical videos". His video's cover everything
  from high level concepts such as why Rusts design made certain choices and how they help you be a better developer,
  down to how to use specific language features. (Note: Tris' video on "async isn't real and can't hurt you" convinced
  me to add a reminder you don't _have_ to write async Rust to the async chapter)
- [Let's Get Rusty] has a lower level focus than No Boilerplate. Bogdan, who runs the channel, goes into more detail on
  common Idioms and specific crates you might need to learn for specific tasks.
- [Chris Biscardi] makes great guides on all things [Bevy], showcases new Bevy games, tools and framework features.
  Even if game dev isn't your thing, Chris routinely runs through [Advent of Code] problems, solving them with Rust,
  which is a great place to see smart algorithms written succinctly in idiomatic code. (Note: Chris is where I learned
  about [nom], and how to do better parsing, which might have helped land my latest job!)

If you know of any other good guides, please let me know! You can even send me a PR for the book on GitHub!

Start building
--------------

But the best way to learn Rust is to start building. Rust is one of the few languages that work in just about any field.
What do you want to build? 

To give you some food for thought, you can build command line apps, cross-platform GUI apps, full stack web apps,
embedded microcontroller apps, machine learning tools, networking tools, libraries that can be consumed by other
programs and even video games. The list is essentially endless.

Early examples for me were;
- a CSV to Json converter using [Serde]. Serde, of no agreed pronunciation, is short for **ser**ialize, **de**serialize.
  It's the goto tool for converting string data formatted in a variety of file formats, into data in your application
  and back again.
- A web server for reading specific details from a WordPress database using [Actix Web] and [Diesel]. Actix Web is an
  incredibly fast web server framework that's surprisingly easy to work with. Diesel is a Database ORM that not
  only lets you read and write to databases, but can also manage things like table structure and migrations.
- A 250,000 cell Game Of Life in [Web Assembly] that ran at 60fps. Web Assembly isn't a framework or library like
  other tools I'm listing here, it's a compile target. You can compile Rust into Web Assembly. I would even go so far
  as to argue that, thanks to how Rust works as a language, and especially the supporting tooling, Rust should be
  everyone's first choice when writing high performance code for frontend web.
- A command line flash card database using [Clap] and [Sqlx]. Clap is a tool for parsing command line arguments as well
  as providing a common experience across cli applications. Sqlx is another framework for working with Databases but
  doesn't have the ORM features built into Diesel.

My advice is to think of something you're missing in your life, ideally something that's limited in scope and
achievable, plan out how you'd structure the solution, and try building it.

Cargo, Crates, and docs.rs
--------------------------

Very little of what we build is actually built from scratch. My goal with Idiomatic Rust in Simple Steps was to teach
Rust without getting distracted with third party libraries (which, even the official book gets a little distracted with
sometimes).

But now we're past that and ready to get really stuck in to everything the ecosystem has to offer.

Like most modern languages, Rust has a default package manager called [crates.io]. Here you'll find a wealth of
libraries for just about every use you can imagine, whether you're building for tiny embedded microcontrollers or
data center scale, distributed, GPU-powered, AI tools. In Rust parlance, we call external libraries "crates".

When you create a project in Cargo, it will create a `Cargo.toml` manifest file. By adding libraries from crates.io
to your `[dependencies]` (or `[dev-dependencies]`) section in your manifest file, Cargo will automatically download them
for you, and you'll be able to access them in your software.

Documentation for libraries can almost always be found on [docs.rs] (usually linked from the crate's page on 
crates.io). docs.rs is built from rustdoc, which we covered in the [documentation] chapter. This means library
documentation will always be structured in a familiar way making it easy to use.

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
[nom]: https://docs.rs/nom

[Serde]: https://serde.rs/
[Actix Web]: https://actix.rs/
[Diesel]: https://diesel.rs/
[Web Assembly]: https://www.rust-lang.org/what/wasm
[Clap]: https://docs.rs/clap
[Sqlx]: https://docs.rs/sqlx

[crates.io]: https://crates.io/
[docs.rs]: https://docs.rs/
[documentation]: ../language-basics/documentation.md

[Discord]: https://fios-quest.com/discord/
