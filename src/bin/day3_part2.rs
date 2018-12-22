#[macro_use] mod common;
use self::common::*;

// Types //////////////////////////////////////////////////////////////////////

type FabricSheet = [[u8; 1000]; 1000];

struct Claim {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

impl Claim {

    /*
     Mark the claim on a sheet of fabric by incrementing the claim count
     of every square inch.
    */
    fn apply(&self, fabric: &mut FabricSheet) {
        for y in self.y..(self.y + self.height) {
            for x in self.x..(self.x + self.width) {
                let point = &mut fabric[y as usize][x as usize];
                *point = point.saturating_add(1);
            }
        }
    }

    /*
     Check if this claim overlaps any other.
     All claims including this one must have already been applied.
    */
    fn is_overlapping(&self, fabric: &FabricSheet) -> bool {
        for y in self.y..(self.y + self.height) {
            for x in self.x..(self.x + self.width) {
                let point = &fabric[y as usize][x as usize];
                match point {
                    0 => panic!("Found unclaimed square. This claim must not have been applied."),
                    1 => (),          // This square is only part of one claim (this one!)
                    _ => return true, // This square is also part of another claim
                }
            }
        }

        // No overlap
        false
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
        id: fields[0],
        x: fields[1],
        y: fields[2],
        width: fields[3],
        height: fields[4],
    }
}

/*
 Find the id of the non-overlapping claim.
 Each claim is represented by a single line defining a rectangle in a 1000x1000 grid.
*/
fn solve(lines: &Vec<String>) -> i32 {
    let mut fabric = [[0u8; 1000]; 1000];
    let claims = lines.iter()
        .map(parse_claim)
        .collect::<Vec<Claim>>();

    // Mark claims
    claims.iter()
        .for_each(|claim| claim.apply(&mut fabric));

    // Find the claim which doesn't overlap
    let claim = claims.iter()
        .find(|claim| claim.is_overlapping(&fabric) == false);

    claim.unwrap().id
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Non-overlapping ID: 239
*/
run_without_benchmark!("day3", |lines: &Vec<String>| {
    println!("\nNon-overlapping ID: {}\n", solve(lines)); //
});

/*
 Timings:
    DEBUG: ~53ms
    RELEASE: ~1.48ms
*/
//run_with_benchmark!("day3", |lines: &Vec<String>| {
//    solve(lines)
//});