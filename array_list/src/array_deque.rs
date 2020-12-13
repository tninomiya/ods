use interface::list::List;
use std::fmt::Debug;

/// List implementation with backing array realized by boxed slice.
/// It is optimized for implementing deque interface.
/// O(1): get(i), set(i, x)
/// O(1 + n - i): add(i, x), remove(i)
pub struct ArrayDeque<T>
where
    T: Clone + Debug,
{
    a: Box<[Option<T>]>,
    j: usize,
    n: usize,
}

impl<T> ArrayDeque<T>
where
    T: Clone + Debug,
{
    /// Generate empty ArrayDeque
    pub fn new() -> Self {
        ArrayDeque {
            a: allocate_with(0).into_boxed_slice(),
            j: 0,
            n: 0,
        }
    }
}

impl<T> Default for ArrayDeque<T>
where
    T: Clone + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

fn allocate_with<T>(n: usize) -> Vec<Option<T>> {
    let mut array = Vec::with_capacity(n);
    unsafe {
        array.set_len(n);
    }
    array
}

impl<T> List<T> for ArrayDeque<T>
where
    T: Clone + Debug,
{
    fn size(&self) -> usize {
        unimplemented!();
    }
    fn get(&self, _i: usize) -> Option<&T> {
        unimplemented!();
    }
    fn set(&mut self, _i: usize, _x: T) -> Option<T> {
        unimplemented!();
    }
    fn add(&mut self, _i: usize, _x: T) {
        unimplemented!();
    }
    fn remove(&mut self, _i: usize) -> Option<T> {
        unimplemented!();
    }
}
