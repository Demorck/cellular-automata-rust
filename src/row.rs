use crate::cell::Cell;
use crate::rules::Rule;

/// Représente une ligne d'un automate cellulaire unidimensionnel.
///
/// Une `Row` contient une configuration de cellules (`Cell`) ainsi qu’un compteur
/// d’itération. L’itération n’est pas modifiée automatiquement, mais peut servir
/// à suivre l’évolution dans un contexte plus large (non utilisée ici).
pub struct Row {
    configuration: Vec<Cell>,
    iteration: usize,
}

impl Clone for Row {
    /// Crée une copie de la ligne, en dupliquant la configuration et l’itération.
    fn clone(&self) -> Row {
        Self {
            configuration: self.configuration.clone(),
            iteration: self.iteration,
        }
    }
}

impl Row {
    /// Retourne le nombre de cellules dans la ligne.
    ///
    /// # Exemple
    /// ```text
    /// let row = Row::new(vec![Cell::new(0), Cell::new(1)]);
    /// assert_eq!(row.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.configuration.len()
    }

    /// Retourne une référence vers la cellule à l’indice donné, ou `None` si l’indice est hors limites.
    ///
    /// # Exemple
    /// ```
    /// let row = Row::new(vec![Cell::new(0)]);
    /// assert!(row.get(0).is_some());
    /// assert!(row.get(1).is_none());
    /// ```
    pub fn get(&self, index: usize) -> Option<&Cell> {
        self.configuration.get(index)
    }

    /// Crée une nouvelle ligne à partir d’un vecteur de cellules.
    ///
    /// L’itération est initialisée à `0`.
    ///
    /// # Exemple
    /// ```text
    /// let cells = vec![Cell::new(1), Cell::new(0)];
    /// let row = Row::new(cells);
    /// ```
    pub fn new(configuration: Vec<Cell>) -> Row {
        Self {
            configuration,
            iteration: 0,
        }
    }

    /// Calcule la prochaine ligne selon une règle d’automate cellulaire.
    ///
    /// Applique la règle à chaque triplet `(gauche, centre, droite)` de cellules.
    /// Les bords sont connectés de manière cyclique.
    ///
    /// # Exemple
    /// ```text
    /// let rule = WolframRule::new(30); // Exemple de règle Wolfram 30
    /// let row = Row::new(vec![...]);
    /// let next = row.next(&rule);
    /// ```
    ///
    /// # Remarques
    /// Le comportement cyclique signifie que la première cellule utilise comme "voisine gauche"
    /// la dernière cellule de la ligne, et inversement.
    pub fn next(&self, rule: &dyn Rule) -> Row {
        let mut next_configuration = Vec::with_capacity(self.configuration.len());

        for i in 0..self.configuration.len() {
            let left = if i == 0 { 0 } else { i - 1 };
            let center = i;
            let right = if i == self.configuration.len() - 1 { 0 } else { i + 1 };

            let new_state = rule.apply(
                self.configuration[left].state(),
                self.configuration[center].state(),
                self.configuration[right].state(),
            );

            next_configuration.push(Cell::new(new_state));
        }

        Self::new(next_configuration)
    }

    /// Retourne une représentation textuelle de la ligne, en concaténant les
    /// caractères retournés par `Cell::display()` pour chaque cellule.
    ///
    /// # Exemple
    /// ```text
    /// let row = Row::new(vec![Cell::new(1), Cell::new(0)]);
    /// println!("{}", row.to_string());
    /// ```
    pub fn to_string(&self) -> String {
        self.configuration.iter().map(|c| c.display()).collect()
    }
}
