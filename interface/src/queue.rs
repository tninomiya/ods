/// Queue represents a collection of elements.
/// Provides manipulations for addition, removal with a certain rule.
pub trait Queue<T> {
    /// Add a value to a queue.
    fn add(&mut self, x: T) -> bool;
    /// Remove a next value and return it.
    fn remove(&mut self) -> Option<T>;
}

/// Stack represents LIFO queue.
pub trait Stack<T> {
    /// Add a value to the tail of a queue.
    fn push(&mut self, x: T);
    /// Remove a last-added value.
    fn pop(&mut self) -> Option<T>;
}

/// Dequeue represents double-ended queue.
pub trait Deque<T> {
    /// Add a value to the head of a queue.
    fn add_first(&mut self, x: T);
    /// Remove a value from the head of a queue.
    fn remove_first(&mut self) -> Option<T>;
    /// Add a value to the tail of a queue.
    fn add_last(&mut self, x: T);
    /// Remove a value from the tail of a queue.
    fn remove_last(&mut self) -> Option<T>;
}
