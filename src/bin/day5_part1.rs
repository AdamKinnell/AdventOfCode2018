#[macro_use] mod common;
use self::common::*;

// Types //////////////////////////////////////////////////////////////////////

/*
 A movable cursor capable of traversing a SparseVector.
 Supports view, change, and remove.
*/
struct SparseVectorCursor<'a, T> {
    vec: &'a mut SparseVector<T>,
    pos: usize,
}

impl <'a,T> SparseVectorCursor<'a,T> {

    fn for_sparse_vec(vec: &'a mut SparseVector<T>) -> SparseVectorCursor<'a,T> {
        let first = vec.gaps[0];
        assert!(first < vec.elements.len()); // At least one element
        SparseVectorCursor { vec:vec, pos:first }
    }

    // Get ////////////////////////////////////////////////

    /*
     Get the value at the current cursor position.
    */
    pub fn get(&'a self) -> &'a T {
        &self.vec.elements[self.pos]
    }

    /*
     Get the mutable value at the current cursor position.
    */
    pub fn get_mut(&'a mut self) -> &'a mut T {
        &mut self.vec.elements[self.pos]
    }

    // Traverse ///////////////////////////////////////////

    /*
     Traverse the sparse array from the current position in the forward or backwards direction.
     The index of the next existent element in that direction will be returned.
    */
    fn traverse(&self, forward: bool) -> Option<usize> {
        // Move to adjacent element position
        // It may not exist sparsely, but must be within the vector bounds
        let adj_pos = if forward {
            if self.pos == self.vec.elements.len() - 1 { return None }
            self.pos + 1
        } else {
            if self.pos == 0 { return None }
            self.pos - 1
        };

        // Find gap to next existent element (distance from new_pos)
        let gap = self.vec.gaps[adj_pos];

        // Move to next existent element (If no gap, then move 0)
        // Out of bounds means cannot move in that direction.
        let new_pos = if forward {
            if adj_pos + gap > self.vec.elements.len() - 1 { return None };
            adj_pos + gap
        } else {
            if gap > adj_pos { return None }
            adj_pos - gap
        };

        // We must have landed on an existent element
        if self.vec.gaps[new_pos] != 0 { panic!("Ended in gap") }

        Some(new_pos)
    }

    // Move ///////////////////////////////////////////////

    fn move_cursor(&mut self, forward: bool) -> bool {
        let pos = self.traverse(forward);
        if let Some(pos) = pos {
            self.pos = pos;
            return true
        } else {
            return false
        }
    }

    /*
     Attempt to move the cursor to the previous element.
     Returns true, if the cursor was moved.
             false, if there is no previous element.
    */
    pub fn move_prev(&mut self) -> bool {
        self.move_cursor(false)
    }

    /*
     Attempt to move the cursor to the next element.
     Returns true, if the cursor was moved.
             false, if there is no next element.
    */
    pub fn move_next(&mut self) -> bool {
        self.move_cursor(true)
    }

    // Delete /////////////////////////////////////////////

    /*
     Mark the current element as removed, then update gap info.
     The cursor will be in an invalid state until moved to one of the returned positions.
     Algorithm is similar to removal and relinking in a doubly linked list.
    */
    fn remove_current(&mut self) -> (Option<usize>, Option<usize>) {

        // Get the index of prev (if exists)
        let prev = self.traverse(false);

        // Get the index of next (if exists)
        let next = self.traverse(true);

        // Get the index where prev->next is stored (or at start)
        let prev_next = prev
            .and_then(|p| Some(p + 1))
            .unwrap_or_else(|| 0);

        // Get the index where next->prev is stored (or at end)
        let next_prev = next
            .and_then(|n| Some(n - 1))
            .unwrap_or_else(|| self.vec.elements.len() - 1);

        // Find the distance to jump from &prev->next to next (and &next->prev to prev)
        let distance = next_prev - prev_next + 1;

        // Update "pointers"
        self.vec.gaps[prev_next] = distance;
        self.vec.gaps[next_prev] = distance;
        self.vec.gaps[self.pos]  = distance; // Don't let it be 0 (helps debug gap placement)

        (prev, next)
    }

    /*
     Remove the element at the current position then try to move the cursor backwards.
     If there is no previous element, the cursor will move forwards instead.
    */
    pub fn remove_then_prev(&mut self) {
        let (prev, next) = self.remove_current();
        self.pos = prev.unwrap_or_else(|| next.unwrap());
    }

    /*
     Remove the element at the current position then try to move the cursor forwards.
     If there is no next element, the cursor will move backwards instead.
    */
    pub fn remove_then_next(&mut self) {
        let (prev, next) = self.remove_current();
        self.pos = next.unwrap_or_else(|| prev.unwrap());
    }
}

/*
 A mutable view of a vector with elements missing.
 Supports view, change, and remove via cursor. Does not support insertion.
*/
struct SparseVector<T> {
    elements: Vec<T>,
    gaps: Vec<usize>,
}

impl <'a,T> SparseVector<T> {

    /*
     Create a sparse representation from a vector of existent elements.
    */
    pub fn from_vec(vec: Vec<T>) -> SparseVector<T> {
        assert!(vec.len() > 0);

        let mut gaps = Vec::new();
        gaps.resize(vec.len(), 0);

        SparseVector { elements:vec, gaps:gaps }
    }

    /*
     Get a cursor to traverse and modify sparse vector entries.
    */
    pub fn cursor(&'a mut self) -> SparseVectorCursor<'a, T> {
        assert!(self.elements.len() > 0);
        SparseVectorCursor::for_sparse_vec(self)
    }

    /*
     Get a vector of all existent elements.
    */
    pub fn iter(&'a self) -> impl Iterator<Item=&T> {
        self.elements.iter()
            .zip(&self.gaps)
            .filter(|(_, gap)| **gap == 0)
            .map(|(elem,_)| elem)
    }
}

// Functions //////////////////////////////////////////////////////////////////

/*
 Check if two chars are the same letter with opposing capitalization.
*/
fn can_react(a: char, b: char) -> bool {
    (a as i32 - b as i32).abs() == 32
}

/*
 Process polymer reactions until inert.
*/
fn react(polymer: &String) -> String {

    // Create sparse vector to represent polymer
    let polymer= polymer.clone()
        .chars()
        .collect::<Vec<char>>();
    let mut sparse_vec = SparseVector::from_vec(polymer);
    let mut cursor = sparse_vec.cursor();

    // Find and react pairs until inert
    loop {
        let left = *cursor.get();
        if !cursor.move_next() {
            break;
        }
        let right = *cursor.get();

        if can_react(left, right) {
            cursor.remove_then_prev(); // Remove right
            cursor.remove_then_prev(); // Remove left
            // Next iteration will compare either side of gap
        } else {
            // Next iteration will compare next pair
        }
    }

    sparse_vec.iter().collect::<String>()
}

/*
 Find length of polymer after all interactions have been resolved.
*/
fn solve(polymer: &String) -> usize {
    react(polymer).len()
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~28ms
    RELEASE: ~0.74ms
*/
run!{
    input = "day5",
    run = |input: &Input| {
        let units_remaining = solve(input.raw());
        assert_eq!(units_remaining, 11814);
        println!("Units Remaining: {}", units_remaining);
    },
    bench = |input: &Input| {
        solve(input.raw());
    }
}