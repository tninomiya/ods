/// List provides interface manipulation for a collection of elements.
pub trait List<T> {
    /// Return the length of a list.
    fn size(&self) -> usize;
    /// Return the value at the position i.
    fn get(&self, i: usize) -> Option<&T>;
    /// Set the value at the position i.
    fn set(&mut self, i: usize, x: T) -> Option<T>;
    /// Add a value at the position i, and shift following elements to backward.
    fn add(&mut self, i: usize, x: T);
    /// Remove a value at the position i, and shift following elements to forward.
    fn remove(&mut self, i: usize) -> Option<T>;
}
