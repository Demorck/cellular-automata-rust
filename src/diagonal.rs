use std::fs::OpenOptions;
use std::io::Write;
use std::mem::swap;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
pub struct Diagonal {
    pattern: Vec<u8>,
    transit: Vec<u8>,
    leading_zeros: usize,
}

impl Clone for Diagonal {
    fn clone(&self) -> Self {
        Diagonal {
            pattern: self.pattern.clone(),
            transit: self.transit.clone(),
            leading_zeros: 0
        }
    }
}


impl Diagonal {
    pub fn new(pattern: Vec<u8>, transit: Vec<u8>) -> Diagonal {
        Diagonal { pattern, transit, leading_zeros: 0 }
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
        self.leading_zeros = new.leading_zeros;
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

    pub fn elude_transit(&mut self)
    {
        let pi = self.pattern.len();
        loop {
            if  self.transit.len() <= 1 {
                break;
            }
            let state_pattern = self.pattern.last().unwrap();
            let state_transit = self.transit.last().unwrap();

            if state_transit == state_pattern {
                self.transit.pop();
            } else {
                break;
            }
            self.pattern.rotate_right(1);
        }

        self.transit.append(&mut self.pattern.clone());
    }

    pub fn get_from_index(&self, i: usize) -> u8 {
        let tau = self.transit.len();
        let pi = self.pattern.len();
        let zeta = self.leading_zeros;

        if i <= zeta { 0 } else if i <= tau + zeta {
            let k = i - zeta - 1;
            *self.transit.get(k).unwrap()
        } else {
            let k = (i - tau - zeta - 1) % pi;
            *self.pattern.get(k).unwrap()
        }
    }

    pub fn set_leading_zeros(&mut self, zeros: usize) {
        self.leading_zeros = zeros;
    }

    pub fn leading_zeros(&self) -> usize {
        self.leading_zeros
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
        assert_eq!(diagonal, needed);


        // let mut diagonal = Diagonal::new_from_binary("000000000000000000000001101101101110101101010110011001100110", "0110");
        // diagonal.elude_transit();
        // let needed = Diagonal::new_from_binary("0110110110111010110101", "0110");
        // assert_eq!(diagonal, needed)
    }

    #[test]
    fn test_get_from_index_without_leading() {
        let diagonal = Diagonal::new_from_binary("0000101010101010", "11001100");
        assert_eq!(diagonal.get_from_index(1), 0);
        assert_eq!(diagonal.get_from_index(5), 1);
        assert_eq!(diagonal.get_from_index(10), 0);
        assert_eq!(diagonal.get_from_index(15), 1);
        assert_eq!(diagonal.get_from_index(20), 0);
        assert_eq!(diagonal.get_from_index(25), 1);
    }

    #[test]
    fn test_get_from_index_with_leading() {
        let mut diagonal = Diagonal::new_from_binary("101010101010", "11001100");
        diagonal.set_leading_zeros(4);
        assert_eq!(diagonal.get_from_index(1), 0);
        assert_eq!(diagonal.get_from_index(5), 1);
        assert_eq!(diagonal.get_from_index(10), 0);
        assert_eq!(diagonal.get_from_index(15), 1);
        assert_eq!(diagonal.get_from_index(20), 0);
        assert_eq!(diagonal.get_from_index(25), 1);
    }
}

pub struct Fast30 {
    last_diagonal: Box<Diagonal>,
    penult_diagonal: Box<Diagonal>,
    current_diagonal: Box<Diagonal>,
    current_period: usize,
    iteration: usize,
    elude_diagonal_steps: usize,

    save_steps: usize,
    start_time: Instant,
    path_to_file: String,
    last_save: Option<Instant>,

    logging: bool,
    logging_steps: usize,

    transit: bool,
    transit_steps: usize,
    transit_vec: Vec<u32>,
    last_transit: usize
}

impl Fast30 {
    pub fn new() -> Self {
        let mut penult_diagonal = Diagonal::new(Vec::with_capacity(64), Vec::with_capacity(1_000_000));
        penult_diagonal.pattern.push(1);

        let mut last_diagonal=  Diagonal::new(Vec::with_capacity(64), Vec::with_capacity(1_000_000));
        last_diagonal.pattern.push(1);
        last_diagonal.leading_zeros = 0;

        let transit = Vec::with_capacity(1_000);

        let current_diagonal = Diagonal::new(Vec::with_capacity(64), Vec::with_capacity(1_000_000));
        Self {
            last_diagonal: Box::new(last_diagonal),
            penult_diagonal: Box::new(penult_diagonal),
            current_diagonal: Box::new(current_diagonal),
            current_period: 1,
            iteration: 2,
            elude_diagonal_steps: 1,

            save_steps: 100_000,
            path_to_file: "output/diagonal.txt".to_string(),
            start_time: Instant::now(),
            last_save: None,

            logging: true,
            logging_steps: 1_000_000,

            transit: true,
            transit_steps: 1_000,
            transit_vec: transit,
            last_transit: 0
        }
    }

    fn next(&mut self)
    {
        if self.is_doubling() {
            self.current_period *= 2;
            println!("On double à l'iteration: {}", self.iteration + 1);

            // self.elude_diagonals();
            // println!("{}", self.to_string())
        }

        let d_k1 = &self.last_diagonal;
        let tau_k1 = d_k1.transit.len();
        let zeta_k1 = d_k1.leading_zeros;

        let d_k2 = &self.penult_diagonal;
        let tau_k2 = d_k2.transit.len();
        let zeta_k2 = d_k2.leading_zeros;

        self.current_diagonal.transit.clear();
        self.current_diagonal.pattern.clear();

        let number_zeros = (self.iteration + 1) / 2;
        self.current_diagonal.set_leading_zeros(number_zeros);

        let mut j = 0;
        let mut i = number_zeros;

        let mut last_state = 0;

        while j < self.current_period {
            let etat_centre = d_k1.get_from_index(i);
            let etat_gauche = d_k2.get_from_index(i);

            last_state = etat_gauche ^ (etat_centre | last_state);

            self.current_diagonal.transit.push(last_state);

            if i > tau_k1 + zeta_k1 && i > tau_k2 + zeta_k2 {
                j += 1;
                self.current_diagonal.pattern.push(last_state);
            }

            i += 1;
        }

        // println!("[{}] New diagonal: {}", self.iteration, self.current_diagonal.to_string());
        swap(&mut self.penult_diagonal, &mut self.last_diagonal);
        swap(&mut self.last_diagonal, &mut self.current_diagonal);
    }

    pub fn set_steps_elude(&mut self, steps: usize) -> &mut Self
    {
        self.elude_diagonal_steps = steps;
        self
    }

    pub fn set_transit_steps(&mut self, steps: usize) -> &mut Self
    {
        self.transit_steps = steps;
        let new_vec = Vec::with_capacity(self.transit_steps);
        self.transit_vec = new_vec;
        self
    }

    fn is_doubling(&self) -> bool {
        !self.last_diagonal.has_state_in_pattern(1) && self.penult_diagonal.count_state_in_pattern(1) % 2 == 1
    }

    pub fn evolve(&mut self, steps: usize)
    {
        for _ in 0..steps
        {
            self.next();
            self.iteration += 1;

            if self.iteration % 2 == 0 {
                self.elude_diagonals(false);
            }

            if self.iteration % self.save_steps == 0 {
                self.save_to_file();
            }

            if self.logging && self.iteration % self.logging_steps == 0 {
                println!("Iteration: {}", self.iteration);
            }

            if self.transit {
                self.transit_vec.push((self.last_diagonal.transit.len() - self.current_period) as u32);
            }

            if self.iteration % self.transit_steps == 0 {
                self.save_transit();
                self.transit_vec.clear();
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
        result.push_str("Number of leading zeros: ");
        result.push_str(&self.last_diagonal.leading_zeros.to_string());
        result.push_str("\n");

        result.push_str("Penultimate Diagonal:\n");
        result.push_str(self.penult_diagonal.to_string().as_str());
        result.push_str("Number of leading zeros: ");
        result.push_str(&self.penult_diagonal.leading_zeros.to_string());

        result
    }

    fn save_to_file(&mut self) {
        use std::fs::OpenOptions;
        use std::io::Write;

        let now = Instant::now();

        let time_since_start = now.duration_since(self.start_time).as_secs();
        let time_since_last = self.last_save.map_or(0, |prev| now.duration_since(prev).as_secs());

        self.last_save = Some(now);

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.path_to_file)
            .expect("Unable to open file");

        let content = format!(
            "Iteration: {} | since start: {}s | since last: {}s\n{}\n",
            self.iteration,
            time_since_start,
            time_since_last,
            self.to_string()
        );

        file.write_all(content.as_bytes())
            .expect("Unable to write to file");
    }

    fn save_transit(&mut self) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("output/transit.txt")
            .expect("Unable to open file");

        let mut content = "i;transit;diff\n".to_string();
        let mut i = self.iteration - self.transit_steps;
        let mut k = 0;
        for transit in &self.transit_vec {
            let delta = if i == 0 { 0 } else {
                if k == 0 {
                    self.last_transit as i32
                } else {
                    (*transit as i32) - (*self.transit_vec.get(k - 1).unwrap()) as i32
                }
            };

            content.push_str(&format!("{};{};{}\n", i, transit, delta));
            i += 1;
            k += 1;
        }

        file.write_all(content.as_bytes())
            .expect("Unable to write to file");
    }

}
