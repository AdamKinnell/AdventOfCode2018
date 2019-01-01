#[macro_use] mod common;
use self::common::*;

// Types ////////////////////////////////////////////////////////////////////

struct Claim {
    _id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

/*
 Mark the claim on a piece of fabric by incrementing the claim count
 of every square inch.
*/
impl Claim {
    fn apply(&self, fabric: &mut [[u8; 1000]; 1000]) {
        for y in self.y..(self.y + self.height) {
            for x in self.x..(self.x + self.width) {
                let point = &mut fabric[y as usize][x as usize];
                *point = point.saturating_add(1);
            }
        }
    }
}

// Functions //////////////////////////////////////////////////////////////////

/*
 Parse a claim from a string in the following format:
     #1 @ 509,796: 18x15
*/
fn parse_claim(claim: &String) -> Claim {
    let fields: Vec<i32> = claim.chars()
        .map(|c| match c {
            '#'|'@'|','|':'|'x' => ' ',
            _ => c
        })
        .collect::<String>()
        .split_whitespace()
        .map(|p| p.parse().unwrap())
        .collect();

    Claim {
        _id: fields[0],
        x: fields[1],
        y: fields[2],
        width: fields[3],
        height: fields[4],
    }
}

/*
 Find the total area of overlapping claims.
 Each claim is represented by a single line defining a rectangle in a 1000x1000 grid.
*/
fn solve(lines: &Vec<String>) -> i32 {
    let claims = lines.iter().map(parse_claim);

    // Mark claims
    let mut fabric = [[0u8; 1000]; 1000];
    claims.for_each(|claim| claim.apply(&mut fabric));

    // Count squares which are claimed multiple times
    let overlap = fabric.iter()
        .map(|r| r.iter())
        .flatten()
        .filter(|claims| **claims > 1)
        .count();

    overlap as i32
}

// Entry Point ////////////////////////////////////////////////////////////////

run! {
    input = "day3",
    run = |input: &Input| {
        let overlap = solve(&input.to_lines());
        assert_eq!(overlap, 121259);
        println!("Overlap: {}", overlap);
    },
    bench = |input: &Input| {
        // DEBUG: ~89.1ms
        // RELEASE: ~2.0ms
        solve(&input.to_lines())
    }
}