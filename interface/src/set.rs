/// USet represents an unordered collection of unique elements.
pub trait USet<T>
where
    T: PartialEq + Eq,
{
    /// Return the length of a list.
    fn size(&self) -> usize;
    /// Add a value to a collection if it doesn't exist.
    /// Return true if it's ingested, otherwise return false.
    fn add(&mut self, x: T) -> bool;
    /// Remove a value from a collection.
    /// Return the element if a given one exists in a collection, otherwise return null(Empty).
    fn remove(&mut self, x: T) -> Option<T>;
    /// Return a value if a given one exists in a collection, otherwise return null(Empty).
    fn find(&self, x: T) -> Option<T>;
}

/// SSet represents an ordered collection of unique elements.
pub trait SSet<T>
where
    T: PartialEq + Eq + PartialOrd + Ord,
{
    /// Return the length of a list.
    fn size(&self) -> usize;
    /// Add a value to a collection if it doesn't exist.
    /// Return true if it's ingested, otherwise return false.
    fn add(&mut self, x: T) -> bool;
    /// Remove a value from a collection.
    /// Return the element if a given one exists in a collection, otherwise return null(Empty).
    fn remove(&mut self, x: T) -> Option<T>;
    /// Return a minimum value which satisfies condition: y >= x(x: given value).
    /// Called as successor search.
    fn find(&self, x: T) -> Option<T>;
}
