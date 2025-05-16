use crate::line::Row;
use crate::rules::Rule;

pub struct Automaton<R: Rule> {
    grid: Vec<Row<R>>,
    iteration: usize,
}

impl<R: Rule> Automaton<R> {

    pub fn next(&mut self) {
        let last_line = self.grid.last().unwrap();
        let new_line = last_line.next();
        self.grid.push(new_line);
        self.iteration += 1;
    }

    pub fn evolve(&mut self, steps: u64)
    {
        for _ in 0..steps {
            self.next();
        }
    }
}