use interface::queue::Queue;
use std::fmt::Debug;

/// List implementation with backing array realized by boxed slice.
/// It is optimized for implementing fifo queue interface.
/// O(1): add(x), remove()
#[derive(Debug)]
pub struct ArrayQueue<T: Clone + Debug> {
    a: Box<[Option<T>]>,
    j: usize,
    n: usize,
}

impl<T> ArrayQueue<T>
where
    T: Clone + Debug,
{
    /// Generate empty ArrayQueue
    pub fn new() -> Self {
        ArrayQueue {
            a: allocate_with(0).into_boxed_slice(),
            j: 0,
            n: 0,
        }
    }

    fn capacity(&self) -> usize {
        self.a.len()
    }

    fn resize(&mut self) {
        let len = std::cmp::max(self.n * 2, 1);
        let mut new_array = allocate_with(len);

        for k in 0..self.n {
            new_array[k] = self.a[(self.j + k) % self.capacity()].take();
        }
        self.a = new_array.into_boxed_slice();
        self.j = 0;
    }

    fn size(&self) -> usize {
        self.n
    }
}

fn allocate_with<T>(n: usize) -> Vec<Option<T>> {
    let mut array = Vec::with_capacity(n);
    unsafe {
        array.set_len(n);
    }
    array
}

impl<T> Queue<T> for ArrayQueue<T>
where
    T: Clone + Debug,
{
    fn add(&mut self, x: T) -> bool {
        if self.size() + 1 > self.capacity() {
            self.resize();
        }
        let pos = (self.j + self.size()) % self.capacity();
        self.a[pos] = Some(x);
        self.n += 1;
        return true;
    }

    fn remove(&mut self) -> Option<T> {
        if self.n == 0 {
            return None;
        }
        let x = self.a[self.j].take();
        self.j = (self.j + 1) % self.capacity();
        self.n -= 1;
        return x;
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayQueue;
    use interface::queue::Queue;

    #[test]
    fn queue_test() {
        let mut queue: ArrayQueue<i32> = ArrayQueue::new();
        assert_eq!(queue.add(1), true);
        assert_eq!(queue.add(2), true);
        assert_eq!(queue.add(3), true);
        assert_eq!(queue.remove(), Some(1));
        assert_eq!(queue.remove(), Some(2));
        assert_eq!(queue.remove(), Some(3));
        assert_eq!(queue.remove(), None);
    }
}
