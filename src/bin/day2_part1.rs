#[macro_use] mod common;
use self::common::*;

use itertools::Itertools;

// Functions //////////////////////////////////////////////////////////////////

fn has_n_repetitions(n: i32, s: &str) -> bool {
    s.chars()
        .sorted()                            // "abababb" => "aaabbbb"
        .group_by(|c| *c)                    // "aaabbbb" => ["aaa","bbbb"]
        .into_iter()                         //
        .map(|(_c, group)| group.count())    // ["aaa","bbbb"] => [3, 4]
        .find(|count| *count == n as usize)  // n in [3, 4]?
        .is_some()                           //
}

fn checksum_boxes(box_ids: Vec<String>) -> i32 {

    // Box IDs with two duplicate letters
    let two_count = box_ids
        .iter()
        .filter(|s| has_n_repetitions(2, s))
        .count();

    // Box IDs with three duplicate letters
    let three_count = box_ids
        .iter()
        .filter(|s| has_n_repetitions(3, s))
        .count();

    (two_count * three_count) as i32
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~13.34ms
    RELEASE: ~518us
*/
run! {
    input = "day2",
    run = |input: &Input| {
        let checksum = checksum_boxes(input.to_lines());
        assert_eq!(checksum, 5434);
        println!("Checksum: {}", checksum);
    },
    bench = |input: &Input| {
        checksum_boxes(input.to_lines())
    }
}