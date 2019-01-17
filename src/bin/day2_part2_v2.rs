#[macro_use] mod common;
use self::common::*;

use itertools::Itertools;

// Functions //////////////////////////////////////////////////////////////////

/*
 Check if two strings differ by exactly one character.
 Assumes both strings are of equal length.
*/
fn differs_by_exactly_one(a: &String, b: &String) -> bool {
    a.chars().zip(b.chars())
        .filter(|(a, b)| a != b)
        .count() == 1
}

fn solve(box_ids: &Vec<String>) -> (String, String, String) {

    // Find pair
    let (a,b) = box_ids.iter()
        .combinations(2)
        .map(|pair| (pair[0], pair[1]))
        .find(|(a,b)| differs_by_exactly_one(a, b))
        .unwrap();

    // Find common characters
    let common: String = a.chars().zip(b.chars())
        .filter(|(a,b)| a == b)
        .map(|(c, _)| c)
        .collect();

    (a.clone(), b.clone(), common)
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~149ms
    RELEASE: ~4.17ms
*/
run! {
    input = "day2",
    run = |input: &Input| {
        let (a,b,common) = solve(&input.to_lines());

        assert_eq!(a, "agirmdjvlhedpsyoqfzuknpjwt");
        assert_eq!(b, "agitmdjvlhedpsyoqfzuknpjwt");
        assert_eq!(common, "agimdjvlhedpsyoqfzuknpjwt");

        println!("Found two strings differing by exactly one character:");
        println!("a: {}", a);
        println!("b: {}", b);
        println!("common: {}", common)
    },
    bench = |input: &Input| {
        solve(&input.to_lines())
    }
}