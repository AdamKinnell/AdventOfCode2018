#[macro_use] mod common;
use self::common::*;

use regex::*;
use itertools::*;

// Types //////////////////////////////////////////////////////////////////////

struct Vec2D {
    x: i32,
    y: i32,
}

struct Point {
    position: Vec2D,
    velocity: Vec2D,
}

impl Point {

    /*
     Parse a point (including position and velocity) from a string.
    */
    fn parse(point: &String) -> Point {
        let regex = Regex::new(r"-?\d+").unwrap(); // TODO: lazy-static
        let captures = regex.find_iter(point);

        let mut coordinates = captures
            .map(|m| m.as_str().parse().unwrap());

        Point {
            position: Vec2D {
                x: coordinates.next().unwrap(),
                y: coordinates.next().unwrap(),
            },
            velocity: Vec2D {
                x: coordinates.next().unwrap(),
                y: coordinates.next().unwrap(),
            },
        }
    }

    /*
     Find the manhattan distance between the current position of two points.
    */
    fn dist(&self, other: &Point) -> i32 {
        let x_dist = (self.position.x - other.position.x).abs() as i32;
        let y_dist = (self.position.y - other.position.y).abs() as i32;
        x_dist + y_dist
    }

    /*
     Move the position of the point back or forward in time by n steps.
    */
    fn time_offset(&mut self, time_steps: i32) {
        let offset_x = self.velocity.x * time_steps;
        let offset_y = self.velocity.y * time_steps;
        self.position.x += offset_x;
        self.position.y += offset_y;
    }

    /*
     Move the position of the point forward in time by one step.
    */
    fn time_inc(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }
}

struct Rect {
    from: Vec2D,
    to: Vec2D,
}

impl Rect {

    fn width(&self) -> usize {
        (self.to.x - self.from.x).abs() as usize
    }

    fn height(&self) -> usize {
        (self.to.y - self.from.y).abs() as usize
    }

    fn area(&self) -> usize {
        self.width() * self.height()
    }
}

// Functions //////////////////////////////////////////////////////////////////

/*
 Calculate how close or far away all points are from each other.
 Lower numbers mean the points are closer together.
*/
fn calculate_distance_hint(points: &Vec<Point>) -> usize {
    points.windows(2)
        .map(|pair| pair[0].dist(&pair[1]) as usize)
        .sum()
}

/*
 Find a bounding rectangle which can fit all given points.
*/
fn find_boundary(points: &Vec<Point>) -> Rect {
    let min_x = points.iter().map(|p| p.position.x).min().unwrap();
    let min_y = points.iter().map(|p| p.position.y).min().unwrap();
    let max_x = points.iter().map(|p| p.position.x).max().unwrap();
    let max_y = points.iter().map(|p| p.position.y).max().unwrap();

    Rect {
        from: Vec2D { x:min_x, y:min_y },
        to: Vec2D { x:max_x, y:max_y },
    }
}

/*
 Convert the relative positions of each point into a multi-line string.
 A '#' indicates
*/
fn stringify_points(points: &Vec<Point>,) -> String {
    let boundary= find_boundary(&points);
    let num_chars = boundary.area() + boundary.height(); // Include newlines
    let mut string = String::with_capacity(num_chars);

    // Print message
    for y in (boundary.from.y)..=(boundary.to.y) {
        for x in (boundary.from.x)..=(boundary.to.x) {
            let is_point_here = points.iter()
                .any(|p| p.position.x == x && p.position.y == y);

            if is_point_here {
                string.push('#')
            } else {
                string.push('.')
            }
        }
        string.push('\n');
    }

    string
}

// Entry Point ////////////////////////////////////////////////////////////////

fn solve(points: &Vec<String>) -> (i32, String) {

    // Parse points
    let mut points = points.iter()
        .map(Point::parse)
        .collect::<Vec<Point>>();

    // Find timestamp
    let mut time = 0;
    let mut minimal = std::usize::MAX;
    loop {
        let distance_hint = calculate_distance_hint(&points);
        if distance_hint < minimal {
            // Points are still converging
            minimal = distance_hint;
            points.iter_mut().for_each(|p| p.time_offset(1));
            time += 1;

        } else {
            // Points are moving away now
            points.iter_mut().for_each(|p| p.time_offset(-1));
            time -= 1;
            break
        }
    }

    // Read message
    let message = stringify_points(&points);

    (time, message)
}

/*
 Timings:
    DEBUG: ~1.37s
    RELEASE: ~33.6ms
*/
run! {
    input = "day10",
    run = |input: &Input| {
        let (timestamp, message) = solve(&input.to_lines());
        assert_eq!(timestamp, 10086);
        println!("Timestamp: {}", timestamp);
        print!("Message:\n\n{}", message);
    },
    bench = |input: &Input| {
        solve(&input.to_lines())
    }
}