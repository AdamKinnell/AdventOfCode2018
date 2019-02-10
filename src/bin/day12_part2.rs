#[macro_use] mod common;
use self::common::*;

use std::collections::VecDeque;

// Types //////////////////////////////////////////////////////////////////////

/*
 Represents the state of a pot P, as well as that of two pots on either side.
 i.e. "LLPRR"

 The state of each pot is represented as a single bit,
 and all 5 states are packed into the 5 low-order bits of a single byte.
*/
#[derive(Copy, Clone)]
struct PotContext(u8);

impl PotContext {

    /*
     Parse context from a line of the form: "...##"
    */
    fn parse(context: &str) -> PotContext {
        assert_eq!(context.len(), 5);
        PotContext(context.chars().fold(0, |pc, c| {
            if c == '#' { (pc << 1) | 1 } else { (pc << 1) | 0 }
        }))
    }

    /*
     Shift the pot context over to the right by one pot.
     The new pot's state will be recorded, and the old left-most state will be discarded.
    */
    fn shift(&mut self, pot_state: bool) {
        self.0 <<= 1;              // Move over
        self.0 &= 0b11111;         // Discard left-most pot
        self.0 |= pot_state as u8; // Record right-most pot state
    }
}

/*
 Represents a set of transitions for pots based on their neighbors.
*/
struct PotTransitionRules {
    transitions: [bool; 32], // 2**5 permutations of 5 bits
}

impl PotTransitionRules {

    /*
     Parse transition table from a sequence of lines of the form: "...## => #"
    */
    fn parse(lines: &[String]) -> PotTransitionRules {
        let mut transitions = [false; 32];
        for line in lines {
            if line.ends_with('#') {
                let index = PotContext::parse(&line[0..5]);
                transitions[index.0 as usize] = true;
            }
        }
        PotTransitionRules { transitions:transitions }
    }

    /*
     Evaluate the state of the center pot in the given context.
    */
    fn evaluate(&self, context: PotContext) -> bool {
        self.transitions[context.0 as usize]
    }
}

/*
 Represents a contiguous row of pots, each of which may either have a plant or not.

 Only the pots between the left-most and right-most planted pots (inclusive) are stored.
 To track the absolute indices of each pot, an offset from zero is also stored.
*/
#[derive(Clone)]
struct PotRow {
    zero_at: i64,         // The position of the 0 index to allow storing negative indices
    pots: VecDeque<bool>, // The state of each pot; True indices are offset by -zero_at
}

impl PotRow {

    /*
     Parse the initial state of pots from a line of the form: "#..#.#..##.".
     '#' indicates a plant in that position, while '.' indicates the absence of one.
    */
    fn parse(line: &str) -> PotRow {
        let pots = line.chars()
            .map(|c| c == '#')
            .collect::<VecDeque<bool>>();
        PotRow { pots:pots, zero_at:0 }
    }

    /*
     Spread plants according to the given set of transition rules.
    */
    fn spread(&mut self, rules: &PotTransitionRules) {

        // Pad rows to allow for windows containing first and last planted pots
        for _ in 0..2 { self.pots.push_front(false) }
        for _ in 0..4 { self.pots.push_back(false) }
        self.zero_at += 2; // Account for added left padding

        // Establish sliding window starting before the first plant
        // i.e. "....."
        let mut context = PotContext(0);

        // Iteratively slide window right along pots and record the transition results
        let mut i = 0;
        while i < self.pots.len() - 2 {

            // Slide window
            let context_pot = self.pots[i+2];
            context.shift(context_pot);

            // Process transition in context
            let current_pot = &mut self.pots[i];
            *current_pot = rules.evaluate(context);

            i += 1;
        }

        // Remove padding
        while *self.pots.front().unwrap() == false {
            self.pots.pop_front();
            self.zero_at -= 1;
        }
        while *self.pots.back().unwrap() == false {
            self.pots.pop_back();
        }

    }

    /*
     Sum indexes of all pots containing a plant.
    */
    fn sum(&self) -> i64 {
        let mut sum = 0;
        for (i, pot) in self.pots.iter().enumerate() {
            if *pot { sum += i as i64 - self.zero_at }
        }
        sum
    }
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Simulate plant growth over n generations.
*/
fn solve(lines: Vec<String>, generations: i64) -> i64 {

    // Parse input
    let mut row = PotRow::parse(&lines[0][15..]);
    let rules = PotTransitionRules::parse(&lines[2..]);

    // Evolve over n generations
    for gen in 1..=generations {
        let last_row = row.clone();

        row.spread(&rules);

        // Check if relative positions have reached equilibrium
        if row.pots == last_row.pots {
            // Only offset is changing now, so "fast-forward"
            let gen_offset = row.zero_at - last_row.zero_at;
            let gens_remaining = generations - gen;
            row.zero_at += gen_offset * gens_remaining;
            break
        }
    }

    // Pots have evolved n generations
    row.sum()
}

/*
 Timings:
    DEBUG: ~3.77ms
    RELEASE: ~72.3us
*/
run! {
    input = "day12",
    run = |input: &Input| {
        let sum = solve(input.to_lines(), 50000000000);
        assert_eq!(sum, 1000000000508);
        println!("Sum of pot ids containing plants: {}", sum);
    },
    bench = |input: &Input| {
        solve(input.to_lines(), 50000000000);
    }
}