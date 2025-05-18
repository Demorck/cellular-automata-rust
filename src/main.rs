use cellular_automaton::automaton::Automaton;
use cellular_automaton::automaton_analysis::{AutomatonAnalysis, DIAGONAL};
use cellular_automaton::cell::Cell;
use cellular_automaton::line::Row;
use cellular_automaton::rules::{Rule30, WolframRule};

fn main() {
    // Taille de la ligne
    let size = 31;

    // Configuration initiale : tout à 0 sauf le centre
    let mut config = vec![Cell::new(0); size];
    config[size / 2] = Cell::new(1);

    // Création de la première ligne
    let first_row = Row::new(config);


    let rule = Box::new(WolframRule::new(30));
    // Initialisation de l'automate avec une seule ligne
    let mut automaton = Automaton::new(first_row, rule);
    // Génère 15 itérations
    automaton.evolve(20);

    // Affichage
    for row in automaton.grid().iter().rev() {
        println!("{}", row.to_string());
    }

    // Affichage de la diagonale gauche
    let analysis = AutomatonAnalysis::new(&automaton);
    let diagonals = analysis.extract_diagonals(DIAGONAL::LEFT);
    for (i, diag) in diagonals.iter().enumerate() {
        println!("Diagonal {}: {:?}", i, diag);
    }
}