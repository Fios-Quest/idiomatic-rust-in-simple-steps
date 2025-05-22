Control Flow
============

Programs are typically executed one line at a time (this is called flow), but we can alter what the next line is with
control flow.

There are two main ways of doing this: [branching](#branching) and [looping](#looping).

Before we do that though, lets talk about two of Rust's coolest features, which will come up a lot later, 
[patterns](#patterns) and how [blocks are also expressions](#blocks-are-expressions).

Patterns
--------

In the last chapter we talked about compound types. Tuples, Structs, and Enums allow the construction of more complex
data from less complex data. However, if we want to extract any of the component parts of that data we can do that!

Patterns can be used to "destructure" compound data types like tuples fairly trivially:

```rust
let point = (123, 456);
let (x, y) = point;
println!("The point was at x: {x} and y: {y}");
```

> It's important to note though, that the original data will no longer be accessible if it doesn't implement `Copy`:
> 
> ```rust,compile_fail
> // This code won't compile!
> let point = (123.to_string(), 456.to_string());
> let (x, y) = point;
> let this_wont_work = point.0;
> # println!("using these variable to remove irrelevant warnings {x}, {y}, {this_wont_work}");
> ```
> 
> We'll talk more about copy, ownership and move semantics later in the book.

Destructuring with patterns also works for Tuple Structs, however, you need to specify the name of the struct like
you're doing a weird backwards struct instantiation. 

```rust
struct Point (u64, u64);

fn main() {
    let point = Point(123, 456);
    
    let Point(x, y) = point;
    
    println!("The point was at x: {x} and y: {y}");
}
```

The same thing also works for Structs with Named Fields:

```rust
struct Point {
    x: u64,
    y: u64,
}

fn main() {
    let point = Point { x: 123, y: 456 };
    
    let Point { x, y } = point;
    
    println!("The point was at x: {x} and y: {y}");
}
```

In the above example we extract the structs named fields straight into variables of the same name as it's easy and the
names were appropriate. However, it might be better in the context of your program to name them something else. Below
we've renamed `x` to `width` and `y` to `height`:

```rust
# struct Point {
#    x: u64,
#    y: u64,
# }
#
# fn main() {
#    let rect = Point { x: 123, y: 456 };
#
let Point { x: width, y: height } = rect;

println!("The rect was {width} wide and {height} high");
# }
```

Unfortunately, you can not extract data from Enums this way as the value of an Enum is one of a set of, not only values,
but potentially subtypes or shapes or however you'd like to describe them. Take for example the humble Options:

```rust
let maybe_yuki: Option<char> = Some('雪');
let maybe_not: Option<char> = None;
```

How can we extract a `char` from `Option<char>` if we don't know whether the variable is `Some` or `None`... well, 
actually, we'll come to that soon. 🙂

Blocks are Expressions
----------------------

Before we get too deep into Rusts control flow I want to show you one of Rusts coolest features, expressions.

An expression in Rust is anything that could have a value. So, for example, `a + b` is an expression where we're adding
`a` to `b` which results in a value. You will also use expressions like `a == b` to compare whether the values of `a`
and `b` are the same, this results in a value of `true` or `false`.

Usually you might use an expression as part of an assignment or an evaluation, for example `let c = a + b` or 
`if a == b { ... }`, however, Rust also allows you to use a block (code between `{` and `}`) as an expression _and_ the
final value of that block can itself be an expression.

Here's a very contrived example:

```rust
let c = {
    let a = 3;
    let b = 5;
    a + b
};
println!("{c}");
```

Some cool things to note:
- `a` and `b` only exist within the code block
- the lines with `let` have semicolons
- the line with the expression `a + b` does not
- `c` will be equal to the evaluation of the code block, which itself is equal to the result of `a + b`
- the code block which `c` is equal to is also terminated with an exclamation

Why is this so cool? Because branches, loops and even functions all use code blocks!

Branching
---------

### If

The most basic form of branching is the `if` statement.

In its most simple form it's an `if` followed by an expression (unlike many languages this does not need to be in 
brackets) followed by a code block. The expression must evaluate to a boolean, either `true` or `false`. If the
expression evaluates to `true`, then the code in the block will be run, otherwise it won't be: 

```text
if <expression> {
    <code to run if expression is true>
}
```

For example, we could create an expression that evaluates to a boolean by comparing if two numbers are the same, using
double equals:

```rust
let a = 1;
let b = 1;
    
if a == b {
    println!("if expression is true print this");
}
println!("regardless of whether expression was true print this");
```

If you want to run some code if the expression is `true`, but some different code if it's `false`, then you can extend
`if` with `else`. Here we compare if the first number is greater than the second number.

```rust
let a = 1;
let b = 1;
    
if a > b {
    println!("if expression is true print this");
} else {
    println!("if expression is false print this instead");
}
```

You can chain `if`/`else` statements to create more complex branches.

```rust
let a = 1;
let b = 1;
    
if a > b {
    println!("a is greater than b");
} else if a == b {
    println!("a is equal to b");
} else {
    println!("a must be less than b");
}
```

Remember though, code blocks, including those in `if` and `else` are themselves expressions. This means they can 
effectively return their own values

```rust
let a = 1;
let b = 1;
    
let message = if a > b {
    "a is greater than b".to_string() 
} else if a == b {
    "a is equal to b".to_string()
} else {
    "a must be less than b".to_string()
};
    
println!("{message}");
```

Some important things to note:
1. The last line of each code block has no semicolon
2. When we create expressions like this, we must terminate them with a semicolon (see after the final `}`)
3. All branches must evaluate to the same _Type_, even if they don't evaluate to the same _value_
4. Doing big blocks of `if`/`else if`/`else` is a mess, there's a [better way](#match)!

#### Pattern Matching inside `if` and `else`

There is another way you can branch with `if` that doesn't require a boolean expression, pattern matching.

There are two ways to do this `if let ...` and `let ... else`.

Let's go back to that Option from earlier:

```rust
let maybe_yuki: Option<char> = Some('雪');
let maybe_not: Option<char> = None;
    
if let Some(c) = maybe_yuki {
    // This line will be printed
    println!("The character was '{}'", c);
}
    
if let Some(c) = maybe_not {
    // This line will not
    println!("The character was '{}'", c);
}
```

In the line `if let Some(c) = maybe_yuki` we are pattern matching on the Option, if it matches the pattern of 
`Some(<variable>)`, then we extract the contents of the `Some` into the `<variable>`. Within the block (and only within
the block), the variable `c` has the value from inside the `Some` variant of the `Option`.

This may be easier to observe with our own enum type. Imagine the following:

```rust
enum Vector {
    Two(f32, f32),
    Three(f32, f32, f32),
}

fn main() {
    let v = Vector::Three(3.0, 4.0, 5.0);
    
    if let Vector::Two(x, y) = v {
        // This line will not be printed
        println!("The 2D vector has the magnitude '{}'", (x*x + y*y).sqrt());
    }
    
    if let Vector::Three(x, y, z) = v {
        // This line will
        println!("The 3D vector has the magnitude '{}'", (x*x + y*y + z*z).sqrt());
    }
}
```

This example is a little contrived, there are better ways to do this.

You can also do the opposite, branch if the pattern does not match, using `let ... else`. The important thing to note
here is that execution can not continue after the code block, you must exit the current flow, whether thats returning 
from a function or breaking from a loop

```rust
fn some_function() {
    let maybe_yuki: Option<char> = Some('雪');
        
    let Some(c) = maybe_yuki else {
        // This code is executed if the maybe_yuki was None
        // We must exit from the code here, as we can not go back to the normal execution
        return;
    };
    // From this point forward, the contents of the Option has been extracted into the variable `c`
    println!("The character was '{}'", c);
}
# some_function();
```

### Match

This pattern matching stuff is really handy, right?!

Well the creators of Rust thought so too, in fact, they made a whole control flow mechanism around it!

`match` is a bit like `if` in that it can branch, and act as an expression. However, `match` can do a lot more than
`if`: it will match against multiple possibilities, allows match guards for fine grain control of pattern matching, and
it's exhaustive, meaning that a match _must_ deal with every possibility.

Lets look at our Vector example again:

```rust
# enum Vector {
#     Two(f32, f32),
#     Three(f32, f32, f32),
# }
#
# fn main() {
let v = Vector::Three(3.0, 4.0, 5.0);

match v {
    Vector::Two(x, y) => println!("The 2D vector has magnitude '{}'", (x*x + y*y).sqrt()),
    Vector::Three(x, y, z) => println!("The 3D vector has magnitude '{}'", (x*x + y*y + z*z).sqrt()),
}
# }
```

First of all, you can see that this pattern is _much_ cleaner than having a lot of `if let`s. We're matching against
the variants of an enum, and can immediately extract the contents from each variant. We could also use match as an
expression:

```rust
# enum Vector {
#     Two(f32, f32),
#     Three(f32, f32, f32),
# }
#
let v = Vector::Three(3.0, 4.0, 5.0);
    
let magnitude = match v {
    Vector::Two(x, y) => (x*x + y*y).sqrt(),
    Vector::Three(x, y, z) => (x*x + y*y + z*z).sqrt(),
};

println!("The vector has the magnitude '{}'", magnitude);
```

(This gets even more exciting when we get into functions)

What happens if we add another variant to the enum though? Well, that `match` statement will see that not every case is
handled, and cause an error.

```rust,compile_fail
enum Vector {
    Two(f32, f32),
    Three(f32, f32, f32),
    Four(f32, f32, f32, f32),
}

fn main() {
    let v = Vector::Three(3.0, 4.0, 5.0);
    
    // This match will no longer compile
    let magnitude = match v {
        Vector::Two(x, y) => (x*x + y*y).sqrt(),
        Vector::Three(x, y, z) => (x*x + y*y + z*z).sqrt(),
    };

    println!("The vector has the magnitude '{}'", magnitude);
}
```

We can deal with this by either adding the missing case, or using `_`, which is a special variable that immediately 
discards whatever is put into it and will match anything.

```rust
# enum Vector {
#    Two(f32, f32),
#    Three(f32, f32, f32),
#    Four(f32, f32, f32, f32),
# }
#
# let v = Vector::Three(3.0, 4.0, 5.0);
#
let magnitude = match v {
    Vector::Two(x, y) => (x*x + y*y).sqrt(),
    Vector::Three(x, y, z) => (x*x + y*y + z*z).sqrt(),
    // This specific example isn't great, now any variant that doesn't match will return zero, an error might be better
    _ => 0.0,
};
# println!("The vector has the magnitude '{}'", magnitude);
```

Patterns on match arms are tested from top to bottom, and you can also match on more specific patterns, like values:

```rust
# enum Vector {
#     Two(f32, f32),
#     Three(f32, f32, f32),
# }
#
let v = Vector::Two(0.0, 0.0);

let magnitude = match v {
    // This arm will match, print the statement and return 0
    Vector::Two(0.0, y) =>  {
        println!("Hey, did you know that x was zero?");
        y
    },
    // Although `v` does match this arm, because we already matched on the previous arm, this block won't be run
    Vector::Two(x, 0.0) => {
        println!("Hey, did you know that y was zero?");
        x
    }
    // Nor will this one
    Vector::Two(x, y) => (x*x + y*y).sqrt(),
    Vector::Three(x, y, z) => (x*x + y*y + z*z).sqrt(),
};
#    println!("The vector has the magnitude '{}'", magnitude);
```

There's one more trick up `match`'s sleeve which is match guards. Say we want to do something similar to the above, but
instead of matching on exactly zero, we want to match on values less than 10. We could make an arm for every variant, or
we could use a match guard which is like a mini if statement:

```rust
# enum Vector {
#    Two(f32, f32),
#    Three(f32, f32, f32),
# }
#
let v = Vector::Two(0.0, 0.0);

let magnitude = match v {
    // This arm will match, print the statement and return 0
    Vector::Two(x, y) if x < 10.0 =>  {
        println!("Hey, did you know that x was small?");
        (x*x + y*y).sqrt()
    },
    // Although `v` does match this arm, because we already matched on the previous arm, this block won't be run
    Vector::Two(x, y)  if y < 10.0  => {
        println!("Hey, did you know that y was small?");
        (x*x + y*y).sqrt()
    }
    // Nor will this one
    Vector::Two(x, y) => (x*x + y*y).sqrt(),
    Vector::Three(x, y, z) => (x*x + y*y + z*z).sqrt(),
};
#    println!("The vector has the magnitude '{}'", magnitude);
```

Looping
-------

### Loop

The most basic loop is, well, `loop`.

When you enter a loop, the code inside it will run until it's explicitly told to stop. For example:

```rust
# let mut protect_the_loop: u8 = 0;
loop {
    println!("These lines will print out forever");
    println!("Unless the program is interrupted, eg, with Ctrl + C");
#     protect_the_loop = protect_the_loop + 1;
#     if protect_the_loop >= 10 {
#         println!("I hid a break in this code as you can't Ctrl + C if you run this on Rust Playground / via the book");
#         break;
#     } 
}
```

This might seem a little bit unhelpful, surely you never want to get trapped inside a loop forever, but actually, we
often want to keep a program running inside a loop.

You can manually exit the loop using the `break` keyword. Like other languages, you can simply break from a loop, but
remember that blocks can be expressions, and this applies to loops too! That means we can have a loop that does some
work, and once the work is done, break with the value we want to take from the loop.

In the example below, we run a loop until we find some cool number (note the use of `if let`), then break with that
value. The Type of found is an `u64` (don't forget you can expand the code in the example if you're curious), and by
breaking with that value, the Type of the whole loop becomes `u64` too!

```rust
# use std::time::*;
# use std::thread::*;
# fn prep() {
#     loop {    
#         let secs = UNIX_EPOCH
#             .elapsed()
#             .expect("Call the Doctor, time went backwards")
#             .as_secs();
#         if secs % 2 == 1 {
#             break;
#         } 
#         sleep(Duration::from_millis(100));
#     }   
# }
# fn find_a_cool_number() -> Option<u64> {
#     let secs = UNIX_EPOCH
#         .elapsed()
#         .expect("Call the Doctor, time went backwards")
#         .as_secs();
#     (secs % 2 == 0).then_some(secs / 2)
# }
# 
# fn main() {
let some_cool_number = loop {
    println!("Looking for a cool number...");

    if let Some(found) = find_a_cool_number() {
        break found;
    }
#    sleep(Duration::from_millis(100));
};

println!("The number we found was {some_cool_number}");
# }
```

Another useful keyword when looping is `continue`. Imagine you have a series of things that need to be processed but
you can skip over _some_ of those things.

The following example will continuously get images, and run a time-consuming `process_image` function, unless the image
is an SVG, in which case it will skip it. 

```rust
# use std::{
#     io::{stdout, Write},
#     thread::sleep,
#     time::{Duration, UNIX_EPOCH}
# };
#
# fn main() {
#     let mut protect_the_loop: u8 = 0;
loop {
    let image = get_image();
    if image.is_svg {
        println!("Skipping SVG");
        continue;
    }
    process_image(image);
#
#         protect_the_loop = protect_the_loop + 1;
#         if protect_the_loop >= 10 {
#             println!("Protecting the loop again, this is only for demo purposes");
#             break;
#         }
}
# }
# 
# 
# struct Image {
#     is_svg: bool,
# }
# 
# fn get_image() -> Image {
#     let micros = UNIX_EPOCH
#         .elapsed()
#         .expect("Call the Doctor, time went backwards")
#         .as_micros();
#     Image {
#         is_svg: micros % 3 == 0,
#     }
# }
# 
# fn process_image(_image: Image) {
#     println!("Processing Image, please wait... done");
# }
```

There's one more neat trick up Rust's sleeve. As with most languages, Rust of course supports nested loops, but to aid
with things like `break` and `continue` it also supports labels.

Labels start with a single quote `'` and mark the loop they are for with a colon.

This very contrived example steps through a set of instructions. See if you can guess what will happen (see below 
for the answer).

```rust
enum LoopInstructions {
    DoNothing,
    ContinueInner,
    ContinueOuter,
    BreakInner,
    BreakOuter,
}

fn main() {
    let sequence = [
        LoopInstructions::DoNothing,
        LoopInstructions::ContinueInner,
        LoopInstructions::ContinueOuter,
        LoopInstructions::BreakInner,
        LoopInstructions::BreakOuter
    ];

    // This lets us get one bit of the sequence at a time
    // Don't worry too much about it for now!
    let mut iter = sequence.iter();

    'outer: loop {
        println!("Start outer");
        'inner: loop {
            println!("Start inner");

            match iter.next() {
                Some(LoopInstructions::ContinueInner) => continue 'inner,
                Some(LoopInstructions::ContinueOuter) => continue 'outer,
                Some(LoopInstructions::BreakInner) => break 'inner,
                Some(LoopInstructions::BreakOuter) => break 'outer,
                _ => {}
            }

            println!("End inner");
        }
        println!("End outer");
    }
}
```

1. The outer loop starts so we get **"Start outer"**
2. We enter the inner loop so we see **"Start inner"**
3. The **first** instruction `DoNothing` is read, it matches the last arm which does nothing so we continue
4. After the match we hit **"End inner"**
5. The inner loop starts again so we get **"Start inner"**
6. The **second** instruction `ContinueInner` matches, we execute `contine 'inner` so we start the inner loop again
7. We've started the inner loop again due to the previous instruction and get **"Start inner"**
8. The third instruction `ContinueOuter` matches, we execute `continue 'outer` so go to the beginning of that loop
9. We're back at the start so we see **"Start outer"**
10. And re-enter the inner loop **"Start inner"**
11. The **fourth** instruction is `BreakInner` so we execute `break 'inner`, when exits the inner loop
12. We exit the inner loop and continue from that point so we finally see **"End outer"**
13. The outer loop starts over so we see **"Start outer"**
14. We enter the inner loop and see **"Start inner"**
15. The **final** instruction `BreakOuter` matches so we execute `break 'outer`, which exits the outer loop and ends 
    the program

### While

While `loop` is great for programs that actually do want to try to keep running forever (or perhaps has many exit
conditions), we often only want to loop over something `while` something is true. The `while` loop takes an expression
that evaluates to true or false. The expression is checked at the start of each iteration through the loop; if it's
true, the loop will execute.

```rust
# fn main() {
let mut counter = 0;
while counter < 10 {
    println!("The counter is at {counter}");
    counter += 1;
}
println!("The loop has finished");
# }
```

The above is actually not a great way to loop over numbers, imagine if we forgot to add to counter!

Here's a different example where we call a function until we're happy with the result.

```rust
# fn get_seconds() -> u64 {
#     std::time::UNIX_EPOCH
#         .elapsed()
#         .expect("Call the Doctor, time went backwards")
#         .as_secs()
# }
#
# fn main() {
while get_seconds() % 3 != 0 {
    println!("The time in seconds is not divisible by 3");
}
println!("The time was successfully divided by 3!");
# }
```

What's really cool though is that you can do all the tricks we've learned above, including pattern matching with
`while let`.

```rust
# fn main() {
# let mut messages = "The quick brown fox jumped over the lazy dog".split(" ");
# let mut get_message = move || messages.next();
while let Some(message) = get_message() {
    println!("Message received: {message}")
}
println!("All messages processed");
# }
```

`while let` is extremely useful, and we'll see it more in the future, particularly when we deal with async await later.

### For In

A very common reason for looping in software is because we want to loop over every item in a collection and perform the
same set of instructions for each. This is where `for ... in ...` comes in.

For In allows you to step through an `Iterator`, or anything that implements `IntoIterator`, both of which we'll talk
more about in a later chapter. Simply put though, this lets us step over each item in a collection, stream or series of
data, even series' that might be infinite!

Often times you might want to do this with a collection such as an [Array](data-types.md#arrays). For example:

```rust
# fn main() {
let messages: [&str;2] = ["Hello", "world"];
for message in messages {
    println!("Message received: {message}")
}
println!("All messages processed");
# }
```

#### Range

Another neat Rust type that works really well here is the Range. We haven't covered Range yet but if you've been peeking
at the code samples throughout the last few chapters, you might have spotted a few!

Ranges allow you to specify a "closed" or "half open" range of numbers... kinda, see below.

> Actually, Ranges allow you to specify a range of anything so long as it implements the traits `PartialEq` and 
> `PartialOrd`. I've personally never seen this done for anything except numbers and characters, but it's worth pointing
> out. We'll talk more about PartialEq and PartialOrd in a later chapter.

We write Ranges in the form `start..end` where `start` is inclusive and `end` is `exclusive`. This means that `2..5`
includes 2 but not 5. If you want to create a range that includes the final number, prefix that number with `=`, e.g.
`2..=5`:

```rust
# fn main() {
let exclusive = 0..5;
let inclusive = 0..=5;
    
// This is another way of using variables in println!
// We use empty curly brackets as a positional marker
// and then fill those markers in with values after string slice
println!("Does exclusive range contain end: {}", exclusive.contains(&5)); 
println!("Does inclusive range contain end: {}", inclusive.contains(&5));
# }
```

As mentioned, Range's can be "half open" which means you can get away with specifying only the start or the end. This is
where the Type of the start and end really start to matter though.

```rust
# fn main() {
let u8_range = 0u8..; // this Range is explicitly defined with a u8
let i8_range = ..0i8; // this Range is defined with an i8
# }
```

A big warning here though: half open Ranges are dangerous when it comes to `for ... in ... ` loops. Ranges with no start
can't be used at all, and Ranges with no end will continue to try to produce numbers beyond the upper limits of the
type being used at which point your program will crash.

They're great though, if we just want to do something 10 times.

```rust
# fn main() {
for i in 0..10 {
    println!("Loop: {i}");
}
# }
```

Homework
--------

The best way to learn anything is to practice it. For this section, I'd like you create a program call Fizz Buzz.

In Fizz Buzz we want to run through a series of numbers (say 1 to 100 inclusive). For each number:
- if the number is divisible by 3, print the word Fizz
- if the number is divisible by 5, print the word Buzz
- if the number is divisible by both 3 and 5, print FizzBuzz
- otherwise, just print the number

You can do this a few ways, but you'll need to loop over each number and then choose what to do with it with those 
numbers. As a starting point, you could use a range to generate the numbers, then use a `for ... in ...` loop to get
each number one at a time, then some `if`/`else` statements to get the output.

Can you work out any other ways to do it?
