#[macro_use] mod common;
use self::common::*;

// Types //////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, PartialEq)]
struct Vec2D {
    x: i32,
    y: i32,
}

trait Index2D {
    /*
     Get the value at the given coordinate, or return <default> if out of bounds.
    */
    fn get2d(&self, x: i32, y: i32, default: i32) -> i32;
}

impl Index2D for [[i32; 300]; 300] {
    fn get2d(&self, x: i32, y: i32, default: i32) -> i32 {
        if x < 0 || y < 0 {
            default
        } else {
            self[y as usize][x as usize]
        }
    }
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

 Each coordinate stores the sum of all coordinates to the left and above (inclusive).
 Assume the following 3x3 sub-matrix of a grid (in parenthesis):

    0   1   2   3   4
 0 [ ] [ ] [ ] [ ] [ ] ...
 1 [ ] [ ] [ ] [ ] [ ] ...
 2 [ ] [ ] (a) (b) (c) ...
 3 [ ] [ ] (d) (e) (f) ...
 4 [ ] [ ] (g) (h) {i} ...

 In the example above, to calculate the area of the 3x3 window, we calculate:
    (4,4) - (1,4) - (4,1) + (1,1), which gives us the area by inclusion-exclusion.

*/
fn solve(serial_no: i32) -> Vec2D {

    let mut best_pos = Vec2D { x:-1, y:-1 };
    let mut best_power_sum = std::i32::MIN;

    // Generate summed-area table
    let mut grid = [[0; 300]; 300];
    for y in 0..300 {
        for x in 0..300 {

            // Calculate power level
            let coord = Vec2D { x: (x + 1) as i32, y: (y + 1) as i32 };
            let power = find_power_level(coord, serial_no);

            // Calculate summed-area
            let above = grid.get2d(x, y-1, 0);
            let left = grid.get2d(x-1, y, 0);
            let above_left = grid.get2d(x-1, y-1, 0);

            grid[y as usize][x as usize] = power + above + left - above_left;
        }
    }

    // Find largest 3x3 sub-matrix sum
    for y in 2..300 {
        for x in 2..300 {

            // Calculate sum of 3x3 grid (see diagram above)
            let _4_4 = grid.get2d(x, y, 0);
            let _4_1 = grid.get2d(x, y-3, 0);
            let _1_4 = grid.get2d(x-3, y, 0);
            let _1_1 = grid.get2d(x-3, y-3, 0);
            let power_sum = _4_4 - _1_4 - _4_1 + _1_1;

            // Remember highest power square
            if power_sum > best_power_sum {
                best_power_sum = power_sum;
                best_pos = Vec2D { x:(x + 1) as i32, y:(y + 1) as i32 };
            }
        }
    }


    Vec2D { x:best_pos.x - 2, y:best_pos.y - 2 } // Lower-Right => Top-Left
}

/*
 Timings:
    DEBUG: ~14.6ms
    RELEASE: ~482us
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