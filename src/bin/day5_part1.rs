#[macro_use] mod common;
use self::common::*;

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

    let mut polymer= polymer.clone()
        .chars()
        .collect::<Vec<char>>();
    let mut polymer_temp = Vec::<char>::with_capacity(polymer.len());

    // Process reactions until inert
    loop {
        // Perform one scan for reactions
        let mut i = 0;
        while i < polymer.len() {
            if i+1 < polymer.len() && can_react(polymer[i], polymer[i+1]) {
                i += 2;
            } else {
                polymer_temp.push(polymer[i]);
                i += 1;
            }
        }

        if polymer.len() == polymer_temp.len() {
            break; // No more reactions in polymer
        } else {
            // Prepare for next scan
            std::mem::swap(&mut polymer, &mut polymer_temp);
            polymer_temp.clear();
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
    DEBUG: ~112ms
    RELEASE: ~1.4ms
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