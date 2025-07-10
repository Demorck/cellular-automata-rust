
/// Représente une diagonale gauche dans l'automate cellulaire sous le code de Wolfram 30 avec une condition initiale triviale.
///
/// Chaque diagonale est définie par une période de transition (avant le motif), le motif, répété en boucle et les zéros au début.
/// Deux diagonales sont égales si elles ont le même motif, la même période de transition et le même nombre de zéros au début.
/// On pourrait faire un `Vec<bool>` mais en rust, un booléen prend 8 bits, donc un `Vec<u8>` est pareil pour stocker des états binaires.
/// TODO: On pourrait par contre faire un `Vec<u64>` pour stocker des états binaires (par chunk de 64 bits). Mais il faudrait adapter le code.
/// TODO: Mettre les champs en privés (à voir)
#[derive(Debug, PartialEq, Eq)]
pub struct Diagonal {
    pub pattern: Vec<u8>,
    pub transit: Vec<u8>,
    pub leading_zeros: usize,
}

impl Clone for Diagonal {
    /// Implémentation de la méthode `Clone` pour permettre la duplication de la diagonale.
    /// Cette implémentation crée une nouvelle instance de `Diagonal` avec les mêmes valeurs pour `pattern`, `transit` et `leading_zeros`.
    fn clone(&self) -> Self {
        Diagonal {
            pattern: self.pattern.clone(),
            transit: self.transit.clone(),
            leading_zeros: self.leading_zeros.clone(),
        }
    }
}


impl Diagonal {
    /// Crée une nouvelle diagonale avec un motif et une période de transition donnés.
    ///
    /// # Arguments
    ///
    /// * `pattern` - Un vecteur d'octets représentant le motif de la diagonale.
    /// * `transit` - Un vecteur d'octets représentant la période de transition avant le motif.
    ///
    /// TODO: Inverser les paramètres pour que le motif soit en deuxième.
    pub fn new(pattern: Vec<u8>, transit: Vec<u8>) -> Diagonal {
        Diagonal { pattern, transit, leading_zeros: 0 }
    }

    /// Crée une nouvelle diagonale à partir de deux chaînes de caractères représentant le motif et la période de transition en binaire.
    ///
    /// # Arguments
    ///
    /// * `transit` - Une chaîne de caractères représentant la période de transition en binaire.
    /// * `pattern` - Une chaîne de caractères représentant le motif en binaire.
    ///
    /// # Exemple
    ///
    /// ```text
    /// let diagonal = Diagonal::new_from_binary("100110111", "0101");
    /// ```
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

    /// Supprime tous les états de la diagonale.
    pub fn clear(&mut self) {
        self.transit.clear();
        self.pattern.clear();
        self.leading_zeros = 0;
    }

    /// Ajoute un état à la période de transition de la diagonale.
    ///
    /// # Complexité
    /// Prend O(1) pour ajouter un état à la fin du vecteur de transition. Si le vecteur de transition est plein, il prend O(n) pour allouer de la mémoire supplémentaire où n est la capacité actuelle.
    pub fn push_transit(&mut self, state: u8)
    {
        self.transit.push(state);
    }

    /// Ajoute un état à la période de motif de la diagonale.
    ///
    /// # Complexité
    /// Prend O(1) pour ajouter un état à la fin du vecteur de motif. Si le vecteur de motif est plein, il prend O(n) pour allouer de la mémoire supplémentaire où n est la capacité actuelle.

    pub fn push_pattern(&mut self, state: u8) {
        self.pattern.push(state);
    }

    /// Définit la diagonale actuelle avec une nouvelle diagonale.
    pub fn set(&mut self, new: Diagonal)
    {
        self.transit = new.transit;
        self.pattern = new.pattern;
        self.leading_zeros = new.leading_zeros;
    }

    /// Retourne l'état de la i-ème position dans la diagonale en prenant en compte les zéros de tête, la période de transitoire et le motif.
    ///
    /// # Arguments
    ///
    /// * `i` - L'index de l'état à récupérer.
    ///
    /// # Exemples
    ///
    /// ```text
    /// let diagonal = Diagonal::new_from_binary("100110111", "0101");
    /// assert_eq!(diagonal.get_from_index(3), 1);
    ///
    /// diagonal.set_leading_zeros(2);
    /// assert_eq!(diagonal.get_from_index(3), 0); // les deux zéros de tête sont pris en compte
    ///
    /// assert_eq!(diagonal.get_from_index(18), 1); // Après bouclage dans le motif, on a le dernier 1 (0 à 1 zéros de tête + (2 à 10) transitoire  + 11 à 14 motif puis 15 à 18 bouclé dans le motif)
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

    // Retourne vraie si l'état donné est présent dans le motif de la diagonale.
    pub fn has_state_in_pattern(&self, state: u8) -> bool {
        self.pattern.contains(&state)
    }

    /// Compte le nombre d'occurrences d'un état donné dans le motif de la diagonale.
    pub fn count_state_in_pattern(&self, state: u8) -> usize {
        self.pattern.iter().filter(|&x| x == &state).count()
    }

    /// Diminue la période de transition en éliminant les états qui sont identiques à ceux du motif.
    ///
    /// # Exemple
    /// ```text
    /// let mut diagonal = Diagonal::new_from_binary("010110111001100", "1100");
    /// let result = Diagonal::new_from_binary("0101101", "1100");
    /// assert_eq!(diagonal, result);
    ///
    /// let mut diagonal = Diagonal::new_from_binary("01011010011001100", "1100");
    /// let result = Diagonal::new_from_binary("010110", "1001");
    /// assert_eq!(diagonal, result);
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

    /// Définit le nombre de zéros de tête pour la diagonale.
    pub fn set_leading_zeros(&mut self, zeros: usize) {
        self.leading_zeros = zeros;
    }

    /// Retourne le nombre de zéros de tête de la diagonale.
    pub fn leading_zeros(&self) -> usize {
        self.leading_zeros
    }

    /// Retourne une représentation sous forme de chaîne de caractères de la diagonale.
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
