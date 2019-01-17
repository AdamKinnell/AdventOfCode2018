#[macro_use] mod common;
use self::common::*;

// Types //////////////////////////////////////////////////////////////////////

struct MarbleNode {
    value: usize,
    clockwise_i: usize,
    counterclockwise_i: usize,
}

struct  MarbleCircleCursor {
    marbles: Vec<MarbleNode>,
    position: usize,
}


impl MarbleCircleCursor {

    /*
     Create a new marble circle starting at the specified marble.
    */
    fn new(value: usize) -> MarbleCircleCursor {
        let node = MarbleNode {
            value: value,
            clockwise_i: 0,
            counterclockwise_i: 0,
        };

        MarbleCircleCursor {
            marbles: vec![node],
            position: 0,
        }
    }

    /*
     Delete this marble node, and move to the next node clockwise.
     Must not be the only node.
    */
    fn remove_then_clockwise(&mut self) {
        let node = &self.marbles[self.position];
        let clockwise_i = node.clockwise_i;
        let counterclockwise_i = node.counterclockwise_i;

        // Fix up pointers
        let counterclockwise = &mut self.marbles[counterclockwise_i];
        counterclockwise.clockwise_i = clockwise_i;

        let clockwise = &mut self.marbles[clockwise_i];
        clockwise.counterclockwise_i = counterclockwise_i;

        // Advance clockwise
        self.position = clockwise_i;
    }

    /*
     Insert a new marble in the clockwise direction.
    */
    fn insert_clockwise(&mut self, value: usize) {
        let clockwise_i = self.marbles[self.position].clockwise_i;
        let counterclockwise_i = self.position;

        // Create node
        let node = MarbleNode {
            value: value,
            clockwise_i: clockwise_i,
            counterclockwise_i: counterclockwise_i,
        };
        self.marbles.push(node);
        let middle_i = self.marbles.len() - 1;

        // Fix up pointers
        let counterclockwise = &mut self.marbles[counterclockwise_i];
        counterclockwise.clockwise_i = middle_i;

        let clockwise = &mut self.marbles[clockwise_i];
        clockwise.counterclockwise_i = middle_i;

        // Advance clockwise
        self.position = middle_i
    }

    /*
     Move the cursor to point to the marble clockwise n spaces.
    */
    fn move_clockwise(&mut self, num: usize) {
        for _ in 0..num {
            self.position = self.marbles[self.position].clockwise_i
        }
    }

    /*
     Move the cursor to point to the marble counter-clockwise n spaces.
    */
    fn move_counterclockwise(&mut self, num: usize) {
        for _ in 0..num {
            self.position = self.marbles[self.position].counterclockwise_i
        }
    }

    /*
     Get the value of the current marble under the cursor.
    */
    fn get(&self) -> usize {
        self.marbles[self.position].value
    }
}

// Functions //////////////////////////////////////////////////////////////////

/*
 Play an elf marble game until completion.
*/
fn play(players: usize, marbles: usize) -> usize {
    let mut marble_cursor = MarbleCircleCursor::new(0);
    let mut player_scores = Vec::new();
    player_scores.resize(players, 0);

    // Play all marbles
    let mut current_player = 0;
    for next_marble in 1..marbles {

        // Take turn
        if next_marble % 23 == 0 {
            player_scores[current_player] += next_marble;
            marble_cursor.move_counterclockwise(7);
            player_scores[current_player] += marble_cursor.get();
            marble_cursor.remove_then_clockwise();
        } else {
            // Insert marble between marbles 1 and 2 clockwise
            marble_cursor.move_clockwise(1);
            marble_cursor.insert_clockwise(next_marble);
        }

        // Next player
        current_player = (current_player + 1) % players;
    }

    *player_scores.iter().max().unwrap()
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~2.05s
    RELEASE: ~167ms
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