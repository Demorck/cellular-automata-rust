use std::collections::BTreeMap;
use crate::automaton::Automaton;
use crate::cell::Cell;

pub enum DIAGONAL {
    LEFT,
    RIGHT
}


pub struct AutomatonAnalysis<'a> {
    automaton: &'a Automaton,
    multiplier_left: usize,
    multiplier_right: usize,
}

impl<'a> AutomatonAnalysis<'a> {
    pub fn new(automaton: &'a Automaton) -> Self {
        Self {
            automaton,
            multiplier_left: 1,
            multiplier_right: 1
        }
    }

    /// Extrait les diagonales et retourne un Vec<u8> pour chaque diagonale
    pub fn extract_diagonals(
        &self,
        diagonal: DIAGONAL,
    ) -> Vec<Vec<u8>> {
        let iteration = self.automaton.iteration();
        let mut diagonals = Vec::new();

        for i in 1..iteration {
            let diag = self.extract_diagonal(i, &diagonal);
            match diag {
                None => { break; }
                Some(d) => { diagonals.push(d); }
            }
        }

        diagonals
    }

    fn extract_diagonal(
        &self,
        n: usize,
        diagonal: &DIAGONAL,
    ) -> Option<Vec<u8>> {
        let grid = self.automaton.grid();
        let iteration = self.automaton.iteration();
        let middle = (self.automaton.col() - 1)/2;
        let mut result = Vec::new();

        if n > middle { return None; }

        let multiplier = match diagonal {
            DIAGONAL::LEFT => { self.multiplier_left },
            DIAGONAL::RIGHT => { self.multiplier_right },
        };

        let mut offset = 0;

        for i in (n..iteration).step_by(multiplier) {
            if n + offset > middle || (middle + offset) > self.automaton.col()
            {
                break;
            }

            let col = match diagonal {
                DIAGONAL::LEFT => middle - offset,
                DIAGONAL::RIGHT => middle + offset
            };

            if let Some(row) = grid.get(i) {
                if col < row.len() {
                    let cell = row.get(col).unwrap();
                    result.push(cell.state());
                }
            }

            offset += 1;
        }

        Some(result)
    }


    pub fn rightmost_same_state(&self, cell_type: Cell) -> BTreeMap<u16, u16>
    {
        let middle = (self.automaton.col() - 1)/2;
        let mut result: BTreeMap<u16, u16> = BTreeMap::new();
        for i in 1..self.automaton.max_iteration() {
            let mut counter: u16 = 1;
            loop {
                let index = middle + i - counter as usize;
                let current_row = self.automaton.grid().get(i).unwrap();
                let current_cell = match current_row.get(index) {
                    None => {
                        println!("Cell not found: {} at row: {} and counter {}", index, i, counter);
                        break;
                    }
                    Some(p) => { p }
                };

                if current_cell.state() == cell_type.state() {
                    counter += 1;
                } else {
                    break;
                }
            }

            if !result.contains_key(&(counter)) { result.insert(counter, i as u16); };
        }

        result
    }
}
