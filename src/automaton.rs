use std::cmp::max;
use crate::row::Row;
use crate::rules::Rule;

pub struct Automaton {
    grid: Vec<Row>,
    iteration: usize,
    rule: Box<dyn Rule>,
    col: usize,
}

impl Automaton {
    pub fn new(first_row: Row, rule: Box<dyn Rule>) -> Self {
        let col = first_row.len();
        Self {
            grid: vec![first_row],
            iteration: 1,
            rule,
            col
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

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn max_iteration(&self) -> usize {
        max(self.col, self.iteration)
    }

    pub fn to_string(&self) -> String {
        self.grid.iter().map(|r| r.to_string() + "\n").collect()
    }
}