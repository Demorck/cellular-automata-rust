use std::fs::{File, OpenOptions};
use std::hash::RandomState;
use std::ops::Deref;
use rand::Rng;
use cellular_automaton::automaton::Automaton;
use cellular_automaton::automaton_analysis::{AutomatonAnalysis, DIAGONAL};
use cellular_automaton::cell::Cell;
use cellular_automaton::diagonal::Fast30;
use cellular_automaton::pattern::Pattern;
use cellular_automaton::row::Row;
use cellular_automaton::rules::{Rule30, WolframRule};

const ITERATION: usize = 150_000;

fn main() {
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
    // println!("Automaton evolved for {} iterations.", ITERATION);
    // //
    // let mut analysis = AutomatonAnalysis::new(&automaton);
    // analysis.extract_diagonals(DIAGONAL::LEFT);
    // //
    // let a = analysis.extract_patterns(DIAGONAL::LEFT, over_two);



    //
    // let mut counter = 0;
    // for (d, p, o) in a {
    //     counter += 1;
    //     let s: String = d.iter().map(|&cell| cell.state().to_string()).collect();
    //     println!("Diagonal {}: {}, Period:  {}, Offset: {}", counter, s, p, o);
    // }


    let first_diag = vec![Cell::new(1)];
    let second_diag = vec![Cell::new(1)];

    let cell_zero = Cell::new(0);
    let cell_one = Cell::new(1);
    let mut rng = rand::rng();

    let mut cell_type = Cell::new(1);
    let mut breaked = false;
    let mut j = 0;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // ou append(false)
        .open("output/pattern.txt")
        .expect("Unable to open file");
    let mut counter = 0;
    let mut pattern = Pattern::new(first_diag.clone(), second_diag.clone());

    for i in 0..100_000_000 {
        if !pattern.contains(&cell_one) {
            if pattern.count_state_in_left(1) % 2 == 0 {
                if counter % 2 == 0 {
                    cell_type = Cell::new(0);
                } else {
                    cell_type = Cell::new(1);
                }
                println!("Hop, on double pas ici: {}", i);
                counter += 1;
            } else {
                cell_type = Cell::new(1);
                println!("Hop, on double de fou ici: {}", i);
            }
        }

        pattern = pattern.next(Some(&cell_type.clone()));

        // let string = format!("{};{}", i, pattern.to_string());
        // write_line(&mut file, string.as_str()).expect("TODO: panic message");
    }

    // let mut fast = Fast30::new();
    // fast.evolve(100_000);
    // println!("{}", fast.to_string(false));
    // let vec1 = vec![1; 40];
    // let mut vec2 = vec![0];
    // for _ in 0..40 {
    //     vec2.push(1);
    // }
    //
    // let diag = fast.next_2(vec2.clone(), vec1.clone(), 2);
    // println!("{:?}", diag);
    //
    // let diag1 = fast.next_2(diag.clone(), vec1.clone(), 3);
    // println!("{:?}", diag1);
    //
    // let diag2 = fast.next_2(diag1.clone(), diag.clone(), 4);
    // println!("{:?}", diag2);


    // let first_diag = vec![Cell::new(1)];
    // let second_diag = vec![Cell::new(1)];
    //
    // let mut cell_type = Cell::new(1);
    // let mut pattern = Pattern::new(first_diag, second_diag);
    // let mut active_pattern = Vec::with_capacity(30);
    // let mut index_double = Vec::with_capacity(30);
    // active_pattern.push(&pattern);
    //
    // // recurse_pattern(pattern, &cell_type, 0, &mut index_double);
    //
    //
    // for i in 1..1_000_000 {
    //     let last_pattern = pattern;
    //     pattern = last_pattern.next(Some(&cell_type));
    //     if !pattern.contains(&cell_type) {
    //         println!("{}", i);
    //     }
    //     if last_pattern.len() < pattern.len() {
    //         println!("Last pattern: {:?}", last_pattern.to_string());
    //         println!("New pattern: {}", pattern.to_string());
    //         println!("Iteration: {}", i);
    //         println!("#######");
    //
    //         index_double.push(i + 2);
    //         if i + 2 == 401 {
    //             cell_type.set_state(1);
    //         }
    //     }
    // }
    //
    // println!("{:?}", index_double);
}

fn over_two(x: usize) -> usize {
    if x > 100 { x / 2 } else { x }
}

fn identite(x: usize) -> usize { x }

fn zero(x: usize) -> usize { 0 }


fn recurse_pattern(pattern: Pattern, cell_type: &Cell, start: usize, index_double: &mut Vec<usize>)
{
    let mut pattern =  pattern;
    let cell_zero = Cell::new(0);
    let cell_one = Cell::new(1);

    for i in start..1_000_000 {
        let last_pattern = pattern;
        pattern = last_pattern.next(Some(cell_type));
        if !pattern.contains(&cell_one) {
            index_double.push(i + 2);

            println!("Recursion at {} with default: 0", i);
            recurse_pattern(pattern.clone(), &cell_zero, i, index_double);


            println!("Recursion at {} with default: 1", i);
            recurse_pattern(pattern.clone(), &cell_one, i, index_double);

            index_double.remove(index_double.len() - 1);
        }
    }

    println!("{:?}", index_double);
}

fn write_line(file: &mut std::fs::File, data: &str) -> std::io::Result<()> {
    use std::io::Write;
    writeln!(file, "{}", data)
}