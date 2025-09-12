pub struct SystemTime {
    seconds: f64,
}

impl SystemTime {
    pub fn new() -> Self {
        Self { seconds: 0. }
    }
    fn count(&mut self) {
        for i in 0..3600 {
            self.seconds += i as f64
        }
    }
}
