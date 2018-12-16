use itertools::Itertools;

/*
 Check if two strings differ by exactly one character.
 Assumes both strings are of equal length.
*/
fn differs_by_exactly_one(a: &String, b: &String) -> bool {
    a.chars().zip(b.chars())
        .filter(|ab| ab.0 != ab.1)
        .count() == 1
}

/*
 Read lines from a file.
*/
fn get_input(path:&str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let box_ids = get_input("res/input/day2.txt");

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

    // Print answer
    println!("Found two strings differing by exactly one character:");
    println!("a: {}", a); // agi r mdjvlhedpsyoqfzuknpjwt
    println!("b: {}", b); // agi t mdjvlhedpsyoqfzuknpjwt
    println!("common: {}", common) // agimdjvlhedpsyoqfzuknpjwt
}