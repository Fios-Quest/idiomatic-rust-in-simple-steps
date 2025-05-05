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
