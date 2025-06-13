use std::mem::swap;

pub struct Diagonal {
    pattern: Vec<u8>,
    transit: Vec<u8>
}

impl Diagonal {
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
    last_diagonal: Box<Diagonal>,
    penult_diagonal: Box<Diagonal>,
    current_diagonal: Box<Diagonal>,
    current_period: usize,
    iteration: usize
}


impl Diagonal {
    pub fn new(pattern: Vec<u8>, transit: Vec<u8>) -> Diagonal {
        Diagonal { pattern, transit }
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
            iteration: 2
        }
    }

    fn next(&mut self)
    {
        let d_k1 = &self.last_diagonal;
        let tau_k1 = d_k1.transit.len();
        let pi_k1 = d_k1.pattern.len();

        let d_k2 = &self.penult_diagonal;
        let tau_k2 = d_k2.transit.len();
        let pi_k2 = d_k2.pattern.len();

        if self.is_doubling() {
            self.current_period *= 2;
            // println!("On double à l'iteration: {}", self.iteration + 1);
        }

        self.current_diagonal.transit.clear();
        self.current_diagonal.pattern.clear();

        // self.current_diagonal.transit.reserve(100_000);
        // self.current_diagonal.pattern.reserve(self.current_period);

        self.current_diagonal.transit.push(0);

        let mut j = 0;
        let mut i = 1;

        let mut last_state = 0;

        while j < self.current_period {
            let etat_centre = if i <= tau_k1 {
                unsafe { *d_k1.transit.get_unchecked(i - 1) }
            } else {
                unsafe { *d_k1.pattern.get_unchecked((i - 1 - tau_k1) % pi_k1) }
            };
            let etat_gauche = if i <= tau_k2 {
                unsafe { *d_k2.transit.get_unchecked(i - 1) }
            } else {
                unsafe { *d_k2.pattern.get_unchecked((i - 1 - tau_k2) % pi_k2) }
            };

            last_state = etat_gauche ^ (etat_centre | last_state);

            self.current_diagonal.transit.push(last_state);

            if i > tau_k1 {
                j += 1;
                self.current_diagonal.pattern.push(last_state);
            }
            i += 1;
        }


        swap(&mut self.penult_diagonal, &mut self.last_diagonal);
        swap(&mut self.last_diagonal, &mut self.current_diagonal);

        self.iteration += 1;
    }

    fn is_doubling(&self) -> bool {
        !self.last_diagonal.has_state_in_pattern(1) && self.penult_diagonal.count_state_in_pattern(1) % 2 == 1
    }

    pub fn evolve(&mut self, steps: usize)
    {
        for _ in 0..steps
        {
             self.next();
        }
    }
}