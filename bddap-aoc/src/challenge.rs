pub struct Challenge {
    pub year: usize,
    pub day: usize,
    pub part: usize,
    pub name: &'static str,
    pub run: fn(&str) -> String,
}

impl Challenge {
    pub fn run(&self, input: &str) -> String {
        (self.run)(input)
    }
}
