use std::ops::Index;
use std::slice::Iter;

/// Hollow usize vector sorted with generic indices
///
/// Allow manipulation of big vector which contains a lot of zeroes.
/// This type of vector is very useful to represent the connection between locations and
/// transitions in order to avoid creating a matrix mainly filled with zeros.
#[derive(Debug, PartialEq, Default, Clone)]
pub struct Marking<T: Ord + Copy> {
    values: Vec<(T, usize)>,
}

impl<T> Index<T> for Marking<T>
where
    T: Ord + Copy,
{
    type Output = usize;

    fn index(&self, index: T) -> &Self::Output {
        // Search for the index in the array with complexity O(ln(n))
        // If the index is found, returns the value stored there, otherwise returns zero.
        match self.values.binary_search_by(|&v| v.0.cmp(&index)) {
            Ok(pos) => &self.values[pos].1,
            Err(_) => &0,
        }
    }
}

/// Iterator over a fusion of two markings, allows to iter over two markings and check equality
/// without searching all values for example
pub struct DualMarkingIterator<'m, T>
where
    T: Ord + Copy,
{
    /// Counter of the left marking
    current_left: usize,
    /// Counter of the right marking
    current_right: usize,
    /// Reference to the left marking
    left: &'m Marking<T>,
    /// Reference to the right marking
    right: &'m Marking<T>,
}

impl<'m, T> Iterator for DualMarkingIterator<'m, T>
where
    T: Ord + Copy,
{
    type Item = (T, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        // We reach the end of both iterators
        if self.current_left >= self.left.len() && self.current_right >= self.right.len() {
            None
        } else if self.current_left >= self.left.len() {
            // We reach the end of the left iterator, so we continue with values of the right iterator
            self.current_right += 1;
            Some((
                self.right.values[self.current_right - 1].0,
                0,
                self.right.values[self.current_right - 1].1,
            ))
        } else if self.current_right >= self.right.len() {
            // We reach the end of the right iterator, so we continue with values of the left iterator
            self.current_left += 1;
            Some((
                self.left.values[self.current_left - 1].0,
                self.left.values[self.current_left - 1].1,
                0,
            ))
        } else if self.left.values[self.current_left].0 == self.right.values[self.current_right].0 {
            // Both index of each iterators are equals, so we return both values
            self.current_left += 1;
            self.current_right += 1;
            Some((
                self.left.values[self.current_left - 1].0,
                self.left.values[self.current_left - 1].1,
                self.right.values[self.current_right - 1].1,
            ))
        } else if self.left.values[self.current_left].0 < self.right.values[self.current_right].0 {
            // Left index is less than right index, so we increment left counter and return its value
            self.current_left += 1;
            Some((
                self.left.values[self.current_left - 1].0,
                self.left.values[self.current_left - 1].1,
                0,
            ))
        } else if self.left.values[self.current_left].0 > self.right.values[self.current_right].0 {
            // Right index is less than left index, so we increment right counter and return its value
            self.current_right += 1;
            Some((
                self.left.values[self.current_right - 1].0,
                0,
                self.right.values[self.current_right - 1].1,
            ))
        } else {
            None
        }
    }
}

impl<T> Marking<T>
where
    T: Ord + Copy,
{
    /// Return a iterator over all present elements in the marking
    #[must_use]
    pub fn iter(&self) -> Iter<'_, (T, usize)> {
        self.values.iter()
    }

    /// Return an iterator over two marking at same time.
    /// Useful to compare marking with a complexity of O(2n) instead of O(2nln(n))
    #[must_use]
    pub fn iter_with<'m>(&'m self, other: &'m Self) -> DualMarkingIterator<'m, T> {
        DualMarkingIterator {
            current_left: 0,
            current_right: 0,
            left: self,
            right: other,
        }
    }

    /// Remove all items in marking
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Returns the number of elements in the marking.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns [`true`] if the vector contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Increment the value contained in the marking by weight.
    ///
    /// If the tag does not contain a value associated with the index, weight is inserted into the
    /// index.
    pub fn insert_or_add(&mut self, index: T, weight: usize) {
        match self.values.binary_search_by(|&v| v.0.cmp(&index)) {
            Ok(pos) => {
                if self.values[pos].1 == 0 && weight != 0 {}
                self.values[pos].1 += weight;
            }
            Err(pos) => {
                self.values.insert(pos, (index, weight));
            }
        }
    }

    /// Keeps the minimum value between that contained in the marking and weight.
    ///
    /// If the tag does not contain a value associated with the index, weight is inserted into the index.
    pub fn insert_or_min(&mut self, index: T, weight: usize) {
        match self.values.binary_search_by(|&v| v.0.cmp(&index)) {
            Ok(pos) => {
                if weight == 0 && self.values[pos].1 != 0 {}
                self.values[pos].1 = self.values[pos].1.min(weight);
            }
            Err(pos) => {
                self.values.insert(pos, (index, weight));
            }
        }
    }

    /// Keeps the maximum value between that contained in the marking and weight.
    ///
    /// If the tag does not contain a value associated with the index, weight is inserted into the index.
    pub fn insert_or_max(&mut self, index: T, weight: usize) {
        match self.values.binary_search_by(|&v| v.0.cmp(&index)) {
            Ok(pos) => {
                self.values[pos].1 = self.values[pos].1.max(weight);
            }
            Err(pos) => {
                self.values.insert(pos, (index, weight));
            }
        }
    }

    /// Delete a specific index from the marking
    pub fn delete(&mut self, index: T) {
        if let Ok(index) = self.values.binary_search_by(|&v| v.0.cmp(&index)) {
            self.values.remove(index);
        }
    }
}
