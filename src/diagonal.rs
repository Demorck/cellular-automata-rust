use std::cmp::max;
use std::mem::swap;
use crate::cell::Cell;
use crate::pattern::Pattern;

#[derive(Debug, PartialEq, Eq)]
pub struct Diagonal {
    pattern: Vec<u8>,
    transit: Vec<u8>
}

impl Clone for Diagonal {
    fn clone(&self) -> Self {
        Diagonal {
            pattern: self.pattern.clone(),
            transit: self.transit.clone()
        }
    }
}


impl Diagonal {
    pub fn new(pattern: Vec<u8>, transit: Vec<u8>) -> Diagonal {
        Diagonal { pattern, transit }
    }

    pub fn new_from_binary(transit: &str, pattern: &str) -> Diagonal {
        let pattern = pattern
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let transit = transit
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        Self::new(pattern, transit)
    }


    pub fn clear(&mut self) {
        self.transit.clear();
        self.pattern.clear();
    }

    pub fn push_transit(&mut self, state: u8)
    {
        self.transit.push(state);
    }

    pub fn push_pattern(&mut self, state: u8) {
        self.pattern.push(state);
    }

    pub fn set(&mut self, new: Diagonal)
    {
        self.transit = new.transit;
        self.pattern = new.pattern;
    }

    pub fn get_last(&self, index: usize) -> u8 {
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
        self.pattern.contains(&state)
    }

    pub fn count_state_in_pattern(&self, state: u8) -> usize {
        self.pattern.iter().filter(|&x| x == &state).count()
    }


    // [0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1] - [1, 0, 0, 1] => [1, 1, 0, 0,]
    pub fn elude_transit(&mut self)
    {
        let mut counter = 0;
        let tau = self.transit.len();
        let pi = self.pattern.len();
        let mut idx_transit = tau - 1;
        let mut idx_pattern = pi - 1;
        loop {
            if  self.transit.len() <= 1 || pi <= 1 {
                break;
            }
            let state_pattern = self.pattern.last().unwrap();
            let state_transit = self.transit.last().unwrap();

            if state_transit == state_pattern {
                self.transit.pop();
            } else {
                break;
            }

            // idx_pattern = if idx_pattern == 0 { self.pattern.len() - 1} else { idx_pattern - 1 };
            // idx_transit = if idx_transit == 0 { self.transit.len() - 1 } else { idx_transit - 1 };
            counter += 1;
            self.pattern.rotate_right(1);
        }
        //
        // let rotate = counter % self.pattern.len();
        // self.pattern.rotate_right(rotate);

        self.transit.append(&mut self.pattern.clone());
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str("\tTransit: ");
        for state in &self.transit {
            result.push_str(&format!("{}", state));
        }
        result.push('\n');

        result.push_str("\tPattern: ");
        for state in &self.pattern {
            result.push_str(&format!("{}", state));
        }
        result.push('\n');

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_in_binary() {
        let diagonal = Diagonal::new(vec![0, 1, 0, 1], vec![1, 0, 0, 1]);
        let binary = Diagonal::new_from_binary("1001", "0101");

        assert_eq!(diagonal.pattern, binary.pattern);
        assert_eq!(diagonal.transit, binary.transit);

        let binary = Diagonal::new_from_binary("01010101", "10011001");

        assert_ne!(diagonal.pattern, binary.pattern);
        assert_ne!(diagonal.transit, binary.transit);
    }

    #[test]
    fn test_has_state_in_pattern() {
        let diagonal = Diagonal::new_from_binary("0000", "1100");
        assert!(diagonal.has_state_in_pattern(1));

        let diagonal = Diagonal::new_from_binary("0000", "0000");
        assert!(!diagonal.has_state_in_pattern(1));
    }

    #[test]
    fn test_count_state_in_pattern() {
        let diagonal = Diagonal::new_from_binary("0000", "1100");
        assert_eq!(diagonal.count_state_in_pattern(1), 2);

        let diagonal = Diagonal::new_from_binary("0000", "0000");
        assert_eq!(diagonal.count_state_in_pattern(1), 0);
    }

    #[test]
    fn test_elude_transit() {
        let mut diagonal = Diagonal::new_from_binary("00101011001100", "1100");
        diagonal.elude_transit();
        let needed = Diagonal::new_from_binary("00101", "0110");
        assert_eq!(diagonal, needed);

        let mut diagonal = Diagonal::new_from_binary("0000000000000000", "0000");
        diagonal.elude_transit();
        let needed = Diagonal::new_from_binary("0", "0000");
        assert_eq!(diagonal, needed);

        let mut diagonal = Diagonal::new_from_binary("01101101101110101101010110011001100110", "0110");
        diagonal.elude_transit();
        let needed = Diagonal::new_from_binary("0110110110111010110101", "0110");
        assert_eq!(diagonal, needed)
    }
}

pub struct Fast30 {
    last_diagonal: Box<Diagonal>,
    penult_diagonal: Box<Diagonal>,
    current_diagonal: Box<Diagonal>,
    current_period: usize,
    iteration: usize,
    elude_diagonal_steps: usize
}

impl Fast30 {
    pub fn new() -> Self {
        let mut penult_diagonal = Diagonal::new(Vec::with_capacity(64), Vec::with_capacity(1_000_000));
        penult_diagonal.pattern.push(1);

        let mut last_diagonal=  Diagonal::new(Vec::with_capacity(64), Vec::with_capacity(1_000_000));
        last_diagonal.pattern.push(1);
        last_diagonal.transit.push(0);

        let current_diagonal = Diagonal::new(Vec::with_capacity(64), Vec::with_capacity(1_000_000));
        Self {
            last_diagonal: Box::new(last_diagonal),
            penult_diagonal: Box::new(penult_diagonal),
            current_diagonal: Box::new(current_diagonal),
            current_period: 1,
            iteration: 2,
            elude_diagonal_steps: 20
        }
    }

    fn next(&mut self)
    {
        if self.is_doubling() {
            self.current_period *= 2;
            // println!("On double à l'iteration: {}", self.iteration + 1);

            // self.elude_diagonals();
            // println!("{}", self.to_string())
        }

        let d_k1 = &self.last_diagonal;
        let tau_k1 = d_k1.transit.len();
        let pi_k1 = d_k1.pattern.len();

        let d_k2 = &self.penult_diagonal;
        let tau_k2 = d_k2.transit.len();
        let pi_k2 = d_k2.pattern.len();


        self.current_diagonal.transit.clear();
        self.current_diagonal.pattern.clear();

        self.current_diagonal.transit.push(0);

        let mut j = 0;
        let mut i = 1;

        let mut idx_k1 = 0;
        let mut idx_k2 = 0;

        let mut last_state = 0;

        while j < self.current_period {
            let etat_centre = if i <= tau_k1 {
                unsafe { *d_k1.transit.get_unchecked(i - 1) }
            } else {
                unsafe { *d_k1.pattern.get_unchecked(idx_k1) }
            };
            let etat_gauche = if i <= tau_k2 {
                unsafe { *d_k2.transit.get_unchecked(i - 1) }
            } else {
                unsafe { *d_k2.pattern.get_unchecked(idx_k2) }
            };

            last_state = etat_gauche ^ (etat_centre | last_state);

            self.current_diagonal.transit.push(last_state);

            if i > tau_k1 && i > tau_k2 {
                j += 1;
                self.current_diagonal.pattern.push(last_state);
            }

            if i > tau_k1 {
                idx_k1 += 1;
                if idx_k1 == pi_k1 {
                    idx_k1 = 0;
                }
            }

            if i > tau_k2 {
                idx_k2 += 1;
                if idx_k2 == pi_k2 {
                    idx_k2 = 0;
                }
            }

            i += 1;
        }


        swap(&mut self.penult_diagonal, &mut self.last_diagonal);
        swap(&mut self.last_diagonal, &mut self.current_diagonal);

        self.iteration += 1;
    }

    pub fn set_steps_elude(&mut self, steps: usize)
    {
        self.elude_diagonal_steps = steps;
    }

    fn is_doubling(&self) -> bool {
        !self.last_diagonal.has_state_in_pattern(1) && self.penult_diagonal.count_state_in_pattern(1) % 2 == 1
    }

    pub fn evolve(&mut self, steps: usize)
    {
        for _ in 0..steps
        {
             self.next();
            if self.iteration % self.elude_diagonal_steps == 0 {
                self.elude_diagonals(false);
            }
        }
    }

    pub fn elude_diagonals(&mut self, log: bool)
    {
        if log {
            println!("Eluding diagonals at iteration: {}", self.iteration);
            println!("Before: {}", self.to_string());
        }
        self.last_diagonal.elude_transit();
        self.penult_diagonal.elude_transit();
        if log { println!("After: {}", self.to_string()); }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str("Last Diagonal:\n");
        result.push_str(self.last_diagonal.to_string().as_str());

        result.push_str("Penultimate Diagonal:\n");
        result.push_str(self.penult_diagonal.to_string().as_str());

        result
    }
}
