#[macro_use] mod common;
use self::common::*;

use std::collections::HashSet;

// Types //////////////////////////////////////////////////////////////////////

/*
 Represents a set of transitions for pots.
*/
struct PotTransitionRules {
    transitions: HashSet<String>, // String contains 5 chars.
}

impl PotTransitionRules {

    /*
     Parse transition table from a sequence of lines like:
        "...## => #"
        "..#.. => #"
        ...
    */
    fn parse<'a>(lines: impl IntoIterator<Item=&'a String>) -> PotTransitionRules {
        let transitions = lines.into_iter()
            .filter(|l| l.ends_with('#')) // Results in plant existing
            .map(|l| l.chars().take(5).collect::<String>())
            .collect::<HashSet<String>>();

        PotTransitionRules { transitions:transitions }
    }

    /*
     Check if a pot P in the given context should contain a plan after transitioning.
     The state of two pots on either side of P should also be given.
     i.e. "LLPRR"
    */
    fn evaluate(&self, context: impl Into<String>) -> char {
        let context = context.into();
        assert_eq!(context.len(), 5);

        if self.transitions.contains(&context) {
            '#'
        } else {
            '.'
        }
    }
}

/*
 Represents a row of pots, each with it's own index and state.
*/
struct PotRow {
    zero_at: i32, // The position of the 0 index.
    pots: Vec<char>,
}

impl PotRow {

    /*
     Parse the initial state of pots from a line such as:
        "#..#.#..##......###...###"
    */
    fn parse(line: &str) -> PotRow {
        let line = format!("....{}....", line);
        let pots = line.chars().collect();
        PotRow { pots:pots, zero_at:4 }
    }

    /*
     Spread plants according to the given set of transition rules.
     TODO: This code is terrible.
    */
    fn spread(&mut self, rules: &PotTransitionRules) {

        // Calculate new state for every pot which could have changed
        // (Worst case: "....# => #" or "#...." => #)
        // Will use existing 4-pot padding on either side.
        let transitioned_pots = self.pots
            .windows(5)
            .map(|context| context.iter().collect::<String>() )
            .map(|context| rules.evaluate(context))
            .collect::<String>();

        // Remove all padding
        let left_plant = transitioned_pots.find('#').unwrap();
        let right_plant = transitioned_pots.rfind('#').unwrap();
        let unpadded_pots = &transitioned_pots[left_plant..=right_plant];

        // Ensure a constant padding of 4 pots either side for future spreading
        self.pots = "....".chars()
            .chain(unpadded_pots.chars())
            .chain("....".chars())
            .collect::<Vec<char>>();

        // Update zero position
        let mut zero_at = self.zero_at;
        zero_at -= 2;                 // Transitioning initially shortens by 2 padding pots
        zero_at -= left_plant as i32; // After removing padding
        zero_at += 4;                 // After adding padding
        self.zero_at = zero_at;
    }

    /*
     Sum indexes of all pots containing a plant.
    */
    fn sum(&self) -> i32 {
        self.pots.iter()
            .enumerate()
            .fold(0, |sum, (i,p)| {
                if *p == '#' {
                    let i = i as i32 - self.zero_at;
                    sum + i
                } else {
                    sum
                }
            })
    }
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Simulate plant growth over n generations.
*/
fn solve(lines: Vec<String>, generations: i32) -> i32 {

    // Parse input
    let mut pots = PotRow::parse(&lines[0][15..]);
    let rules = PotTransitionRules::parse(&lines[2..]);

    // Evolve over n generations
    for _ in 0..generations {
        pots.spread(&rules);
    }

    pots.sum()
}

/*
 Timings:
    DEBUG: ~6.13ms
    RELEASE: ~230us
*/
run! {
    input = "day12",
    run = |input: &Input| {
        let sum = solve(input.to_lines(), 20);
        assert_eq!(sum, 1733);
        println!("Sum of pot ids containing plants: {}", sum);
    },
    bench = |input: &Input| {
        solve(input.to_lines(), 20);
    }
}