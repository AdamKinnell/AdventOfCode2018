#[macro_use] mod common;
use self::common::*;

use itertools::Itertools;
use std::collections::HashMap;

// Types //////////////////////////////////////////////////////////////////////

enum EventInfo {
    ShiftChangeTo(i32),
    WakeUpAt(i32),
    SleepAt(i32),
}

struct Event {
    _date: String,
    info: EventInfo,
}

impl Event {
    fn parse(line: &String) -> Event {
        let parts= line
            .replace(|c| "[]:#".contains(c)," ")
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();

        if let [date, _hour, minute, word_1, word_2] = &parts[0..5] {
            match word_1.as_ref() {
                "Guard" => {
                    let id: i32 = word_2.parse().unwrap();
                    Event { _date:date.clone(), info: EventInfo::ShiftChangeTo(id) }
                },
                "wakes" => {
                    let minute = minute.parse().unwrap();
                    Event { _date:date.clone(), info: EventInfo::WakeUpAt(minute) }
                },
                "falls" => {
                    let minute = minute.parse().unwrap();
                    Event { _date:date.clone(), info: EventInfo::SleepAt(minute) }
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
    events: Vec<EventInfo> // Must be ordered and non-overlapping
}

impl Shift {

    fn is_asleep_at(&self, min: i32) -> bool {
        let mut is_asleep = false;
        for event in self.events.iter() {
            match event {
                EventInfo::WakeUpAt(e_min) if is_asleep => {
                    if min < *e_min { return true } // Asleep before wake event
                    is_asleep = false;
                },
                EventInfo::SleepAt(e_min) if !is_asleep => {
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
                EventInfo::WakeUpAt(m) => mins_asleep += m - sleep_at,
                EventInfo::SleepAt(m) => sleep_at = *m,
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
        use self::EventInfo::*;
        match event.info {
            ShiftChangeTo(id) => {
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
    DEBUG: ~25.4ms
    RELEASE: ~1.80ms
*/
//run_with_benchmark!("day4", |lines| {
//    solve(lines);
//});