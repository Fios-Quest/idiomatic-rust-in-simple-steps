use std::pin::Pin;

struct ExampleOfSelfReference {
    value: usize,
    reference_to_value: Option<*const usize>,
}

impl ExampleOfSelfReference {
    fn new(value: usize) -> Self {
        Self {
            value,
            reference_to_value: None,
        }
    }

    fn set_reference(&mut self) {
        self.reference_to_value = Some(&raw const self.value);
    }

    fn get_value(&self) -> usize {
        // SAFETY: This is intentionally NOT safe, don't try this at home!
        unsafe { *self.reference_to_value.expect("Did not set_reference") }
    }
}

fn main() {
    let mut example = ExampleOfSelfReference::new(1);
    example.set_reference();

    // Pin doesn't take ownership of the data, it takes a mutable reference to it
    let mut pinned_example = Pin::new(&mut example);

    // We can still read the value thanks to Deref
    assert_eq!(pinned_example.get_value(), 1);

    // But we can no longer mutate it
    // example.value = 2;
    // pinned_example.value = 2;

    // Or move the underlying data
    // let example = example;

    // We can, however, access the original data via a mutable reference
    pinned_example.as_mut().value = 2;
    assert_eq!(pinned_example.get_value(), 2);
}
