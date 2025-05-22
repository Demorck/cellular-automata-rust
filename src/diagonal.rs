use crate::cell::Cell;

pub struct Diagonal {
    pattern: Vec<Cell>,
    transit: Vec<Cell>,
}

pub struct Fast30 {
    grid: Vec<Vec<Cell>>
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
        Self {
            grid: vec![vec![first_cell]],
        }
    }

    pub fn next(&mut self) {
        let last_len = self.grid.last().unwrap().len();
        let new_len = last_len + 2;
        let mut result = vec![Cell::new(0); new_len];
        let middle_index = (new_len - 1) / 2;

        for i in (0..new_len).rev() {
            let offset_from_middle = i.abs_diff(middle_index);
            let left_index = (i - 2) as isize;
            let center_index = (i - 1) as isize;
            let right_index = i;

            let left_cell = if left_index < 0 { Cell::new(0) } else { self.grid.last().unwrap()[left_index as usize].clone() };
            let center_cell = if i == 0 || i == new_len { Cell::new(0) } else { self.grid.last().unwrap()[center_index as usize].clone() };
            let right_cell = if right_index >= new_len { Cell::new(0) } else { self.grid.last().unwrap()[right_index].clone() };




        }
    }
}