use cellular_automaton::automaton::Automaton;
use cellular_automaton::automaton_analysis::{AutomatonAnalysis, DIAGONAL};
use cellular_automaton::cell::Cell;
use cellular_automaton::row::Row;
use cellular_automaton::rules::{Rule30, WolframRule};

const ITERATION: usize = 100;

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

    let mut analysis = AutomatonAnalysis::new(&automaton);
    analysis.extract_diagonals(DIAGONAL::LEFT);

    let a = analysis.extract_patterns(DIAGONAL::LEFT, over_two);

    println!("{:?}", a);

}

fn over_two(x: usize) -> usize { x / 2 }