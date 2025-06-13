pub trait Rule {
    fn apply(&self, left: u8, center: u8, right: u8) -> u8;

    fn states(&self) -> u64;
}

pub struct Rule30;

impl Rule for Rule30 {
    fn apply(&self, left: u8, center: u8, right: u8) -> u8 {
        left ^ (center | right)
    }

    fn states(&self) -> u64 {
        todo!()
    }
}

pub struct WolframRule {
    rule_number: u8,
}

impl WolframRule {
    pub fn new(rule_number: u8) -> Self {
        Self { rule_number }
    }
}

impl Rule for WolframRule {

    fn apply(&self, left: u8, center: u8, right: u8) -> u8 {
        let index = (left << 2) | (center << 1) | right;
        let bit = (self.rule_number >> index) & 1;

        bit
    }

    fn states(&self) -> u64 {
        self.rule_number as u64
    }
}