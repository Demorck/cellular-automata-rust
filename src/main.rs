use cellular_automaton::automaton::Automaton;
use cellular_automaton::automaton_analysis::{AutomatonAnalysis, DIAGONAL};
use cellular_automaton::cell::Cell;
use cellular_automaton::pattern::Pattern;
use cellular_automaton::row::Row;
use cellular_automaton::rules::{Rule30, WolframRule};

const ITERATION: usize = 500_000;

fn main() {
    // Taille de la ligne
    // let size = if ITERATION % 2 == 0 { ITERATION + 1 } else { ITERATION };
    // let milieu = (size - 1) / 2;
    //
    // // Configuration initiale : tout à 0 sauf le centre
    // let mut config = vec![Cell::new(0); size];
    // config[milieu] = Cell::new(1);
    //
    // // Création de la première ligne
    // let first_row = Row::new(config);
    //
    // let rule = Box::new(WolframRule::new(30));
    // // Initialisation de l'automate avec une seule ligne
    // let mut automaton = Automaton::new(first_row, rule);
    // automaton.evolve(ITERATION as u64);
    //
    // let mut analysis = AutomatonAnalysis::new(&automaton);
    // analysis.extract_diagonals(DIAGONAL::LEFT);
    //
    // let a = analysis.extract_patterns(DIAGONAL::LEFT, zero);
    //
    // let mut counter = 0;
    // for (d, p, o) in a {
    //     counter += 1;
    //     let s: String = d.iter().map(|&cell| cell.state().to_string()).collect();
    //     println!("Diagonal {}: {}, Period:  {}, Offset: {}", counter, s, p, o);
    // }


    let first_diag = vec![Cell::new(1)];
    let second_diag = vec![Cell::new(1)];

    let mut cell_type = Cell::new(1);
    let mut pattern = Pattern::new(first_diag, second_diag);
    let mut index_double = Vec::with_capacity(10);


    for i in 1..1_000_000 {
        let last_pattern = pattern;
        pattern = last_pattern.next(Some(&cell_type));
        if last_pattern.len() < pattern.len() {
            index_double.push(i + 2);
            if i + 2 == 401 {
                cell_type.set_state(1);
            }
        }
    }

    println!("{:?}", index_double);
}

fn over_two(x: usize) -> usize {
    if x > 100 { x / 2 } else { x }
}

fn identite(x: usize) -> usize { x }

fn zero(x: usize) -> usize { 0 }