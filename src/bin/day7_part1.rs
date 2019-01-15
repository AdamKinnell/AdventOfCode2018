#[macro_use] mod common;
use self::common::*;

use std::collections::BTreeMap;

// Functions //////////////////////////////////////////////////////////////////

/*
 Find the next step given a list of possible steps and their dependencies.
 The next step must not have any dependencies.
*/
fn find_next_step(steps: &BTreeMap<char, Vec<char>>) -> char {
    for (step, dependencies) in steps {
        if dependencies.is_empty() { return *step }
    }
    panic!("All steps have unsatisfied dependencies.")
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
 Find the order in which steps must be completed, based on dependencies.
*/
fn solve(instructions: Vec<String>) -> String {

    // Parse steps and their dependencies
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

    // Determine step ordering
    let mut completed_steps = Vec::new();
    while ! &steps.is_empty() {
        let step = find_next_step(&steps);

        // Perform step
        completed_steps.push(step);

        // Mark step as complete
        steps.remove(&step);
        remove_dependency(step, &mut steps);
    }

    completed_steps.into_iter()
        .collect::<String>()
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~860us
    RELEASE: ~37us
*/
run! {
    input = "day7",
    run = |input: &Input| {
        let step_order = solve(input.to_lines());
        assert_eq!(step_order, "HPDTNXYLOCGEQSIMABZKRUWVFJ");
        println!("Steps should be performed in the following order:\n\t{}", step_order);
    },
    bench = |input: &Input| {
        solve(input.to_lines());
    }
}