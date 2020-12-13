use interface::list::List;
use std::fmt::Debug;

/// List implementation with backing array realized by boxed slice.
/// It is optimized for implementing deque interface.
/// O(1): get(i), set(i, x)
/// O(1 + min{i, n - i}): add(i, x), remove(i)
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

    fn capacity(&self) -> usize {
        self.a.len()
    }

    fn within_bound(&self, i: usize) -> bool {
        i < self.n
    }

    #[allow(clippy::needless_range_loop)]
    fn resize(&mut self) {
        let len = std::cmp::max(self.n * 2, 1);
        let mut new_array = allocate_with(len);

        for k in 0..self.n {
            new_array[k] = self.a[(self.j + k) % self.capacity()].take();
        }
        self.a = new_array.into_boxed_slice();
        self.j = 0;
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
        self.n
    }

    fn get(&self, i: usize) -> Option<&T> {
        if !self.within_bound(i) {
            None
        } else {
            self.a[(self.j + i) % self.capacity()].as_ref()
        }
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        if !self.within_bound(i) {
            panic!(
                "index must be positive and less than the size of list. i: {}, n: {}",
                i,
                self.size()
            )
        }
        self.a[(self.j + i) % self.capacity()].replace(x)
    }

    fn add(&mut self, i: usize, x: T) {
        if self.size() + 1 > self.capacity() {
            self.resize();
        }
        if i < self.size() / 2 {
            // swap to left for a[0]..=a[i-1]
            self.j = if self.j == 0 {
                self.capacity() - 1
            } else {
                self.j - 1
            };
            for k in 0..i {
                self.a[(self.j + k) % self.capacity()] =
                    self.a[(self.j + k + 1) % self.capacity()].take();
            }
        } else {
            // swap to right for a[i]..=a[n-1]
            for k in ((i + 1)..=self.size()).rev() {
                self.a[(self.j + k) % self.capacity()] =
                    self.a[(self.j + k - 1) % self.capacity()].take();
            }
        }
        self.a[(self.j + i) % self.capacity()].replace(x);
        self.n += 1;
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        let x = self.a.get_mut((self.j + i) % self.capacity())?.take();

        if i < self.size() / 2 {
            // swap to right for a[0]..=a[i-1]
            for k in (1..=i).rev() {
                self.a[(self.j + k) % self.capacity()] =
                    self.a[(self.j + k - 1) % self.capacity()].take();
            }
            self.j = (self.j + 1) % self.capacity();
        } else {
            // swap to left for a[i+1]..=a[n-1]
            for k in i..(self.size() - 1) {
                self.a[(self.j + k) % self.capacity()] =
                    self.a[(self.j + k + 1) % self.capacity()].take();
            }
        }
        self.n -= 1;
        if 3 * self.size() < self.capacity() {
            self.resize();
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayDeque;
    use interface::list::List;

    #[test]
    fn list_test() {
        let mut list: ArrayDeque<i32> = ArrayDeque::new();
        assert_eq!(list.size(), 0);
        assert_eq!(list.get(0), None);

        list.add(0, 2);
        assert_eq!(list.get(0), Some(&2));
        assert_eq!(list.size(), 1);

        list.add(0, 1);
        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.size(), 2);

        assert_eq!(list.remove(0), Some(1));
        assert_eq!(list.get(0), Some(&2));
        assert_eq!(list.size(), 1);

        assert_eq!(list.set(0, 5), Some(2));
        assert_eq!(list.get(0), Some(&5));

        assert_eq!(list.remove(0), Some(5));
        assert_eq!(list.size(), 0);
        assert_eq!(list.get(0), None);
    }
}
