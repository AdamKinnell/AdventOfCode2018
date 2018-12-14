use std::fs;
use std::collections::HashSet;

fn main() {

    let input = fs::read_to_string("res/input/day1.txt")
        .expect("Unable to read input");

    // Get an infinite list of frequency changes
    let changes = input.split("\r\n")
        .map(|x| x.parse::<i32>().unwrap())
        .cycle();

    // Find the first frequency seen twice
    let mut seen = HashSet::new();
    let mut freq = 0;
    for change in changes {
        if seen.contains(&freq) {
            break;
        }
        seen.insert(freq);
        freq += change;
    }

    println!("First Repeated Frequency: {}", freq);
}
