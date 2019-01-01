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
*/
fn react(polymer: &String) -> String {
    let capacity = polymer.len();
    let mut polymer= polymer.clone()
        .chars()
        .collect::<Vec<char>>();

    // Perform multiple passes to scan for reactions
    loop {
        let mut iter = polymer.iter()
            .chain(Some(' ').iter())
            .tuple_windows();

        let mut result = Vec::with_capacity(capacity);
        while let Some((&a,&b)) = iter.next() {
            if can_react(a,b) {
                let _ = iter.next();
            } else {
                result.push(a);
            }
        }

        if polymer.len() == result.len() {
            break; // No more reactions in polymer
        } else {
            polymer = result; // Prepare for next scan
        }
    }

    polymer.into_iter().collect()
}

/*
 Find length of polymer after all interactions have been resolved.
*/
fn solve(polymer: &String) -> usize {
    react(polymer).len()
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~75.7ms
    RELEASE: ~1.76ms
*/
run!{
    input = "day5",
    run = |input: &Input| {
        let units_remaining = solve(input.raw());
        assert_eq!(units_remaining, 11814);
        println!("Units Remaining: {}", units_remaining);
    },
    bench = |input: &Input| {
        solve(input.raw());
    }
}