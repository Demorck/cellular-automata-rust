use std::cmp::PartialEq;
use crate::cell::Cell;
use crate::pattern::Pattern;

pub struct Diagonal {
    pattern: Vec<Cell>,
    transit: Vec<Cell>,
}

pub struct Fast30 {
    diagonals: Vec<Diagonal>,
    current_pattern: Pattern,
    current_period: usize,
    is_doubling: bool,
}


impl Diagonal {
    pub fn new(pattern: Vec<Cell>, transit: Vec<Cell>) -> Diagonal {
        Diagonal { pattern, transit }
    }
}

impl Fast30 {
    pub fn new() -> Fast30 {
        let mut first_cell = Cell::new(1);
        first_cell.fix();
        let first_diag = Diagonal::new(vec![first_cell.clone()], vec![]);
        let second_diag = Diagonal::new(vec![first_cell], vec![]);
        let pattern = Pattern::new(vec!(Cell::new(0)), vec!(Cell::new(1)));
        Self {
            diagonals: vec![first_diag, second_diag],
            current_pattern: pattern,
            current_period: 1,
            is_doubling: false
        }
    }

    pub fn next(&mut self)
    {
        let last_diag = self.diagonals.last().unwrap();
        // let len = self.diagonals.len() as isize;
        let penultimate_diag = self.diagonals.get(self.diagonals.len() - 2).unwrap();
        let is_current_diag_even = self.diagonals.len() % 2 == 0; // so the current one is even if the last is odd
        let mut current_trans: Vec<Cell> = vec![];
        let mut need_to_fix = false;

        let get_cell = |index: usize, diag: &Diagonal| {
            /// Si on veut le i-ème bit de la diagonale, on regarde par rapport à la période de transition
            /// Si le i-ème bit est au delà, alors on commence le motif
            /// Si pendant le motif, on tombe sur un 1, ça fixe le motif actuel
            return if index < diag.transit.len() {
                diag.transit.get(index).unwrap().clone()
            } else {
                let cell = diag.pattern.get(index % diag.pattern.len()).unwrap();
                cell.clone()
            };
        };

        loop {
            let index = match is_current_diag_even {
                true => { current_trans.len() }
                false => {
                    if (current_trans.len() as isize - 1) < 0 {
                        0
                    } else { current_trans.len() - 1 }
                }
            };
            let center_cell = if !is_current_diag_even && index == 0 { Cell::new(0) } else {
                let mut cell = get_cell(index, last_diag);
                if cell.state() == 1 && cell.is_fixed() || self.is_doubling {
                    need_to_fix = true;
                    cell.fix();
                    self.is_doubling = false;
                }

                cell
            };
            let left_cell = get_cell(current_trans.len(), penultimate_diag);
            let right_cell = if current_trans.is_empty() { Cell::new(0) } else { current_trans.last().unwrap().clone() };

            /// Si c'est fixé, ça veut dire qu'on est dans le motif du précédent ET qu'il y a eu un 1.
            /// Le motif est fixé, on s'arrête là
            if need_to_fix {
                self.current_pattern = self.current_pattern.next(Some(&Cell::new(1)));
                if !self.current_pattern.contains(&Cell::new(1))
                {
                    self.current_period *= 2;
                    self.is_doubling = true;
                }

                let current_diag = Diagonal::new(self.current_pattern.get_center(), current_trans);
                self.diagonals.push(current_diag);
                break;
            }

            /// Si le motif n'est pas fixé, on est soit en période de transition, on applique la règle
            let new_cell = left_cell ^ (center_cell | right_cell);
            current_trans.push(new_cell);
        }

    }

    pub fn evolve(&mut self, steps: usize)
    {
        for _ in 0..steps
        {
            self.next();
        }
    }

    // pub fn to_string(&self) ->String {
    //     let mut result = String::new();
    //     let max_len = self.grid.last().unwrap().len();
    //     for line in self.grid.iter().rev() {
    //         let space_to_add = (max_len - line.len()) / 2;
    //
    //         for _ in 0..space_to_add {
    //             result.push(' ');
    //         }
    //
    //         line.iter().for_each(|cell|{
    //             result.push_str(&cell.to_string());
    //         });
    //
    //         for _ in 0..space_to_add {
    //             result.push(' ');
    //         }
    //
    //         result.push_str("\n");
    //     }
    //
    //     result
    // }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        let mut counter = 1;
        for diagonal in &self.diagonals {
            result.push_str("Diagonal ");
            result.push_str(counter.to_string().as_str());
            result.push_str(" - Pattern: ");
            diagonal.pattern.iter().for_each(|cell| result.push_str(cell.to_string().as_str()));
            result.push_str(" - Transit: ");
            diagonal.transit.iter().for_each(|cell| result.push_str(cell.to_string().as_str()));
            result.push_str("\n");
            counter += 1;

        }

        result
    }
}