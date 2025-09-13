use std::pin::pin;

struct HorribleExampleOfSelfReference {
    value: usize,
    reference_to_value: Option<*const usize>,
}

impl HorribleExampleOfSelfReference {
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
    let mut example = HorribleExampleOfSelfReference::new(1);

    // We need to set the reference now as the constructor moves the data too!
    example.set_reference();

    let pinned_example = pin!(example);

    // We can still read the value
    assert_eq!(pinned_example.get_value(), 1);

    // But we can no longer mutate it
    // example.value = 2;
    // pinned_example.value = 2;

    // Or move the underlying data
    // let example = example;
}
