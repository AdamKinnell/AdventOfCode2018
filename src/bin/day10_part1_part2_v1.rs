#[macro_use] mod common;
use self::common::*;

use std::borrow::Borrow;
use regex::*;
use lazy_static::*;

// Types //////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
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
     Move the position of the point back or forward in time by n steps.
    */
    fn time_offset(&self, time_steps: i32) -> Point {
        let offset_x = self.velocity.x * time_steps;
        let offset_y = self.velocity.y * time_steps;
        Point {
            position: Vec2D {
                x: self.position.x + offset_x,
                y: self.position.y + offset_y,
            },
            velocity: self.velocity.clone(),
        }
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
fn calculate_distance_hint<'a,T>(points: T) -> usize
where T: IntoIterator,
      T::Item: Borrow<Point>,
{
    find_boundary(points).area()
}

/*
 Find a bounding rectangle which can fit all given points.
*/
fn find_boundary<'a,T>(points: T) -> Rect
    where T: IntoIterator,
          T::Item: Borrow<Point>,
{
    let mut min_x = std::i32::MAX;
    let mut min_y = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut max_y = std::i32::MIN;

    for point in points {
        let point = point.borrow();
        min_x = std::cmp::min(min_x, point.position.x);
        max_x = std::cmp::max(max_x, point.position.x);
        min_y = std::cmp::min(min_y, point.position.y);
        max_y = std::cmp::max(max_y, point.position.y);
    }

    Rect {
        from: Vec2D { x:min_x, y:min_y },
        to: Vec2D { x:max_x, y:max_y },
    }
}

/*
 Convert the relative positions of each point into a multi-line string.
 A '#' indicates the presence of a point, while a '.' indicates the absence of any point.
*/
fn stringify_points(points: &Vec<Point>) -> String {
    let boundary = find_boundary(points.iter());
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
 Use a binary search algorithm to find the integer value x, such that f(x) is minimal.
 f() shall be a strictly unimodal function that converges to a single minimal value.

 start = Hint for the starting value of x.
 step = Hint for the initial step of x.
*/
fn find_minimum(start: i32, step: i32, f: &Fn(i32) -> i32) -> i32 {
    let mut x = start;         // Current value of x
    let mut step = step;        // Change in x each iteration
    let mut last_fx = std::i32::MAX; // Last seen value of f(x)
    loop {
        // Try next value of x
        x += step;
        let fx = f(x);

        // How do we adjust x?
        if fx < last_fx {
            // f() is minimal in this direction
        } else if fx > last_fx {
            // f() is growing, the minimum is back the other way
            if step.abs() == 1 {
                return x + -step; // We only just passed it
            } else {
                step = -step; // Search the other direction
                step /= 2;    // Use smaller steps
            }
        }
        last_fx = fx;
    }
}

// Entry Point ////////////////////////////////////////////////////////////////

fn solve(points: &Vec<String>) -> (i32, String) {

    // Parse points
    let points = points.iter()
        .map(Point::parse)
        .collect::<Vec<Point>>();

    // Find timestamp of convergence
    let timestamp= find_minimum(0, 1024, &|x| {
        let points = points.iter()
            .map(|p| p.time_offset(x));
        calculate_distance_hint(points) as i32
    });

    // Read message
    let converged_points = points.iter()
        .map(|p| p.time_offset(timestamp))
        .collect::<Vec<Point>>();
    let message = stringify_points(&converged_points);

    (timestamp, message)
}

/*
 Timings:
    DEBUG: ~15.27ms
    RELEASE: ~320us
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