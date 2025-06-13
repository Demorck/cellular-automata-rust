use std::collections::BTreeMap;
use crate::automaton::Automaton;
use crate::cell::Cell;
use crate::utils::is_periodic;

pub enum DIAGONAL {
    LEFT,
    RIGHT
}


pub struct AutomatonAnalysis<'a> {
    automaton: &'a Automaton,

    diagonals_left: Vec<Vec<&'a Cell>>,
    diagonals_right: Vec<Vec<&'a Cell>>,

    current_period_left: usize,
    current_period_right: usize,

    multiplier_left: usize,
    multiplier_right: usize,
}

impl<'a> AutomatonAnalysis<'a> {
    pub fn new(automaton: &'a Automaton) -> Self {
        Self {
            automaton,

            diagonals_left: vec![],
            diagonals_right: vec![],

            current_period_left: 1,
            current_period_right: 1,

            multiplier_left: 1,
            multiplier_right: 1
        }
    }

    /// Extrait les diagonales et retourne un `Vec<u8>` pour chaque diagonale
    pub fn extract_diagonals(
        &mut self,
        diagonal: DIAGONAL,
    ) {
        let iteration = self.automaton.iteration();

        for i in 1..iteration {
            let diag = self.extract_diagonal(i, &diagonal);
            match diag {
                None => { break; }
                Some(d) => {
                    match diagonal {
                        DIAGONAL::LEFT => { self.diagonals_left.push(d); }
                        DIAGONAL::RIGHT => { self.diagonals_right.push(d);}
                    }
                }
            }
        }
    }

    fn extract_diagonal(
        &self,
        n: usize,
        diagonal: &DIAGONAL,
    ) -> Option<Vec<&'a Cell>> {
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
                    result.push(cell);
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
        for i in 0..self.automaton.max_iteration() {
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

    pub fn extract_patterns(&mut self, type_diagonal: DIAGONAL, offset_f: fn(usize) -> usize) -> Vec<(Vec<&Cell>, usize, usize)>
    {
        let mut result = Vec::new();

        let diagonals = match type_diagonal {
            DIAGONAL::LEFT => { &self.diagonals_left }
            DIAGONAL::RIGHT => { &self.diagonals_right }
        };

        let mut period = match type_diagonal {
            DIAGONAL::LEFT => { self.current_period_left }
            DIAGONAL::RIGHT => { self.current_period_right }
        };

        let mut breaked = false;

        for i in 0..diagonals.len() {
            if breaked { break; }
            let current_diagonal = diagonals.get(i).unwrap();
            let offset = offset_f(i);

            loop {
                let pattern = self.find_pattern(current_diagonal.clone(), offset, period);

                match pattern {
                    (None, None, None) => {
                        period *= 2;
                        if (offset + period) > current_diagonal.len() {
                            breaked = true;
                            break;
                        }
                    }
                    (Some(p), Some(o), Some(t)) => {
                        result.push((p, o, t));
                        break;
                    }
                    (_, _, _) => {
                        // panic!("AAAAAAAAAAAa")
                        break;
                    }
                }
            }
        }

        result
    }



    fn find_pattern<'b>(&self, mut diagonal: Vec<&'b Cell>, start_offset: usize, period: usize) -> (Option<Vec<&'b Cell>>, Option<usize>, Option<usize>) {
        assert!(period > 0, "find_pattern: Period should be stricly positive");
        let mut offset = start_offset;

        if diagonal.len() <= start_offset {
            return (None, None, None);
        }
        diagonal.drain(0..start_offset);

        while diagonal.len() > 2 * period
        {
            if is_periodic(&diagonal, period) {
                diagonal.truncate(period);
                return (Some(diagonal), Some(period), Some(offset));
            }
            diagonal.remove(0);
            offset += 1;
        }

        (None, None, None)
    }
}
