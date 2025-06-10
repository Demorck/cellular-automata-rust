use std::ops::{BitOr, BitXor, Not};

#[derive(Clone, Debug)]
pub struct Cell {
    state: u8,
    is_fixed: bool,
}

impl Not for Cell {
    type Output = Cell;

    fn not(self) -> Self::Output {
        if self.state == 0 { Cell::new(1) } else { Cell::new(0) }
    }
}


impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl BitOr for Cell {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let new_state = self.state | rhs.state;
        Self::new(new_state)
    }
}

impl BitXor for Cell {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let new_state = self.state ^ rhs.state;
        Self::new(new_state)

    }
}


impl Cell {
    pub fn new(state: u8) -> Cell {
        Cell {
            state,
            is_fixed: false,
        }
    }

    pub fn state(&self) -> u8 {
        self.state
    }

    pub fn set_state(&mut self, state: u8) {
        self.state = state;
    }

    pub fn is_fixed(&self) -> bool {
        self.is_fixed
    }

    pub fn fix(&mut self) {
        self.is_fixed = true
    }

    pub fn display(&self) -> char {
        match self.state {
            0 => '.',
            1 => '#',
            _ => '?',
        }
    }

    pub fn to_string(&self) -> String {
        self.state.to_string()
    }
}