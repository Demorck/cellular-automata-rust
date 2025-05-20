use cellular_automaton::automaton::Automaton;
use cellular_automaton::automaton_analysis::{AutomatonAnalysis, DIAGONAL};
use cellular_automaton::cell::Cell;
use cellular_automaton::line::Row;
use cellular_automaton::rules::{Rule30, WolframRule};

const ITERATION: usize = 20_000;

fn main() {
    // Taille de la ligne
    let size = if ITERATION % 2 == 0 { ITERATION + 1 } else { ITERATION };
    let milieu = (size - 1) / 2;

    // Configuration initiale : tout à 0 sauf le centre
    let mut config = vec![Cell::new(0); size];
    config[milieu] = Cell::new(1);

    // Création de la première ligne
    let first_row = Row::new(config);

    let rule = Box::new(WolframRule::new(30));
    // Initialisation de l'automate avec une seule ligne
    let mut automaton = Automaton::new(first_row, rule);
    automaton.evolve(ITERATION as u64);

    // Affichage
    // for row in automaton.grid().iter().rev() {
    //     println!("{}", row.to_string());
    // }

    // Affichage de la diagonale gauche
    // let analysis = AutomatonAnalysis::new(&automaton);
    // let diagonals = analysis.extract_diagonals(DIAGONAL::LEFT);
    // for (i, diag) in diagonals.iter().enumerate() {
    //     println!("Diagonal {}: {:?}", i, diag);
    // }
    //
    // let cell_type = Cell::new(1);
    // let a = analysis.rightmost_same_state(cell_type);
    //
    // println!("{:?}", a);
}