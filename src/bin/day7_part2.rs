#[macro_use] mod common;
use self::common::*;

use std::collections::BinaryHeap;
use std::collections::BTreeMap;

// Functions //////////////////////////////////////////////////////////////////

/*
 Find the next step without any listed dependencies.
 The search will occur in alphabetical order in case of ties.
*/
fn find_next_step(steps: &BTreeMap<char, Vec<char>>) -> Option<char> {
    for (step, dependencies) in steps {
        if dependencies.is_empty() { return Some(*step) }
    }
    None
}

/*
 Remove a step as a listed dependency from the list of given steps.

 This can be used to signify that a step is complete,
 and therefore other steps may now be available.
*/
fn remove_dependency(step: char, steps: &mut BTreeMap<char, Vec<char>>, ) {
    for dependencies in steps.values_mut() {
        dependencies.retain(|d| *d != step)
    }
}

/*
 Parse a list of steps and their dependencies from a list of instructions.
*/
fn parse_steps(instructions: &Vec<String>) -> BTreeMap<char, Vec<char>> {
    let mut steps = BTreeMap::new();

    for instruction in instructions {
        let mut capitals= instruction.chars()
            .filter(|c| *c >= 'A' && *c <= 'Z')
            .skip(1); // Skip 'S' in 'Step ...'

        let dependency = capitals.next().unwrap();
        let step = capitals.next().unwrap();

        // Add dependency to step
        steps.entry(step)
            .or_insert(Vec::new())
            .push(dependency);

        // Add dependant step.
        // Otherwise it won't appear if it has no dependencies itself.
        steps.entry(dependency)
            .or_insert(Vec::new());
    }

    steps
}

/*
 Find the total time taken to perform the dependent steps.

 A number of workers can operate in parallel,
 and each step can be assigned a unique completion time.
*/
fn schedule_work(steps: BTreeMap<char, Vec<char>>,
                 workers: i32,
                 f_timing: &Fn(char) -> i32) -> i32
{
    let mut time = 0;
    let mut workers = workers;

    // Work items (steps) are in one of three different states
    let mut available = steps;
    let mut working = BinaryHeap::new();
    let mut complete = Vec::new();

    // Schedule work until all steps are complete
    while !&available.is_empty() || !&working.is_empty() {

        // Assign steps to available workers
        while workers > 0 {
            let step = find_next_step(&available);
            if let Some(step) = step {
                // Schedule step completion
                let complete_at = time + f_timing(step);
                available.remove(&step);
                working.push(std::cmp::Reverse((complete_at, step)));

                workers -= 1;
            } else {
                // No more work available right now
                break
            }
        }

        // Jump to time of next step completion
        let std::cmp::Reverse((new_time, step)) = working.pop().unwrap();
        complete.push(step);
        remove_dependency(step, &mut available);
        time = new_time;
        workers += 1;
    }

    time
}

/*
 Find the order in which steps must be completed, based on dependencies.
*/
fn solve(instructions: &Vec<String>) -> i32 {
    schedule_work(parse_steps(&instructions), 5,
                  &|s| 60 + (s as i32 - 'A' as i32) + 1)
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~950us
    RELEASE: ~38us
*/
run! {
    input = "day7",
    run = |input: &Input| {
        let time_to_complete = solve(&input.to_lines());
        assert_eq!(time_to_complete, 908);
        println!("Total time to complete: {}", time_to_complete);
    },
    bench = |input: &Input| {
        solve(&input.to_lines());
    }
}