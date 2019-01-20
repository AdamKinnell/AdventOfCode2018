#[macro_use] mod common;
use self::common::*;

use std::borrow::Borrow;

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
        Point {
            position: Vec2D {
                x: point[10..=15].trim_left().parse().unwrap(),
                y: point[18..=23].trim_left().parse().unwrap(),
            },
            velocity: Vec2D {
                x: point[36..=37].trim_left().parse().unwrap(),
                y: point[40..=41].trim_left().parse().unwrap(),
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
fn convergence_heuristic<'a,T>(points: T) -> usize
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
 f(x) shall be non-negative for all tested values of x.

 start = Hint for the starting value of x.
 step = Hint for the initial step of x.
*/
fn find_minimum(start: i32, step: i32, f: &Fn(i32) -> usize) -> i32 {
    let mut x = start;            // Current value of x
    let mut step = step;          // Change in x each iteration
    let mut last_fx = std::usize::MAX; // Last seen value of f(x)

    let mut l_bound = std::i32::MIN;   // x is >= l_bound
    let mut r_bound = std::i32::MAX;   // x is <= r_bound

    loop {
        // Try next value of x
        let fx = f(x);

        // If we're going to hit a boundary next iteration,
        // then search between this point and the boundary.
        if x + step <= l_bound || x + step >= r_bound {
            // Use smaller steps
            if step > 0 {
                step = std::cmp::max(step / 2, 1);
            } else if step < 0 {
                step = std::cmp::min(step / 2, -1);
            }
        }

        // Are we getting closer to the minimum of f(x)?
        match fx.cmp(&last_fx) {
            std::cmp::Ordering::Greater => {
                // f(x) is increasing now, so we shouldn't keep going in this direction.
                // It's possible that we just jumped over the minimum, so this x is the boundary.
                if step > 0 {
                    r_bound = x;
                } else if step < 0 {
                    l_bound = x;
                }

                // Go back the other way
                step = -step;
            },
            std::cmp::Ordering::Less => {
                // f(x) is still decreasing, so we're probably going in the right direction.
                // However, It's possible that we just jumped over the minimum,
                // so the last x is the boundary.
                if step > 0 {
                    l_bound = x - step;
                } else if step < 0 {
                    r_bound = x - step;
                }
            },
            std::cmp::Ordering::Equal => {
                // We've stopped moving
            },
        }

        // Will oscillate on either side of x once found
        if l_bound + 1 == r_bound - 1 {
            // x is between the boundaries
            return l_bound + 1
        }

        last_fx = fx;
        x += step;
    }
}

// Entry Point ////////////////////////////////////////////////////////////////

fn solve(points: &Vec<String>) -> (i32, String) {

    // Parse points
    let points = points.iter()
        .map(Point::parse)
        .collect::<Vec<Point>>();

    // Find timestamp of convergence
    let timestamp= find_minimum(0, 2048, &|x| {
        let points = points.iter()
            .map(|p| p.time_offset(x));
        convergence_heuristic(points)
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
    DEBUG: ~8.0ms
    RELEASE: ~171us
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