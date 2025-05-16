pub trait Rule {
    fn apply(left: u8, center: u8, right: u8) -> u8;

    fn states(&self) -> u64;
}

struct Rule30 {}

impl Rule for Rule30 {
    fn apply(left: u8, center: u8, right: u8) -> u8 {
        left ^ (center | right)
    }

    fn states(&self) -> u64 {
        todo!()
    }
}