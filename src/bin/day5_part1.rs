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

fn solve(polymer: &String) -> usize {

    let mut polymer = polymer.clone()
        .chars()
        .collect::<Vec<char>>();

    loop {
        let mut reactions = 0;

        let mut i = 0 as usize;
        while i < polymer.len() - 1 {
            if can_react(polymer[i], polymer[i+1]) {
                reactions += 1;
                polymer.remove(i); // Remove unit a
                polymer.remove(i); // Remove unit b (now in place of a)
                // i is now at next unit
            } else {
                i += 1;
            }
        }

        if reactions == 0 { break; }
    }

    polymer.len()
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~135ms
    RELEASE: ~83ms
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