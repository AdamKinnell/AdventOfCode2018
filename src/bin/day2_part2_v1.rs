#[macro_use] mod common;
use self::common::*;

// Functions //////////////////////////////////////////////////////////////////

/*
 Compare two strings while ignoring the character at the specified index.
 Assumes both strings are of equal length.
 https://cs.stackexchange.com/a/93576
*/
fn compare_ignoring_i(a: &String, b: &String, ignore_i: usize) -> std::cmp::Ordering {
    let pairs = a.chars().zip(b.chars());

    for (i, (ac, bc)) in pairs.enumerate() {
        if i == ignore_i { continue };
        match ac.cmp(&bc) {
            std::cmp::Ordering::Equal
                => continue,
            std::cmp::Ordering::Less
                => return std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater
                => return std::cmp::Ordering::Greater
        };
    }

    std::cmp::Ordering::Equal
}

/*
 Check if two strings differ by exactly n characters.
 Assumes both strings are of equal length.
*/
fn differs_by_exactly_n_chars(a: &String, b : &String, n: usize) -> bool {
    let pairs = a.chars().zip(b.chars());
    let mut differing = 0;

    for (ac, bc) in pairs {
        if ac != bc {
            differing += 1;
            if differing > n {
                return false
            }
        }
    }

    differing == n
}

/*
 Search the vector for two adjacent strings which differ by exactly n characters.
 Assumes both strings are of equal length.
 The first matching pair will be returned.
*/
fn find_adjacent_differing_by_exactly_n_chars(strings: &Vec<String>, n: usize)
    -> Option<(&String, &String)>
{
    strings.windows(2)
        .map(|w| (&w[0], &w[1]))
        .find(|(a,b)| differs_by_exactly_n_chars(&a, &b, n))
}

/*
 Find a pair of strings differing by exactly one character.
 Assumes all strings are of equal length.
 https://cs.stackexchange.com/a/93576
*/
fn find_differing_by_one(strings: &Vec<String>) -> (String, String) {
    let mut sortable = strings.clone();
    for i in 0..sortable.len() {
        // Strings only differing at position i will be made adjacent
        sortable.sort_by(|a, b| compare_ignoring_i(a,b,i));
        // Check all adjacent pairs
        if let Some(pair) = find_adjacent_differing_by_exactly_n_chars(&sortable, 1) {
            return (pair.0.clone(), pair.1.clone());
        }
    }

    // No pair found
    panic!()
}

/*
 Get the characters that are in the same position in each string.
 Assumes both strings are of equal length.
*/
fn common_chars(a: &String, b: &String) -> String {
    a.chars().zip(b.chars())
        .filter(|(a,b)| a == b)
        .map(|(c, _)| c)
        .collect()
}

// Entry Point ////////////////////////////////////////////////////////////////

fn solve(box_ids: &Vec<String>) -> (String, String, String) {
    let (a,b) = find_differing_by_one(&box_ids);
    let common = common_chars(&a, &b);
    (a, b, common)
}

/*
 Timings:
    DEBUG: ~10.8ms
    RELEASE: ~263us
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