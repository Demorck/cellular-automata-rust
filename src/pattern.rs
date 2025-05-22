use std::ops::Not;
use crate::cell::Cell;

#[derive(Debug)]
pub struct Pattern {
    left_pattern: Vec<Cell>,
    center_pattern: Vec<Cell>,
}

impl Pattern {
    pub fn new(left_pattern: Vec<Cell>, center_pattern: Vec<Cell>) -> Pattern {
        Self {
            left_pattern,
            center_pattern,
        }
    }

    pub fn len(&self) -> usize {
        self.center_pattern.len()
    }

    pub fn next(&self, default_cell: Option<&Cell>) -> Pattern {
        let len = self.center_pattern.len();

        let cell_type = Cell::new(1);

        let ref center_ref = self.center_pattern;
        let ref left_ref = self.left_pattern;

        // Si centre contient au moins un "true"
        if center_ref.contains(&cell_type) {
            let mut result: Vec<Cell>  = vec![cell_type.clone(); len];
            let start_position = center_ref.iter().rposition(|p| p == &cell_type).unwrap();
            let mut last_cell = Cell::new(100);

            // Calculer la nouvelle ligne avec la règle 30
            for i in (1..=len).rev() {
                let index = (i + start_position) % len;
                let index_to = (i + start_position - 1) % len;

                if center_ref[index] == cell_type {
                    last_cell = !left_ref[index].clone();
                } else {
                    last_cell.set_state(last_cell.state() ^ left_ref[index].clone().state());
                }

                result[index_to] = last_cell.clone();
            }

            let new_pattern = Self::new(self.center_pattern.clone(), result);
            new_pattern
        } else {
            // Si le nombre de "true" dans gauche est impair, retourner None
            let number_one = left_ref.iter().filter(|&p| p == &cell_type).count();
            let new_size = len;
            let mut result: Vec<Cell> = vec![cell_type.clone(); new_size];

            let mut last_cell = cell_type.clone();
            result[len - 1] = last_cell.clone();

            // Calculer la nouvelle ligne
            for i in (0..len - 1).rev() {
                last_cell.set_state(last_cell.state() ^ left_ref[i + 1].clone().state());
                result[i] = last_cell.clone();
            }

            let mut left_pattern = self.center_pattern.clone();
            if number_one % 2 == 1 {
                let base = result.clone();
                let conjugue_result = result.into_iter().map(|p| p.not()).collect();
                result = [base, conjugue_result].concat();
                left_pattern = [left_pattern.clone(), left_pattern.clone()].concat();
            }

            let res = Pattern::new(left_pattern, result.clone());
            res
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        result.push_str("[");

        self.left_pattern.iter().for_each(|cell| {
            result.push_str(cell.to_string().as_str());
        });
        result.push_str(", ");
        self.center_pattern.iter().for_each(|cell| {
            result.push_str(cell.to_string().as_str());
        });

        result.push_str("]");


        result
    }

}