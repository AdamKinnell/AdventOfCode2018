
// Structs ////////////////////////////////////////////////////////////////////

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
 Read lines from a file.
*/
fn get_input(path:&str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

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
    for claim in claims {
        claim.apply(&mut fabric);
    }

    // Count squares which are claimed multiple times
    let mut overlap = 0;
    for row in fabric.iter() {
        for num_claims in row.iter() {
            if *num_claims > 1 {
                overlap += 1;
            }
        }
    }

    overlap
}

// Entry Point ////////////////////////////////////////////////////////////////

#[macro_use]
extern crate criterion;
use criterion::Criterion;

/*
 Timings:
    DEBUG: ~75ms
    RELEASE: ~2.25ms
*/
fn criterion_benchmark(c: &mut Criterion) {

    // Setup
    const INPUT_FILE: &str = "res/input/day3.txt";
    let lines= get_input(INPUT_FILE);

    // Print Answer
    println!("\nOverlap: {}\n", solve(&lines));

    // Run Benchmark
    c.bench_function("benchmark",move |b| {
        b.iter(|| {
            solve(&lines)
        })
    });
}

criterion_group!{
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::new(1,0))
        .sample_size(2);
    targets = criterion_benchmark
}
criterion_main!(benches);