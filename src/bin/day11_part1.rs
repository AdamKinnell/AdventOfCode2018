#[macro_use] mod common;
use self::common::*;

// Types //////////////////////////////////////////////////////////////////////

struct Vec2D {
    x: i32,
    y: i32,
}

// Functions //////////////////////////////////////////////////////////////////

/*
 Find the power level of the fuel cell at the given coordinate.
 Coordinates are 1-based.
*/
fn find_power_level(coord: Vec2D, serial_no: i32) -> i32 {
    let rack_id = coord.x + 10;
    let mut power_level = rack_id * coord.y;
    power_level += serial_no;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10; // Get hundreds digit
    power_level -= 5;

    return power_level
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Find the 3x3 square of fuel cells with the largest total power.
 Returns the top-left coordinate of the group.
*/
fn solve(serial_no: i32) -> Vec2D {

    // Generate fuel cell grid
    let mut grid = [[0; 300]; 300];
    for y in 0..300 {
        for x in 0..300 {
            let coord = Vec2D { x:(x + 1) as i32, y:(y + 1) as i32 };
            grid[y][x] = find_power_level(coord, serial_no);
        }
    }

    // Check all 3x3 squares
    let mut best_pos = Vec2D { x:-1, y:-1 };
    let mut best_power = std::i32::MIN;
    for y in 0..300 - 2 {
        for x in 0..300 - 2 {

            // Calculate square power
            let this_power = grid[y..=y+2].iter()
                .map(|y| y[x..=x+2].iter().sum::<i32>())
                .sum::<i32>();

            // Remember highest power square
            if this_power > best_power {
                best_power = this_power;
                best_pos = Vec2D { x:(x + 1) as i32, y:(y + 1) as i32 };
            }
        }
    }

    best_pos
}

/*
 Timings:
    DEBUG: ~77ms
    RELEASE: ~369us
*/
run! {
    input = "day11",
    run = |_: &Input| {
        let pos = solve(9005);
        assert_eq!(pos.x, 20);
        assert_eq!(pos.y, 32);
        println!("Coordinate of fuel cell: ({},{})", pos.x, pos.y);
    },
    bench = |_: &Input| {
        solve(9005)
    }
}