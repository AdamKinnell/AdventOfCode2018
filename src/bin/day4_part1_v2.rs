#[macro_use] mod common;
use self::common::*;

use itertools::Itertools;
use std::collections::HashMap;

// Types //////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, PartialEq)]
enum ShiftStatus {
    Awake,
    Asleep,
}

struct Shift {
    guard: i32,
    mins: [ShiftStatus; 60],
}

impl Shift {
    fn count_mins(&self, state: ShiftStatus) -> usize {
        self.mins.iter()
            .filter(|&&m| m == state)
            .count()
    }
}

// Functions //////////////////////////////////////////////////////////////////

/*
 Create shifts to represent a series of events.

 The input must adhere to the following restrictions:
    + Each line represents a single event in one of the following formats:
    [1518-11-01 00:00] Guard #10 begins shift
    [1518-11-01 00:05] falls asleep
    [1518-11-01 00:25] wakes up
    + All events must be ordered by timestamp in ascending order.
    + Within a shift, the first event (if exists) must be to fall asleep.
    + All other events within a shift must alternate between asleep and awake.
*/
fn create_shifts(lines: &Vec<String>) -> Vec<Shift> {

    let mut shifts = Vec::new();
    for line in lines {

        // Split line
        let parts = line
            .replace(|c| "[]:#".contains(c)," ")
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();

        // Parse event
        if let [_date, _hour, minute, word_1, word_2] = &parts[0..5] {
            match word_1.as_ref() {
                "Guard" => {
                    // Start of new shift
                    let guard: i32 = word_2.parse().unwrap();
                    let shift = Shift { guard: guard, mins: [ShiftStatus::Awake; 60] };
                    shifts.push(shift);
                },
                "falls" => {
                    // Guard falls asleep
                    let minute = minute.parse().unwrap();
                    let shift = shifts.last_mut().unwrap();
                    for min in &mut shift.mins[minute..] {
                        *min = ShiftStatus::Asleep
                    }
                },
                "wakes" => {
                    // Guard wakes up
                    let minute = minute.parse().unwrap();
                    let shift = shifts.last_mut().unwrap();
                    for min in &mut shift.mins[minute..] {
                        *min = ShiftStatus::Awake
                    }
                },
                _ => {
                    panic!("Unknown event")
                }
            }
        } else {
            panic!("Unable to parse event")
        }
    }

    return shifts
}

fn find_sleepiest_guard(shifts: &Vec<Shift>) -> i32 {

    // Calculate minutes asleep for each guard
    let mut sleep_totals = HashMap::new();
    for shift in shifts {
        *sleep_totals.entry(shift.guard).or_insert(0)
            += shift.count_mins(ShiftStatus::Asleep);
    }

    // Find the sleepiest guard
    sleep_totals.iter()
        .max_by(|(_,a), (_,b)| a.cmp(b))
        .unwrap()
        .0 // Guard ID
        .clone()
}

fn find_sleepiest_minute(shifts: &Vec<Shift>, guard: i32) -> i32 {

    // Calculate guard sleep totals for each minute
    let mut sleep_totals = [0; 60];
    for shift in shifts {
        if shift.guard != guard { continue }
        for min in 0..60 {
            if shift.mins[min] == ShiftStatus::Asleep {
                sleep_totals[min] += 1
            }
        }
    }

    // Find the sleepiest minute
    sleep_totals.iter()
        .enumerate()
        .max_by(|(_,a),(_,b)| a.cmp(b))
        .unwrap()
        .0 as i32 // index / minute
}

fn solve(lines: &Vec<String>) -> (i32, i32) {
    let lines: Vec<String> = lines.iter().sorted().cloned().collect();
    let shifts = create_shifts(&lines);
    let sleepiest_guard = find_sleepiest_guard(&shifts);
    let sleepiest_minute = find_sleepiest_minute(&shifts, sleepiest_guard);

    (sleepiest_guard, sleepiest_minute)
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Id: 131
 Minute: 36
 Answer: 4716
*/
run_without_benchmark!("day4", |lines: &Vec<String>| {
    let (id, minute) = solve(lines);
    println!("\nID: {}", id);
    println!("Minute: {}", minute);
    println!("Answer: {}\n", id * minute);
});

/*
 Timings:
    DEBUG: ~26.1ms
    RELEASE: ~1.77ms
*/
//run_with_benchmark!("day4", |lines| {
//    solve(lines);
//});