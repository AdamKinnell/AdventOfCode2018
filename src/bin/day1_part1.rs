use std::fs;

fn main() {

    let input = fs::read_to_string("res/input/day1.txt")
        .expect("Unable to read input");

    let sum:i32 = input.split("\r\n")
        .map(|x| x.parse::<i32>().unwrap())
        .sum();

    println!("Resulting Frequency: {}", sum);
}
