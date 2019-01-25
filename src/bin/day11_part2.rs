#[macro_use] mod common;
use self::common::*;

// Types //////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, PartialEq)]
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

fn solve(serial_no: i32) -> (Vec2D, i32) {

    // Generate summed-area table
    let mut grid = [[0; 300]; 300];
    for y in 0..300 {
        for x in 0..300 {

            // Calculate power level
            let coord = Vec2D { x: (x + 1) as i32, y: (y + 1) as i32 };
            let power = find_power_level(coord, serial_no);

            // Calculate summed-area
            let above      = if y > 0 { grid[y-1][x] } else { 0 };
            let left       = if x > 0 { grid[y][x-1] } else { 0 };
            let above_left = if y > 0 && x > 0 { grid[y-1][x-1] } else { 0 };

            grid[y][x] = power + above + left - above_left;
        }
    }

    // Find sub-matrix with largest sum
    let mut best_matrix_size = 0;
    let mut best_matrix_pos = Vec2D { x:-1, y:-1 };
    let mut best_matrix_sum = std::i32::MIN;

    // For each coordinate on grid
    for y in 0..300 {
        for x in 0..300 {

            // Check all possible square sub-matrices anchored lower-right on (x,y)
            for s in 1..=(300-x) {

                // Calculate sum of sub-matrix (total power)
                let _4_4 = grid[y][x];
                let _4_1 = if y >= s { grid[y-s][x] } else { 0 };
                let _1_4 = if x >= s { grid[y][x-s] } else { 0 };
                let _1_1 = if x >= s && y >= s { grid[y-s][x-s] } else { 0 };
                let matrix_sum = _4_4 - _1_4 - _4_1 + _1_1;

                // Remember highest power square
                if matrix_sum > best_matrix_sum {
                    best_matrix_size = s;
                    best_matrix_pos = Vec2D { x:x as i32, y:y as i32 };
                    best_matrix_sum = matrix_sum;
                }
            }
        }
    }

    (Vec2D { // (0-based) Lower-Right => (1-based) Top-Left
        x:best_matrix_pos.x - best_matrix_size as i32 + 2,
        y:best_matrix_pos.y - best_matrix_size as i32 + 2,
    }, best_matrix_size as i32)
}

/*
 Timings:
    DEBUG: ~960ms
    RELEASE: ~29ms
*/
run! {
    input = "day11",
    run = |_: &Input| {

        // Example 1
        let (pos, size) = solve(18);
        assert_eq!(pos.x, 90);
        assert_eq!(pos.y, 269);
        assert_eq!(size, 16);

        // Example 2
        let (pos, size) = solve(42);
        assert_eq!(pos.x, 232);
        assert_eq!(pos.y, 251);
        assert_eq!(size, 12);

        // Answer
        let (pos, size) = solve(9005);
        assert_eq!(pos.x, 235);
        assert_eq!(pos.y, 287);
        assert_eq!(size, 13);

        println!("Coordinate of {0}x{0} matrix: ({1},{2})", size, pos.x, pos.y);
    },
    bench = |_: &Input| {
        solve(9005)
    }
}