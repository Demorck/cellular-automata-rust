use std::ops::{BitAnd, BitOr, BitXor, Not};

/// Représente une cellule dans un automate cellulaire.
///
/// Chaque cellule possède un état sur 8 bits (0 à 255).
/// Des opérations logiques bit à bit sont définies pour permettre
/// la manipulation et la comparaison des états des cellules.
#[derive(Clone, Debug)]
pub struct Cell {
    state: u8
}

impl Not for Cell {
    type Output = Cell;

    /// Applique l'opérateur logique NOT à la cellule.
    ///
    /// Retourne une nouvelle cellule avec l'état inversé :
    /// - 0 devient 1
    /// - tout autre état devient 0
    fn not(self) -> Self::Output {
        if self.state == 0 { Cell::new(1) } else { Cell::new(0) }
    }
}


impl PartialEq for Cell {
    /// Compare l'égalité entre deux cellules selon leur état.
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl BitOr for Cell {
    type Output = Self;

    /// Applique l'opérateur logique OU bit à bit entre deux cellules.
    ///
    /// Retourne une nouvelle cellule dont l'état est le résultat du OU.
    fn bitor(self, rhs: Self) -> Self::Output {
        let new_state = self.state | rhs.state;
        Self::new(new_state)
    }
}

impl BitXor for Cell {
    type Output = Self;

    /// Applique l'opérateur logique XOR bit à bit entre deux cellules.
    ///
    /// Retourne une nouvelle cellule dont l'état est le résultat du XOR.
    fn bitxor(self, rhs: Self) -> Self::Output {
        let new_state = self.state ^ rhs.state;
        Self::new(new_state)

    }
}

impl BitAnd for Cell {
    type Output = Self;

    /// Applique l'opérateur logique ET bit à bit entre deux cellules.
    ///
    /// Retourne une nouvelle cellule dont l'état est le résultat du ET.
    fn bitand(self, rhs: Self) -> Self::Output {
        let state = self.state & rhs.state;
        Self::new(state)
    }
}


impl Cell {
    /// Crée une nouvelle cellule avec l'état donné.
    ///
    /// # Arguments
    ///
    /// * `state` - Valeur de l'état (0 à 255)
    pub fn new(state: u8) -> Cell {
        Cell {
            state
        }
    }

    /// Retourne l'état courant de la cellule.
    pub fn state(&self) -> u8 {
        self.state
    }

    /// Modifie l'état de la cellule.
    ///
    /// # Arguments
    ///
    /// * `state` - Nouvelle valeur de l'état (0 à 255)
    pub fn set_state(&mut self, state: u8) {
        self.state = state;
    }

    /// Retourne un caractère représentant visuellement l'état de la cellule.
    ///
    /// - `.` pour l'état 0
    /// - `#` pour l'état 1
    /// - `?` pour tout autre état
    pub fn display(&self) -> char {
        match self.state {
            0 => '.',
            1 => '#',
            _ => '?',
        }
    }

    pub fn to_string(&self) -> String {
        self.state.to_string()
    }
}