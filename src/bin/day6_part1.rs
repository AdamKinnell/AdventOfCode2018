#[macro_use] mod common;
use self::common::*;

use itertools::Itertools;

// Types //////////////////////////////////////////////////////////////////////

type Grid = [[i8; 1000]; 1000];

#[derive(Copy, Clone)]
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

    fn dist(&self, other: &Point) -> usize {
        let x_dist = (self.x - other.x).abs() as usize;
        let y_dist = (self.y - other.y).abs() as usize;
        x_dist + y_dist
    }
}

// Functions //////////////////////////////////////////////////////////////////

/*

*/
fn mark_closest(points: &Vec<Point>, grid: &mut Grid) {

    // For each coordinate on grid
    'row: for y in 0..(grid.len()) {
        'col: for x in 0..(grid[0].len()) {

            // Find distances to each point
            let this = Point { x:x as i32, y:y as i32};
            let mut distances = points.iter()
                .enumerate()
                .map(|(i,p)| (i, p.dist(&this)))
                .sorted_by(|(_,d1),(_,d2)| d1.cmp(d2));

            // Find closest point
            let closest = distances.next().unwrap();
            let next_closest = distances.next().unwrap();
            if closest.1 == next_closest.1 {
                // Tie for closest point
                grid[y][x] = -1 as i8;
            } else {
                // Mark closest point
                grid[y][x] = closest.0 as i8;
            }
        }
    }
}

/*

*/
fn find_largest_area(points: &Vec<Point>, grid: &Grid) -> (Point, usize) {
    let mut owned = std::collections::HashMap::new();

    // Count "owned" coordinates for each point
    'row: for y in 0..(grid.len()) {
        'col: for x in 0..(grid[0].len()) {

            // Increment owned count
            let owner = grid[y as usize][x as usize];
            *owned.entry(owner).or_insert(0) += 1;

            // Ignore owner completely if they own an infinite area
            // Any area touching the edge of the grid will be infinite (under manhattan dist)
            if [0,grid.len()-1].contains(&y) || [0, grid[0].len()-1].contains(&x) {
                // Effectively remove this owner from future consideration
                owned.insert(owner, std::i32::MIN);
            }
        }
    }

    // Find point with largest owned area
    let (i, area) = owned.iter()
        .max_by(|a,b| a.1.cmp(b.1))
        .unwrap();

    (points[*i as usize], *area as usize)
}

fn solve(points: Vec<String>) -> (Point, usize) {

    let points = points.iter()
        .map(Point::parse)
        .collect::<Vec<Point>>();

    let mut grid= [[-1; 1000]; 1000];
    mark_closest(&points, &mut grid);
    find_largest_area(&points, &grid)
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~29s
    RELEASE: ~890ms
*/
run! {
    input = "day6",
    run = |input: &Input| {
        let (point, largest_area) = solve(input.to_lines());

        assert_eq!(largest_area, 4398);

        println!("Point with largest area: {},{}", point.x, point.y);
        println!("Largest Area: {}", largest_area);
    },
    bench = |input: &Input| {
        solve(input.to_lines());
    }
}