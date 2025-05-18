use crate::automaton::Automaton;

pub enum DIAGONAL {
    LEFT,
    RIGHT
}


pub struct AutomatonAnalysis<'a> {
    automaton: &'a Automaton,
    multiplier_left: usize,
    multiplier_right: usize,
}

impl<'a> AutomatonAnalysis<'a> {
    pub fn new(automaton: &'a Automaton) -> Self {
        Self {
            automaton,
            multiplier_left: 1,
            multiplier_right: 1
        }
    }

    /// Extrait les diagonales et retourne un Vec<u8> pour chaque diagonale
    pub fn extract_diagonals(
        &self,
        diagonal: DIAGONAL,
    ) -> Vec<Vec<u8>> {
        let iteration = self.automaton.iteration();
        let mut diagonals = Vec::new();

        for i in 1..iteration {
            let diag = self.extract_diagonal(i, &diagonal);
            diagonals.push(diag);
        }

        diagonals
    }

    fn extract_diagonal(
        &self,
        n: usize,
        diagonal: &DIAGONAL,
    ) -> Vec<u8> {
        let grid = &self.automaton.grid();
        let iteration = self.automaton.iteration();
        let mut result = Vec::new();

        let multiplier = 1;
        let mut offset = 0;

        for i in (n..iteration).step_by(multiplier) {
            let col = match diagonal {
                DIAGONAL::LEFT => iteration + 1 - offset,
                DIAGONAL::RIGHT => iteration + 1 + offset
            };

            if let Some(row) = grid.get(i) {
                if col < row.len() {
                    let cell = row.get(col).unwrap();
                    result.push(cell.state());
                }
            }

            offset += 1;
        }

        result
    }
}
