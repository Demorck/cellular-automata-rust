use std::ops::Not;
use crate::cell::Cell;

/// Représente un motif de cellules, avec une partie "gauche" et une partie "centrale".
///
/// Cette structure est utilisée pour décrire l’évolution d’un motif en fonction
/// de la règle 30.
/// Les motifs peuvent évoluer via [`Pattern::next`], qui applique une transformation.
#[derive(Debug)]
pub struct Pattern {
    left_pattern: Vec<Cell>,
    center_pattern: Vec<Cell>,
}

impl Clone for Pattern {
    /// Duplique un `Pattern`, en copiant les vecteurs de cellules.
    fn clone(&self) -> Self {
        Self {
            left_pattern: self.left_pattern.clone(),
            center_pattern: self.center_pattern.clone(),
        }
    }
}

impl Pattern {
    /// Crée un nouveau motif à partir d’un contexte gauche et d’un motif central.
    ///
    /// ```text
    /// let pattern = Pattern::new(vec![Cell::new(1)], vec![Cell::new(0), Cell::new(1)]);
    /// ```
    pub fn new(left_pattern: Vec<Cell>, center_pattern: Vec<Cell>) -> Pattern {
        Self {
            left_pattern,
            center_pattern,
        }
    }

    /// Crée un nouveau couple motif à partir d’une expression binaire à gauche et au centre
    ///
    /// ```text
    /// let pattern = Pattern::new_from_binary("0111", "0000"); // Diagonale 30
    /// ```
    pub fn new_from_binary(left_pattern: &str, center_pattern: &str) -> Pattern {
        let left_cells = left_pattern
            .chars()
            .map(|c| Cell::new(c.to_digit(10).unwrap() as u8))
            .collect();
        let center_cells = center_pattern
            .chars()
            .map(|c| Cell::new(c.to_digit(10).unwrap() as u8))
            .collect();
        Self::new(left_cells, center_cells)
    }

    /// Compte le nombre d’occurrences d’un état donné dans la partie gauche du motif.
    ///
    /// ```text
    /// pattern.count_state_in_left(1);
    /// ```
    pub fn count_state_in_left(&self, state: u8) -> usize {
        self.left_pattern
            .iter()
            .filter(|&x| x == &Cell::new(state))
            .count()
    }

    /// Vérifie si une cellule est présente dans la partie centrale du motif.
    ///
    /// ```text
    /// pattern.contains(&Cell::new(1));
    /// ```
    pub fn contains(&self, x: &Cell) -> bool {
        self.center_pattern.contains(x)
    }

    /// Retourne la taille du motif central.
    ///
    /// ```text
    /// pattern.len(); // Nombre de cellules dans le motif central
    /// ```
    pub fn len(&self) -> usize {
        self.center_pattern.len()
    }

    /// Calcule le motif suivant à partir de ce motif, en appliquant une logique inspirée d’un automate.
    ///
    /// Cette transformation dépend :
    /// - de la présence ou non de l’état `1` dans le motif central,
    /// - du nombre d’occurrences de `1` dans le motif gauche (parité),
    /// - d'une règle implicite basée sur des XOR, NOT, et décalages cycliques.
    ///
    /// Si la partie centrale contient au moins un `1` :
    /// - la transformation utilise une logique basée sur la règle 30 (mais adaptée pour des raisons de performances).
    ///
    /// Sinon :
    /// - le comportement dépend de la parité de `left_pattern`,
    ///   et la taille du motif double en cas de nombre impair de 1.
    ///
    /// # Paramètres
    /// - `default_cell` : utilisé comme valeur initiale si aucune cellule `1` n’est présente
    ///   et que le nombre d’occurrences dans `left_pattern` est pair.
    ///
    /// ```text
    /// let next = pattern.next(Some(&Cell::new(0)));
    /// ```
    pub fn next(&self, default_cell: Option<&Cell>) -> Pattern {
        let len = self.center_pattern.len();

        let cell_type = Cell::new(1);
        let center_ref = &self.center_pattern;
        let left_ref = &self.left_pattern;

        if center_ref.contains(&cell_type) {
            let mut result: Vec<Cell> = vec![cell_type.clone(); len];
            let start_position = center_ref
                .iter()
                .rposition(|p| p == &cell_type)
                .unwrap();
            let mut last_cell = Cell::new(1);

            for i in (1..=len).rev() {
                let index = (i + start_position) % len;
                let index_to = (i + start_position - 1) % len;

                let idx = index % left_ref.len();
                if center_ref[index] == cell_type {
                    last_cell = !left_ref[idx].clone();
                } else {
                    last_cell.set_state(
                        last_cell.state() ^ left_ref[idx].clone().state(),
                    );
                }

                result[index_to] = last_cell.clone();
            }

            Self::new(self.center_pattern.clone(), result)
        } else {
            let number_one = left_ref.iter().filter(|&p| p == &cell_type).count();
            let mut result: Vec<Cell> = vec![cell_type.clone(); len];

            let mut last_cell = if number_one % 2 == 1 {
                cell_type.clone()
            } else {
                default_cell.unwrap().clone()
            };
            result[0] = last_cell.clone();

            for i in 1..len {
                last_cell.set_state(last_cell.state() ^ left_ref[i - 1].clone().state());
                result[i] = last_cell.clone();
            }

            let mut left_pattern = self.center_pattern.clone();
            if number_one % 2 == 1 {
                let base = result.clone();
                let conjugate = result.into_iter().map(|p| p.not()).collect();
                result = [base, conjugate].concat();
                left_pattern = [left_pattern.clone(), left_pattern.clone()].concat();
            }

            Pattern::new(left_pattern, result)
        }
    }

    /// Retourne une chaîne représentant le motif central.
    ///
    /// ```text
    /// pattern.to_string(); // Exemple : "00110"
    /// ```
    pub fn to_string_center(&self) -> String {
        let mut result = String::new();
        self.center_pattern
            .iter()
            .for_each(|cell| result.push_str(cell.to_string().as_str()));
        result
    }

    pub fn to_string_left(&self) -> String {
        let mut result = String::new();
        self.left_pattern
            .iter()
            .for_each(|cell| result.push_str(cell.to_string().as_str()));
        result
    }

    /// Retourne une copie du motif central.
    ///
    /// ```text
    /// let center = pattern.get_center();
    /// ```
    pub fn get_center(&self) -> Vec<Cell> {
        self.center_pattern.clone()
    }
}
