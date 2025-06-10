use std::cmp::PartialEq;
use std::ops::Div;
use std::thread::current;
use crate::cell::Cell;
use crate::pattern::Pattern;
use crate::rules::{Rule, Rule30, WolframRule};
use crate::utils::is_periodic;

pub struct Diagonal {
    pattern: Vec<Cell>,
    transit: Vec<Cell>
}

impl Clone for Diagonal {
    fn clone(&self) -> Self {
        Diagonal {
            pattern: self.pattern.clone(),
            transit: self.transit.clone()
        }
    }
}

pub struct Fast30 {
    diagonals: Vec<Diagonal>,
    current_pattern: Pattern,
    current_period: usize,
    is_doubling: bool,
    iteration: usize
}


impl Diagonal {
    pub fn new(pattern: Vec<Cell>, transit: Vec<Cell>) -> Diagonal {
        Diagonal { pattern, transit }
    }

    pub fn get_last(&self, index: usize) -> Cell {
        if index < 0 {
            panic!("Index {} out of bounds", index);
        }

        if index <= self.transit.len() {
            self.transit[index - 1].clone()
        } else {
            let tau = self.transit.len();
            let pi = self.pattern.len();
            if pi == 0 {
                panic!("Diagonal pattern is empty");
            }

            let index = (index - 1 - tau) % pi;

            self.pattern[index].clone()
        }
    }

    pub fn has_state_in_pattern(&self, state: u8) -> bool {
        self.pattern.contains(&Cell::new(state))
    }

    pub fn count_state_in_pattern(&self, state: u8) -> usize {
        self.pattern.iter().filter(|&x| x == &Cell::new(state)).count()
    }

    pub fn to_string(&self) ->String {
        let mut result = String::new();

        let mut counter = 1;
        result.push_str("Diagonal ");
        result.push_str(counter.to_string().as_str());
        result.push_str(" - Pattern: ");
        self.pattern.iter().for_each(|cell| result.push_str(cell.to_string().as_str()));
        result.push_str(" - Transit: ");
        if !self.transit.is_empty()
        {
            let mut cloned = self.transit.clone();
            // let trailing_zeros = (counter as f64 / 2.0).ceil() as usize;
            // cloned.drain(0..trailing_zeros);
            cloned.iter().for_each(|cell| result.push_str(cell.to_string().as_str()));
        }
        result.push_str("\n");

        result
    }
}

impl Fast30 {
    pub fn new() -> Fast30 {
        let mut first_cell = Cell::new(1);
        first_cell.fix();
        let first_diag = Diagonal::new(vec![first_cell.clone()], vec![]);
        let second_diag = Diagonal::new(vec![first_cell], vec![Cell::new(0)]);
        let pattern = Pattern::new(vec!(Cell::new(0)), vec!(Cell::new(1)));
        Self {
            diagonals: vec![first_diag, second_diag],
            current_pattern: pattern,
            current_period: 1,
            is_doubling: false,
            iteration: 2
        }
    }

    pub fn next(&mut self)
    {
        let leading_zeros = self.iteration.div_ceil(2);
        let d_k1 = self.diagonals.last().unwrap();
        let d_k2 = self.diagonals.get(self.diagonals.len() - 2).unwrap();
        let mut result_transitoire: Vec<Cell> = Vec::new();
        let mut result_motif: Vec<Cell> = Vec::new();
        let mut result_complet = vec![Cell::new(0); leading_zeros];
        let on_double = !d_k1.has_state_in_pattern(1) && d_k2.count_state_in_pattern(1) % 2 == 1;
        if on_double {
            self.current_period *= 2;
            println!("On double à l'iteration: {}", self.iteration + 1);
        }

        let mut dans_periode_k1 = false;
        let mut dans_periode_k2 = false;
        let mut dans_periode_k = false;
        let mut j = 0;
        let mut i = leading_zeros;

        loop {
            if i > d_k1.transit.len() {
                dans_periode_k1 = true;
            }
            if i > d_k2.transit.len() {
                dans_periode_k2 = true;
            }
            if j == self.current_period {
                break;
            }

            let etat_centre = d_k1.get_last(i);

            let etat_gauche = d_k2.get_last(i);

            let etat_droite = result_complet.last().unwrap();

            let new_state = Rule30.apply(etat_gauche.state(), etat_centre.state(), etat_droite.state());
            let new_cell = Cell::new(new_state);
            // On est dans le cas général : période qui double pas + dans le motif du k-1 + centre fixé
            if dans_periode_k1 {
                if dans_periode_k2 && on_double || etat_centre.state() == 1 {
                    dans_periode_k = true;
                }
                j += 1;
                result_motif.push(new_cell.clone());
                // result_transitoire.push(new_cell);
            } else {
                result_transitoire.push(new_cell.clone());
            }

            i += 1;

            result_complet.push(new_cell);
        }

        let new_diagonal = Diagonal::new(result_motif, result_complet);
        let last = d_k1.clone();
        self.diagonals = vec![last, new_diagonal];

        // self.diagonals.push(new_diagonal);
        self.iteration += 1;
    }

    pub fn evolve(&mut self, steps: usize)
    {
        for i in 0..steps
        {
            self.next();
            if i % 1000 == 0 {
                println!("On arrive à {}", i);
            }
            // println!("{}", self.to_string());
        }
    }

    pub fn to_string(&self, with_transit: bool) -> String {
        let mut result = String::new();

        let mut counter = 1;
        for diagonal in &self.diagonals {
            result.push_str("Diagonal ");
            result.push_str(counter.to_string().as_str());
            result.push_str(" - Pattern: ");
            diagonal.pattern.iter().for_each(|cell| result.push_str(cell.to_string().as_str()));
            result.push_str(" - Transit: ");
            if !diagonal.transit.is_empty() && with_transit
            {
                let mut cloned = diagonal.transit.clone();
                // let trailing_zeros = (counter as f64 / 2.0).ceil() as usize;
                // cloned.drain(0..trailing_zeros);
                cloned.iter().for_each(|cell| result.push_str(cell.to_string().as_str()));
            }
            result.push_str("\n");
            counter += 1;

        }

        result
    }

    fn elude_last_transition(&self)
    {

    }
}