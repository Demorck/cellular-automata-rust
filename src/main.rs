use std::fs::OpenOptions;
use cellular_automaton::cell::Cell;
use cellular_automaton::fast_30::fast30::{Fast30};
use cellular_automaton::pattern::Pattern;


const START_DIAGONAL: i64 = 6_130_000_003;
fn main() {

    let mut fast = Fast30::new();
    fast.evolve(3_000_000_000);
}

fn write_double(pattern: &Pattern, i: i64) {
    let mut file_double = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("output/new_pattern_double.txt")
        .expect("Unable to open file");

    let mut string = format!("On double à l'itération: {}\n", i + 3);
    let a = format!("Gauche: {}\n", pattern.to_string_left());
    string.push_str(a.as_str());
    let a = format!("Centre: {}\n-------------\n", pattern.to_string_center());
    string.push_str(a.as_str());

    write_line(&mut file_double, &*string).expect("Problème d'écriture dans le fichier");
}

fn write_double_pas(pattern: &Pattern, i: i64) {
    let mut file_double = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("output/new_pattern_double_pas.txt")
        .expect("Unable to open file");

    let mut string = format!("On double pas à l'itération: {}\n", i + 3);
    let a = format!("Gauche: {}\n", pattern.to_string_left());
    string.push_str(a.as_str());
    let a = format!("Centre: {}\n--------------\n", pattern.to_string_center());
    string.push_str(a.as_str());

    write_line(&mut file_double, &*string).expect("Problème d'écriture dans le fichier");
}

#[allow(dead_code)]
fn over_two(x: usize) -> usize {
    if x > 100 { x / 2 } else { x }
}

#[allow(dead_code)]
fn identite(x: usize) -> usize { x }

#[allow(dead_code)]
fn zero(_x: usize) -> usize { 0 }

#[allow(dead_code)]
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

#[allow(dead_code)]
fn write_line(file: &mut std::fs::File, data: &str) -> std::io::Result<()> {
    use std::io::Write;
    writeln!(file, "{}", data)
}