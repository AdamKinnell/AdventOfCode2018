#[macro_use] mod common;
use self::common::*;

use std::rc::Rc;
use std::cell::RefCell;

// Types //////////////////////////////////////////////////////////////////////

struct MarbleNode {
    value: usize,
    clockwise: Option<Rc<RefCell<MarbleNode>>>,
    counter_clockwise: Option<Rc<RefCell<MarbleNode>>>,
}

impl MarbleNode {

    /*
     Create a standalone marble.
    */
    fn create_first(value: usize) -> Rc<RefCell<MarbleNode>> {

        // Create node
        let marble = MarbleNode {
            value: value,
            clockwise: None,
            counter_clockwise: None
        };
        let marble_ref = Rc::new(RefCell::new(marble));

        // Link to self
        marble_ref.borrow_mut().clockwise = Some(marble_ref.clone());
        marble_ref.borrow_mut().counter_clockwise = Some(marble_ref.clone());

        marble_ref
    }

    /*
     Delete this marble node, and return the next node clockwise.
    */
    fn remove_then_clockwise(node: Rc<RefCell<MarbleNode>>) -> Rc<RefCell<MarbleNode>> {

        let clockwise = node.borrow().clockwise.clone();
        let counter_clockwise = node.borrow().counter_clockwise.clone();

        // Fix up neighbor link: [CCW] -> [CW]
        if let Some(ref counter_clockwise) = counter_clockwise {
            counter_clockwise.borrow_mut().clockwise = clockwise.clone()
        }

        // Fix up neighbor link: [CCW] <- [CW]
        if let Some(ref clockwise) = clockwise {
            clockwise.borrow_mut().counter_clockwise = counter_clockwise.clone()
        }

        // Remove references preventing deallocation
        node.borrow_mut().clockwise = None;
        node.borrow_mut().counter_clockwise = None;

        // All done: [CCW] <-> [CW]
        clockwise.as_ref().unwrap().clone()
    }

    /*
     Insert a new marble in the clockwise direction.
    */
    fn insert_clockwise(node: Rc<RefCell<MarbleNode>>, value: usize) -> Rc<RefCell<MarbleNode>> {

        let clockwise = node.borrow().clockwise.as_ref().unwrap().clone();
        let counterclockwise = node.clone();

        // Create node: [CCW] <- [New] -> [CW]
        let middle = MarbleNode {
            value: value,
            clockwise: Some(clockwise.clone()),
            counter_clockwise: Some(counterclockwise.clone())
        };
        let middle_ref = Rc::new(RefCell::new(middle));

        // Fix up neighbor link: [CCW] -> [New]
        counterclockwise.borrow_mut().clockwise = Some(middle_ref.clone());

        // Fix up neighbor link: [New] <- [CW]
        clockwise.borrow_mut().counter_clockwise = Some(middle_ref.clone());

        // All done: [CCW] <-> [New] <-> [CW]
        middle_ref
    }

    /*
     Get the marble clockwise n spaces.
    */
    fn move_clockwise(node: Rc<RefCell<MarbleNode>>, num: usize) -> Rc<RefCell<MarbleNode>> {
        let mut node = node.clone();
        for _ in 0..num {
            node = {
                let borrowed = node.borrow();
                borrowed.clockwise.as_ref().unwrap().clone()
            }
        }
        node
    }

    /*
     Get the marble counterclockwise n spaces.
    */
    fn move_counterclockwise(node: Rc<RefCell<MarbleNode>>, num: usize) -> Rc<RefCell<MarbleNode>> {
        let mut node = node.clone();
        for _ in 0..num {
            node = {
                let borrowed = node.borrow();
                borrowed.counter_clockwise.as_ref().unwrap().clone()
            }
        }
        node
    }

    /*
     Destroy all connected nodes and free memory.
    */
    fn destroy_all(node: Rc<RefCell<MarbleNode>>) {
        let mut node = node.clone();
        loop {
            let next = {
                node.borrow().clockwise.clone()
            };

            // Unlink node
            node.borrow_mut().clockwise = None;
            node.borrow_mut().counter_clockwise = None;

            if let Some(ref next) = next {
                // Move to next node
                node = next.clone();
            } else {
                // Back at start
                return;
            }
        }
    }
}

// Functions //////////////////////////////////////////////////////////////////

/*
 Play an elf marble game until completion.
*/
fn play(players: usize, marbles: usize) -> usize {
    let mut current_marble = MarbleNode::create_first(0);
    let mut player_scores = Vec::new();
    player_scores.resize(players, 0);

    // Play all marbles
    let mut current_player = 0;
    for next_marble in 1..marbles {

        // Take turn
        if next_marble % 23 == 0 {
            player_scores[current_player] += next_marble;
            current_marble = MarbleNode::move_counterclockwise(current_marble, 7);
            player_scores[current_player] += current_marble.borrow().value;
            current_marble = MarbleNode::remove_then_clockwise(current_marble);
        } else {
            // Insert marble between marbles 1 and 2 clockwise
            current_marble = MarbleNode::move_clockwise(current_marble, 1);
            current_marble = MarbleNode::insert_clockwise(current_marble, next_marble);
        }

        // Next player
        current_player = (current_player + 1) % players;
    }

    // Cleanup
    MarbleNode::destroy_all(current_marble);

    *player_scores.iter().max().unwrap()
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~17.5s
    RELEASE: ~955ms
*/
run! {
    input = "day9",
    run = |_: &Input| {
        let winning_score_a = play(416, 71617 + 1);
        assert_eq!(winning_score_a, 436720);
        println!("Winning Score A: {}", winning_score_a);

        let winning_score_b = play(416, (71617*100) + 1);
        assert_eq!(winning_score_b, 3527845091);
        println!("Winning Score B: {}", winning_score_b);
    },
    bench = |_: &Input| {
        play(416, 71617 + 1);
        play(416, (71617*100) + 1);
    }
}