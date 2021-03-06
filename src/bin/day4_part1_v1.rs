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

    fn mins_asleep(&self) -> i32 {
        let mut mins_asleep = 0;
        let mut sleep_at = 0;
        for event in self.events.iter() {
            match event {
                Event::WakeUpAt(m) => mins_asleep += m - sleep_at,
                Event::SleepAt(m) => sleep_at = *m,
                _ => panic!(),
            }
        }
        return mins_asleep;
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

fn find_sleepiest_guard(shifts: &Vec<Shift>) -> i32 {

    // Calculate minutes asleep for each guard
    let mut sleep_totals = HashMap::new();
    for shift in shifts {
        *sleep_totals.entry(shift.guard).or_insert(0) += shift.mins_asleep();
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
            if shift.is_asleep_at(min) { sleep_totals[min as usize] += 1 }
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

    let events = lines.iter()
        .sorted()
        .map(Event::parse);

    let shifts = create_shifts(events);
    let sleepiest_guard = find_sleepiest_guard(&shifts);
    let sleepiest_minute = find_sleepiest_minute(&shifts, sleepiest_guard);

    (sleepiest_guard, sleepiest_minute)
}

// Entry Point ////////////////////////////////////////////////////////////////

run! {
    input = "day4",
    run = |input: &Input| {
        let (id, minute) = solve(&input.to_lines());
        let answer = id * minute;

        assert_eq!(id, 131);
        assert_eq!(minute, 36);
        assert_eq!(answer, 4716);

        println!("ID: {}", id);
        println!("Minute: {}", minute);
        println!("Answer: {}", answer);
    },
    bench = |input: &Input| {
        // DEBUG: ~26.3ms
        // RELEASE: ~1.84ms
        solve(&input.to_lines())
    }
}