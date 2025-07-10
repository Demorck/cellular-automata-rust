use std::mem::swap;
use std::time::Instant;
use crate::fast_30::diagonal::Diagonal;
use crate::fast_30::flags_options::Fast30Option;
use crate::fast_30::options::Options;

const DEFAULT_TRANSIT_CAPACITY: usize = 1_000_000;
const DEFAULT_PATTERN_CAPACITY: usize = 64;

/// Fast30 est une approche par les diagonales pour générer le diagramme espace-temps de l'automate cellulaire de la règle 30 plus rapidement et surtout d'analyser les diagonales gauches.
/// Il est possible de trouver une diagonale `k` en fonction de la diagonale `k-1` et de la diagonale `k-2`.
///
/// # Champs
/// * `last_diagonal`: La dernière diagonale calculée (k-1).
/// * `penult_diagonal`: La pénultième diagonale calculée (k-2).
/// * `current_diagonal`: La diagonale courante (k).
/// * `current_period`: La période actuelle du motif.
/// * `iteration`: Le nombre d'itérations effectuées.
pub struct Fast30 {
    last_diagonal: Box<Diagonal>,
    penult_diagonal: Box<Diagonal>,
    current_diagonal: Box<Diagonal>,
    current_period: usize,
    iteration: usize,

    pub options: Options
}

impl Fast30 {
    /// Crée une nouvelle instance de `Fast30`.
    ///
    /// Par défault, la capacité du veceur de transit est de 1_000_000 et celle du motif est de 64.
    /// En effet, quand le motif arrive à une longueur de 64, l'itération est à + 2 milliards.
    pub fn new() -> Self {
        let mut penult_diagonal = Diagonal::new(Vec::with_capacity(DEFAULT_PATTERN_CAPACITY), Vec::with_capacity(DEFAULT_TRANSIT_CAPACITY));
        let mut last_diagonal = penult_diagonal.clone();
        let current_diagonal = penult_diagonal.clone();

        penult_diagonal.pattern.push(1);

        last_diagonal.pattern.push(1);
        last_diagonal.leading_zeros = 0;

        Self {
            last_diagonal: Box::new(last_diagonal),
            penult_diagonal: Box::new(penult_diagonal),
            current_diagonal: Box::new(current_diagonal),
            current_period: 1,
            iteration: 2,
            options: Options::new(),
        }
    }

    /// Génère la diagonale k en fonction des diagonales k-1 et k-2.
    fn next(&mut self)
    {
        if self.is_doubling() {
            self.current_period *= 2;
            if self.options.want_log_doubling {
                println!("Doubling at iteration: {}", self.iteration + 1);
            }
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

            if i > tau_k1 + zeta_k1 {
                j += 1;
                self.current_diagonal.pattern.push(last_state);
            }

            i += 1;
        }

        swap(&mut self.penult_diagonal, &mut self.last_diagonal);
        swap(&mut self.last_diagonal, &mut self.current_diagonal);
    }

    fn is_doubling(&self) -> bool {
        !self.last_diagonal.has_state_in_pattern(1) && self.penult_diagonal.count_state_in_pattern(1) % 2 == 1
    }

    /// Génère les diagonales de l'automate cellulaire de la règle 30.
    ///
    /// # Arguments
    /// * `steps` - Le nombre d'itérations à effectuer.
    pub fn evolve(&mut self, steps: usize)
    {
        for _ in 0..steps
        {
            self.next();
            self.iteration += 1;

            if self.iteration % self.options.elude_diagonal_steps == 0 {
                self.elude_diagonals(false);
            }

            if self.options.want_to_save &&  self.iteration % self.options.save_steps == 0 {
                self.save_to_file();
            }

            if self.options.want_log_steps && self.iteration % self.options.logging_steps == 0 {
                println!("Iteration: {}", self.iteration);
            }
        }
    }

    /// Élude les diagonales courantes (k-1 et k-2) de l'automate cellulaire.
    /// # Arguments
    /// * `log` - Si `true`, affiche les informations avant et après l'élusion.
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

    /// Retourne une chaine de caractères représentant l'état actuel des diagonales.
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

    pub fn set_options(&mut self, opts: Fast30Option) {
        self.options.set_options(opts);
    }

    fn save_to_file(&mut self) {
        use std::fs::OpenOptions;
        use std::io::Write;

        let now = Instant::now();

        let time_since_start = now.duration_since(self.options.start_time).as_secs();
        let time_since_last = self.options.last_save.map_or(0, |prev| now.duration_since(prev).as_secs());

        self.options.last_save = Some(now);

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.options.path_to_save_file)
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
}
