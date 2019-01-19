#[macro_use] mod common;
use self::common::*;

use regex::*;
use lazy_static::*;

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
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"-?\d+").unwrap();
        }
        let captures = REGEX.find_iter(point);

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
     Find the time step where the paths of two points intersect.
     The time is assumed to be in the future and will be relative to self.
    */
    fn time_of_intersection(&self, other: &Point) -> Option<f32> {

        // Convert to point/slope form
        let m1 = self.velocity.y as f32 / self.velocity.x as f32;
        let x1 = self.position.x as f32;
        let y1 = self.position.y as f32;
        let m2 = other.velocity.y as f32 / other.velocity.x as f32;
        let x2 = other.position.x as f32;
        let y2 = other.position.y as f32;

        // Paths must not be parallel
        if m1 == m2 {
            return None // Will never intersect
        }

        // Find point of intersection
        let x_intercept = (m1*x1 - m2*x2 + y2 - y1) / (m1 - m2);

        // Find time of intersection
        let x_dist = x1 - x_intercept;
        let time_steps = x_dist / self.velocity.x as f32;

        Some(time_steps.abs())
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

/*
 Find the intersection times of each pair of points, then return the mean.
*/
fn avg_time_of_intersection(points: &Vec<Point>) -> f32 {
    let intersection_times = points.windows(2)
        .filter_map(|pair| pair[0].time_of_intersection(&pair[1]))
        .collect::<Vec<f32>>();
    let sum = intersection_times.iter().sum::<f32>();
    let count = intersection_times.len() as f32;

    sum / count
}

// Entry Point ////////////////////////////////////////////////////////////////

fn solve(points: &Vec<String>) -> (i32, String) {

    // Parse points
    let mut points = points.iter()
        .map(Point::parse)
        .collect::<Vec<Point>>();

    // Find when the message appears
    let intersect_at = avg_time_of_intersection(&points).round() as i32;

    // Fast-forward to when the message appears
    points.iter_mut().for_each(|p| p.time_offset(intersect_at));

    // Read message
    let message = stringify_points(&points);

    (intersect_at, message)
}

/*
 Timings:
    DEBUG: ~11.45ms
    RELEASE: ~287us
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