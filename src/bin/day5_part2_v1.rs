#[macro_use] mod common;
use self::common::*;

mod sparse_vector;
use self::sparse_vector::*;

use itertools::Itertools;

// Functions //////////////////////////////////////////////////////////////////

/*
 Check if two chars are the same letter with opposing capitalization.
*/
fn can_react(a: char, b: char) -> bool {
    (a as i32 - b as i32).abs() == 32
}

/*
 Process polymer reactions until inert.
*/
fn react(polymer: &String) -> String {

    // Create sparse vector to represent polymer
    let polymer= polymer.clone()
        .chars()
        .collect::<Vec<char>>();
    let mut sparse_vec = SparseVector::from_vec(polymer);
    let mut cursor = sparse_vec.cursor();

    // Find and react pairs until inert
    loop {
        let left = *cursor.get();
        if !cursor.move_next() {
            break;
        }
        let right = *cursor.get();

        if can_react(left, right) {
            cursor.remove_then_prev(); // Remove right
            cursor.remove_then_prev(); // Remove left
            // Next iteration will compare either side of gap
        } else {
            // Next iteration will compare next pair
        }
    }

    sparse_vec.iter().collect()
}

/*
 Remove all units of the specified type from a polymer.
*/
fn remove_unit_type(polymer: &String, unit: char) -> String {
    let unit_lower = unit.to_ascii_lowercase();
    let unit_upper = unit.to_ascii_uppercase();
    polymer.replace(|c| c == unit_lower || c == unit_upper, "")
}

/*
 Find length of polymer after all interactions have been resolved.
*/
fn solve(polymer: &String) -> (usize, char) {

    // Get all unit types
    let unit_types = polymer
        .to_ascii_lowercase()
        .chars()
        .sorted()
        .dedup()
        .collect::<Vec<char>>();

    // React polymers without each unit type
    let polymers = unit_types.iter()
        .map(|c| remove_unit_type(polymer, *c))
        .map(|p| react(&p));

    // Find shortest polymer
    let (len,unit) = polymers
        .map(|p| p.len())
        .zip(unit_types.iter())
        .min_by(|(l1, _), (l2,_)| l1.cmp(l2))
        .unwrap();

    (len, *unit)
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~867ms
    RELEASE: ~23.6ms
*/
run!{
    input = "day5",
    run = |input: &Input| {
        let (shortest_length, bad_unit) = solve(input.raw());

        assert_eq!(bad_unit, 'g');
        assert_eq!(shortest_length, 4282);

        println!("Bad unit: {}", bad_unit);
        println!("Shortest length: {}", shortest_length);
    },
    bench = |input: &Input| {
        solve(input.raw());
    }
}