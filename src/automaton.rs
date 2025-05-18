use crate::line::Row;
use crate::rules::Rule;

pub struct Automaton {
    grid: Vec<Row>,
    iteration: usize,
    rule: Box<dyn Rule>
}

impl Automaton {

    pub fn new(first_row: Row, rule: Box<dyn Rule>) -> Self {
        Self {
            grid: vec![first_row],
            iteration: 1,
            rule,
        }
    }

    pub fn next(&mut self) {
        let last_line = self.grid.last().unwrap();
        let new_line = last_line.next(self.rule.as_ref());
        self.grid.push(new_line);
        self.iteration += 1;
    }

    pub fn evolve(&mut self, steps: u64)
    {
        for _ in 0..steps {
            self.next();
        }
    }

    pub fn grid(&self) -> &Vec<Row> {
        &self.grid
    }

    pub fn iteration(&self) -> usize {
        self.iteration
    }
}