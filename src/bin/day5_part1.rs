#[macro_use] mod common;
use self::common::*;

mod sparse_vector;
use self::sparse_vector::*;

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
 Find length of polymer after all interactions have been resolved.
*/
fn solve(polymer: &String) -> usize {
    react(polymer).len()
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~26ms
    RELEASE: ~0.77ms
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