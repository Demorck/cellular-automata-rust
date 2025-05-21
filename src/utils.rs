use std::cmp::PartialEq;
use crate::cell::Cell;

pub fn is_periodic(diagonal: &Vec<&Cell>, period: usize) -> bool
{
    let mut result: bool = true;
    let length = diagonal.len();
    for i in 0..length {
        let pos_period = i + period;
        if pos_period >= length { break; }
        if diagonal[pos_period] != diagonal[i] { result = false; break;}
    }

    result
}