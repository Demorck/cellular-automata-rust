#[derive(Clone)]
pub struct Cell {
    state: u8,
}

impl Cell {
    pub fn new(state: u8) -> Cell {
        Cell { state }
    }

    pub fn state(&self) -> u8 {
        self.state
    }

    pub fn set_state(&mut self, state: u8) {
        self.state = state;
    }

    pub fn display(&self) -> char {
        match self.state {
            0 => '.',
            1 => '#',
            _ => '?',
        }
    }
}