#[macro_use] mod common;
use self::common::*;

use itertools::Itertools;
use std::collections::HashMap;

// Types //////////////////////////////////////////////////////////////////////

enum Event {
    ShiftChangeTo(i32),
    WakeUpAt(i32),
    SleepAt(i32),
}

impl Event {
    fn parse(line: &String) -> Event {
        let parts= line
            .replace(|c| "[]:#".contains(c)," ")
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();

        if let [_date, _hour, minute, word_1, word_2] = &parts[0..5] {
            match word_1.as_ref() {
                "Guard" => {
                    let id: i32 = word_2.parse().unwrap();
                    Event::ShiftChangeTo(id)
                },
                "wakes" => {
                    let minute = minute.parse().unwrap();
                    Event::WakeUpAt(minute)
                },
                "falls" => {
                    let minute = minute.parse().unwrap();
                    Event::SleepAt(minute)
                },
                _ => {
                    panic!("Unknown event")
                }
            }
        } else {
            panic!("Unable to parse event")
        }
    }
}
struct Shift {
    guard: i32,
    events: Vec<Event> // Must be ordered and non-overlapping
}

impl Shift {
    fn is_asleep_at(&self, min: i32) -> bool {
        let mut is_asleep = false;
        for event in self.events.iter() {
            match event {
                Event::WakeUpAt(e_min) if is_asleep => {
                    if min < *e_min { return true } // Asleep before wake event
                    is_asleep = false;
                },
                Event::SleepAt(e_min) if !is_asleep => {
                    if min < *e_min { return false } // Awake before sleep event
                    is_asleep = true;
                },
                _ => panic!(),
            }
        }

        // State after final event
        return is_asleep;
    }
}

// Functions //////////////////////////////////////////////////////////////////

fn create_shifts<I>(events: I) -> Vec<Shift>
    where I : Iterator<Item=Event>
{
    let mut shifts = Vec::new();

    for event in events {
        match event {
            Event::ShiftChangeTo(id) => {
                shifts.push(Shift { guard: id, events: Vec::new() });
            },
            other => {
                let this_shift = shifts.last_mut().unwrap();
                this_shift.events.push(other);
            }
        }
    }

    return shifts
}

fn find_most_frequently_asleep(shifts: &Vec<Shift>) -> (i32, i32) {

    // Sum individual minutes asleep for each guard
    let mut sleep_totals = HashMap::new();
    for shift in shifts {
        let entry = sleep_totals.entry(shift.guard).or_insert([0; 60]);
        for min in 0..60 {
            if shift.is_asleep_at(min) { entry[min as usize] += 1 }
        }
    }

    // Find guard most frequently asleep on the same minute
    let mut best_guard = 0;
    let mut best_min = 0;
    let mut best_min_count = 0;
    for (&guard, mins) in sleep_totals.iter() {

        // Find minute most asleep for this guard
        let (this_min, &this_min_count) = mins.iter()
            .enumerate()
            .max_by(|(_,a),(_,b)| {
                a.cmp(b)
            })
            .unwrap();

        if this_min_count > best_min_count {
            best_guard = guard;
            best_min = this_min;
            best_min_count = this_min_count;
        }
    }

    (best_guard, best_min as i32)
}

fn solve(lines: &Vec<String>) -> (i32, i32) {

    let events = lines.iter()
        .sorted()
        .map(Event::parse);

    let shifts = create_shifts(events);
    let (guard, minute) = find_most_frequently_asleep(&shifts);

    (guard, minute)
}

// Entry Point ////////////////////////////////////////////////////////////////

run! {
    input = "day4",
    run = |input: &Input| {
        let (guard, minute) = solve(&input.to_lines());
        let answer = guard * minute;

        assert_eq!(guard, 2389);
        assert_eq!(minute, 49);
        assert_eq!(answer, 117061);

        println!("Guard: {}", guard);
        println!("Minute: {}", minute);
        println!("Answer: {}", answer);
    },
    bench = |input: &Input| {
        // DEBUG: ~29.3ms
        // RELEASE: ~1.89ms
        solve(&input.to_lines());
    }
}