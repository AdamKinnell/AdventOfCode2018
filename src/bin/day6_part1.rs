#[macro_use] mod common;
use self::common::*;

use itertools::Itertools;

// Types //////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, PartialEq )]
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

#[derive(Copy, Clone)]
struct Rect {
    from: Point,
    to: Point,
}

impl Rect {

    fn for_each_coordinate(&self, f: &mut FnMut(Point)) {
        'row: for y in (self.from.y)..=(self.to.y) {
            'col: for x in (self.from.x)..=(self.to.x) {
                f(Point { x:x as i32, y:y as i32 })
            }
        }
    }

    fn is_on_boundary(&self, point: &Point) -> bool {
        let x_bound = point.x == self.from.x || point.x == self.to.x;
        let y_bound = point.y == self.from.y || point.y == self.to.y;
        x_bound || y_bound
    }
}

// Functions //////////////////////////////////////////////////////////////////

/*
 Find a bounding rectangle which can fit all given points.
*/
fn find_boundaries(points: &Vec<Point>) -> Rect {
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    Rect {
        from: Point { x:min_x, y:min_y },
        to: Point { x:max_x, y:max_y },
    }
}

/*
 Find the point in <to_options> which is closest to <from>.
 Returns None if there is a tie, or the index of the closest point in <to_options>.
*/
fn closest_point(from: Point, to_options: &Vec<Point>) -> Option<usize> {
    // Find distances to each point
    let mut distances = to_options.iter()
        .enumerate()
        .map(|(i,p)| (i, p.dist(&from)))
        .sorted_by_key(|(_,d)| *d);

    // Find closest point
    let closest = distances.next().unwrap();
    let next_closest = distances.next().unwrap();
    if closest.1 == next_closest.1 {
        None // Tie
    } else {
        Some(closest.0)
    }
}

/*
 Find the size of the area owned by each point.

 Infinite areas will be marked with a negative number.
 Any area touching the edge of the grid will extend infinitely.
*/
fn find_owned_area_sizes(points: &Vec<Point>) -> Vec<i32> {

    let bound = find_boundaries(&points);
    let mut owned_area = Vec::new();
    owned_area.resize(points.len(), 0);

    bound.for_each_coordinate(&mut |point| {
        if let Some(owner) = closest_point(point, &points) {
            if bound.is_on_boundary(&point) {
                // Area infinite; Effectively remove this owner from future consideration
                owned_area[owner] = std::i32::MIN;
            } else {
                // Assign coordinate to owner
                owned_area[owner] += 1;
            }
        } else {
            // Tie for closest
        }
    });

    owned_area
}

/*
 Print a visual map of which coordinates are owned by each point.
*/
fn print_ownership_map(points: &Vec<Point>) {
    let bound = find_boundaries(&points);

    println!("Grid:");
    bound.for_each_coordinate(&mut |point| {
        if point.x == bound.from.x { println!() }
        if let Some(owner) = closest_point(point, &points) {
            if point == points[owner] {
                print!("O,") // Mark owning point
            } else {
                print!("{},", owner) // Owned
            }

        } else {
            print!(".,") // Contested
        }
    });
}

/*
 Find the largest non-infinite owned area.
*/
fn solve(points: Vec<String>) -> (Point, usize, usize) {

    // Parse points
    let points = points.iter()
        .map(Point::parse)
        .collect::<Vec<Point>>();

    // Print map to stdout
    //print_ownership_map(&points);

    // Attribute coordinates to owning points
    let owned = find_owned_area_sizes(&points);

    // Find point with largest owned area
    let (owner_p, (owner_i, area)) = points.iter()
        .zip(owned.iter().enumerate())
        .max_by_key(|(_,(_,a))| *a)
        .unwrap();

    (*owner_p, owner_i, *area as usize)
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~2.6s
    RELEASE: ~96.8ms
*/
run! {
    input = "day6",
    run = |input: &Input| {
        let (owner_p, owner_i, largest_area) = solve(input.to_lines());

        assert_eq!(largest_area, 4398);

        println!("Point with largest area: #{} ({},{})", owner_i, owner_p.x, owner_p.y);
        println!("Largest Area: {}", largest_area);
    },
    bench = |input: &Input| {
        solve(input.to_lines());
    }
}