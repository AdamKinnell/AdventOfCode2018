
// SparseVector ///////////////////////////////////////////////////////////////

/*
 A mutable view of a vector with elements missing.
 Supports view, change, and remove via cursor. Does not support insertion.
*/
pub struct SparseVector<T> {
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

    // Traversal //////////////////////////////////////////

    /*
     Traverse the sparse array from the current position in the forward or backwards direction.
     The index of the next element in that direction will be returned (if one exists).
    */
    fn traverse(&self, start: usize, forward: bool) -> Option<usize> {

        // Move to adjacent element position
        // It may not exist sparsely, but must be within the vector bounds
        let adj_pos = if forward {
            if start == self.elements.len() - 1 { return None }
            start + 1
        } else {
            if start == 0 { return None }
            start - 1
        };

        // Find gap to next existent element (distance from new_pos)
        let gap = self.gaps[adj_pos];

        // Move to next existent element (If no gap, then move 0)
        // Out of bounds means cannot move in that direction.
        let new_pos = if forward {
            if adj_pos + gap > self.elements.len() - 1 { return None };
            adj_pos + gap
        } else {
            if gap > adj_pos { return None }
            adj_pos - gap
        };

        // We must have landed on an existent element
        if self.gaps[new_pos] != 0 { panic!("Traverse ended in gap") }

        Some(new_pos)
    }

    /*
     Get a cursor to traverse and modify sparse vector entries.
    */
    pub fn cursor(&'a mut self) -> SparseVectorCursor<'a, T> {
        let pos = self.first_index();
        SparseVectorCursor { vec:self, pos:pos }
    }

    /*
     Get a vector of all existent elements.
    */
    pub fn iter(&'a self) -> SparseVectorIterator<'a,T> {
        let pos = self.first_index();
        SparseVectorIterator { vec:self, pos:Some(pos) }
    }

    // Elements ///////////////////////////////////////////

    /*
     Get the index of the first existent element.
    */
    fn first_index(&self) -> usize {
        self.gaps[0] // Gap between start and first
    }
}

// SparseVectorIterator ///////////////////////////////////////////////////////

/*
 A forward iterator capable of traversing a SparseVector.
*/
pub struct SparseVectorIterator<'a,T> {
    vec: &'a SparseVector<T>,
    pos: Option<usize>,
}

impl <'a,T> Iterator for SparseVectorIterator<'a,T> {

    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(pos) = self.pos {
            let elem = &self.vec.elements[pos];
            self.pos = self.vec.traverse(pos, true); // Next element
            Some(elem)
        } else {
            None // End of iterator
        }
    }
}

// SparseVectorCursor /////////////////////////////////////////////////////////

/*
 A movable cursor capable of traversing a SparseVector.
 Supports view, change, and remove.
*/
pub struct SparseVectorCursor<'a, T> {
    vec: &'a mut SparseVector<T>,
    pos: usize,
}

impl <'a,T> SparseVectorCursor<'a,T> {

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
        self.vec.traverse(self.pos, forward)
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