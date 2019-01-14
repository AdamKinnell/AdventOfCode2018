#[macro_use] mod common;
use self::common::*;

use itertools::Itertools;

// Types //////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {

    fn parse(str: &String) -> Point {
        let coords = str.split(", ")
            .map(|c| c.parse().unwrap())
            .collect_tuple::<(i32, i32)>()
            .unwrap();
        Point { x:coords.0, y:coords.1}
    }

    fn dist(&self, other: &Point) -> i32 {
        let x_dist = (self.x - other.x).abs() as i32;
        let y_dist = (self.y - other.y).abs() as i32;
        x_dist + y_dist
    }

    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct SpiralIterator {
    now_at: Point,
    steps_moved: i32,
    direction: i32,
    num_turns: i32,
}

impl SpiralIterator {
    fn for_center_point(center: Point) -> SpiralIterator {
        SpiralIterator {
            now_at: center,
            steps_moved: 0,
            direction: 0,
            num_turns: 0,
        }
    }
}

impl SpiralIterator {

    /*
     Get the layer of the spiral the iterator is currently at.
    */
    fn current_layer(&self) -> i32 {
        (self.num_turns + 3) / 4
    }
}

impl Iterator for SpiralIterator {
    type Item = Point;

    /*
     Get the next position on the spiral.

     Turn in clockwise direction.
     Steps until next turn increases by 1 every two turns.
     Will therefore follow the movement sequence:
        RIGHT(1), DOWN(1), LEFT(2), UP(2), ...
    */
    fn next(&mut self) -> Option<Point> {

        // Get next point
        let point = self.now_at;

        // Move to next
        self.now_at = self.now_at.add(
            &[
                Point {x: 1,y: 0}, // Right
                Point {x: 0,y: 1}, // Down
                Point {x:-1,y: 0}, // Left
                Point {x: 0,y:-1}, // Up
            ][self.direction as usize]
        );
        self.steps_moved += 1;

        // Turn if needed
        if self.steps_moved == (self.num_turns / 2) + 1 {
            self.direction = (self.direction + 1) % 4;
            self.steps_moved = 0;
            self.num_turns += 1;
        }

        Some(point)
    }
}

// Functions //////////////////////////////////////////////////////////////////

/*
 Find the average of a list of points.
 This can be used as an approximate "center".
*/
fn find_center_point(points: &Vec<Point>) -> Point {

    let avg_x = points.iter()
        .map(|p| p.x)
        .sum::<i32>() / points.len() as i32;

    let avg_y = points.iter()
        .map(|p| p.y)
        .sum::<i32>() / points.len() as i32;

    Point { x:avg_x, y:avg_y }
}

/*
 Sum the manhattan distances from a point to all other points.
*/
fn sum_distances_to(from: Point, points: &Vec<Point>) -> i32 {
    points.iter()
        .map(|p| from.dist(p))
        .sum::<i32>()
}

/*
 Find the size of the safe region by searching outwards from the approximate center point.
*/
fn find_safe_region_size(points: &Vec<Point>, max_distance_sum: i32) -> usize {

    // Start at point of minimal summed distances (hopefully!)
    let center = find_center_point(&points);
    let mut region_size = 0;
    let mut last_found_at_layer = 0;

    let mut iter = SpiralIterator::for_center_point(center);
    while let Some(point) = iter.next() {
        if sum_distances_to(point, points) <= max_distance_sum {
            // Within region
            region_size += 1;
            last_found_at_layer = iter.current_layer();
        } else {
            // Not within region
            if iter.current_layer() == last_found_at_layer + 2 {
                // Region has been isolated by unsafe padding layer
                break
            }
        }
    }

    region_size
}

/*
 Find the area of the safe region.
 i.e. points with a combined distance of < 10,000 from all other points.
*/
fn solve(points: Vec<String>) -> usize {

    // Parse points
    let points = points.iter()
        .map(Point::parse)
        .collect::<Vec<Point>>();

    find_safe_region_size(&points, 10000 - 1)
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~185ms
    RELEASE: ~1.1ms
*/
run! {
    input = "day6",
    run = |input: &Input| {
        let safe_region_size = solve(input.to_lines());
        assert_eq!(safe_region_size, 39560);
        println!("Area of safe region: {}", safe_region_size);
    },
    bench = |input: &Input| {
        solve(input.to_lines());
    }
}