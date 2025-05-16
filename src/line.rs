use crate::cell::Cell;
use crate::rules::Rule;

pub struct Row<R: Rule> {
    configuration: Vec<Cell>,
    iteration: usize,
}

impl <R: Rule> Row<R> {
    pub fn new(configuration: Vec<Cell>) -> Row<R> {
        Self { configuration, iteration: 0 }
    }

    pub fn next(&self) -> Row<R> {
        let mut next_configuration= Vec::with_capacity(self.configuration.len());
        for i in 0..self.configuration.len() {
            let left = if i == 0 { 0 } else { i - 1 };
            let center = i;
            let right = if i == self.configuration.len() - 1 { 0 } else { i + 1 };

            let new_state = R::apply(
                self.configuration[left].state(),
                self.configuration[center].state(),
                self.configuration[right].state()
            );

            next_configuration[center] = Cell::new(new_state);
        }

        Self::new(next_configuration)
    }
}