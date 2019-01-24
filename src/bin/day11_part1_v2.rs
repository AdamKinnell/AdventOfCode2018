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

/*
 Find the 3x3 square of fuel cells with the largest total power.
 Returns the top-left coordinate of the group.

 Assume the following 3x3 sub-matrix of a grid:

         ...
 ... [a] [b] [c] ...
 ... [d] [e] [f] ...
 ... [g] [h] {i} ...
         ...

 For each fuel cell {i}:
    i.power = i
        Stored per row to calculate i.left_power_sum
    i.left_power_sum = g + h + g
        Stored for all grid coordinates to calculate i.above_left_power_sum
    i.above_left_power_sum = (a + b + c) + (d + e + f) + (g + h + i)
        Sum of 3x3 square (where i is bottom left). Only highest is stored.

 When calculating the 3x3 sum, it is done from the lower-right point.
*/
fn solve(serial_no: i32) -> Vec2D {

    let mut best_pos = Vec2D { x:-1, y:-1 };
    let mut best_power_sum = std::i32::MIN;

    let mut left_power_sums_grid = [[0; 300]; 300];

    // For each row in the grid:
    for y in 0..300 {

        let mut power_row = [0; 300];

        // For each fuel cell in the row:
        for x in 0..300 {

            // Calculate i.power
            let coord = Vec2D { x:(x + 1) as i32, y:(y + 1) as i32 };
            let i_power = find_power_level(coord, serial_no);
            power_row[x] = i_power;

            // If 3x3 square not out of bounds to the left:
            if x > 2 {

                // Calculate i.left_power_sum
                let i_left_power_sum = power_row[x-2..=x].iter().sum::<i32>();
                left_power_sums_grid[y][x] = i_left_power_sum;

                // If 3x3 square not out of bounds above:
                if y > 2 {
                    // Calculate i.above_left_power_sum
                    let i_above_left_power_sum = left_power_sums_grid[y-2..=y].iter()
                        .map(|row| row[x])
                        .sum::<i32>();

                    // Remember highest power square (largest i.above_left_power_sum)
                    let power_sum = i_above_left_power_sum;
                    if power_sum > best_power_sum {
                        best_power_sum = power_sum;
                        best_pos = coord
                    }
                }
            }
        }
    }

    Vec2D { x:best_pos.x - 2, y:best_pos.y - 2 } // Lower-Right => Top-Left
}

/*
 Timings:
    DEBUG: ~42ms
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