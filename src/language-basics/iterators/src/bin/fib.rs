// ANCHOR: boilerplate
// ANCHOR: struct
struct Fibonacci {
    previous: u8,
    next: Option<u8>,
}

impl Fibonacci {
    fn new() -> Self {
        Self {
            previous: 0,
            next: Some(1),
        }
    }
}
// ANCHOR_END: struct

// ANCHOR: impl_iterator
impl Iterator for Fibonacci {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // Store "current" value (we're going to overwrite it)
        let current = self.next?;

        // Update internal values
        self.next = current.checked_add(self.previous);
        self.previous = current;

        // Return the "current" value
        Some(current)
    }
}
// ANCHOR_END: impl_iterator
// ANCHOR_END: boilerplate

fn main() {
    // ANCHOR: next
    let mut fib = Fibonacci::new();

    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));
    assert_eq!(fib.next(), Some(8));
    // ANCHOR_END: next

    // ANCHOR: last
    let fib = Fibonacci::new();
    assert_eq!(fib.last(), Some(233));
    // ANCHOR_END: last

    // ANCHOR: loop
    for f in Fibonacci::new() {
        println!("{f}");
    }
    // ANCHOR_END: loop

    // ANCHOR: enumerate
    for (i, f) in Fibonacci::new().enumerate() {
        println!("{i}: {f}");
    }
    // ANCHOR_END: enumerate

    // ANCHOR: take
    for (i, f) in Fibonacci::new().enumerate().take(4) {
        println!("{i}: {f}");
    }
    // ANCHOR_END: take
}
