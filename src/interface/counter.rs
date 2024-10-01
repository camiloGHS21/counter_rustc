pub struct Counter {
    pub value: u64,
}
impl Default for Counter {
    fn default() -> Self {
        Counter { value: 0 }
    }
}

impl Counter {
    pub fn increment(&mut self) {
        self.value += 1;
    }
    pub fn decrement(&mut self) {
        if self.value > 0 {
            self.value -= 1;
        }
    }
}
