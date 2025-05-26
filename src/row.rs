use crate::cell::Cell;
use crate::rules::Rule;

pub struct Row {
    configuration: Vec<Cell>,
    iteration: usize
}

impl Row {
    pub fn clone(&self) -> Row {
        Self {
            configuration: self.configuration.clone(),
            iteration: self.iteration
        }
    }
}

impl Row {
    pub fn len(&self) -> usize {
        self.configuration.len()
    }

    pub fn get(&self, index: usize) -> Option<&Cell> {
        self.configuration.get(index)
    }

    pub fn new(configuration: Vec<Cell>) -> Row {
        Self { configuration, iteration: 0}
    }

    pub fn next(&self, rule: &dyn Rule) -> Row {
        let mut next_configuration= Vec::with_capacity(self.configuration.len());
        for i in 0..self.configuration.len() {
            let left = if i == 0 { 0 } else { i - 1 };
            let center = i;
            let right = if i == self.configuration.len() - 1 { 0 } else { i + 1 };

            let new_state = rule.apply(
                self.configuration[left].state(),
                self.configuration[center].state(),
                self.configuration[right].state()
            );

            next_configuration.push(Cell::new(new_state));
        }

        Self::new(next_configuration)
    }

    pub fn to_string(&self) -> String {
        self.configuration.iter().map(|c| c.display()).collect()
    }
}