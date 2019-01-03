#[macro_use] mod common;
use self::common::*;

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

 Stack-based solution for efficient removal adapted from:
 https://www.reddit.com/r/adventofcode/comments/a3912m/2018_day_5_solutions/

 When when two units react together, one will be in the current iteration, while
 the other will be on the top of the stack. This allows us to efficiently discard them
 with minimal operations compared to a vector (requires re-shuffling; O(n) removals) or
 a double-linked list / sparse array (O(1) removals, but with additional overhead).
*/
fn react(polymer: &String) -> String {

    let mut stack = Vec::with_capacity(polymer.len());
    stack.push('+'); // Unmatchable element to simplify logic

    // Process unit reactions in a single pass
    for right in polymer.chars() {
        let left = *stack.last().unwrap();
        if can_react(left, right) {
            // Discard both units
            stack.pop(); // Discard left unit
            // Right unit will remain unused (and be discarded)
        } else {
            // Keep both units (at least for now)
            stack.push(right) // Keep right unit
            // Left unit is kept on the stack
        }
    }

    // Get reacted polymer (excluding first temp)
    stack.iter().skip(1).collect::<String>()
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
    DEBUG: ~495ms
    RELEASE: ~9.86ms
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