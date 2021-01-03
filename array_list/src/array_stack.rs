use interface::list::List;
use interface::queue::Stack;
use std::fmt::Debug;

/// List implementation with backing array realized by boxed slice.
/// It is optimized for implementing stack interface.
/// O(1): get(i), set(i, x)
/// O(1 + n - i): add(i, x), remove(i)
#[derive(Debug)]
pub struct ArrayStack<T: Clone + Debug> {
    a: Box<[Option<T>]>,
    n: usize,
}

impl<T> Default for ArrayStack<T>
where
    T: Clone + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ArrayStack<T>
where
    T: Clone + Debug,
{
    /// Generate empty ArrayStack.
    pub fn new() -> Self {
        ArrayStack::with_capacity(0)
    }

    fn with_capacity(i: usize) -> Self {
        ArrayStack {
            a: allocate_with(i).into_boxed_slice(),
            n: 0,
        }
    }

    // Return internally allocated capacity of backing array.
    fn capacity(&self) -> usize {
        self.a.len()
    }

    // Validate whether the given index is within a range of the list.
    fn within_bound(&self, i: usize) -> bool {
        i < self.a.len() && i < self.n
    }

    fn resize(&mut self) {
        let len = std::cmp::max(self.n * 2, 1);
        let mut new_array = allocate_with(len);

        for (i, elem) in self.a.iter_mut().enumerate().take(self.n) {
            new_array[i] = elem.clone();
        }
        self.a = new_array.into_boxed_slice();
    }
}

fn allocate_with<T>(n: usize) -> Vec<Option<T>> {
    let mut array = Vec::with_capacity(n);
    unsafe {
        array.set_len(n);
    }
    array
}

impl<T> List<T> for ArrayStack<T>
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
            self.a[i].as_ref()
        }
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        if !self.within_bound(i) {
            panic!(
                "index must be positive and less than the size of list. i: {}, n: {}",
                i, self.n
            )
        } else {
            self.a[i].replace(x)
        }
    }

    fn add(&mut self, i: usize, x: T) {
        if self.size() + 1 > self.capacity() {
            self.resize();
        }

        if i >= self.n {
            self.a[self.n] = Some(x);
        } else {
            self.a[i..self.n].rotate_right(1);
            let end = self.a[i].replace(x);
            self.a[self.n] = end;
        }
        self.n += 1;
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        let x = self.a.get_mut(i)?.take();
        if i < self.n {
            self.a[i..self.n].rotate_left(1);
        }
        self.n -= 1;
        if self.capacity() >= 3 * self.size() {
            self.resize();
        }

        x
    }
}

impl<T> Stack<T> for ArrayStack<T>
where
    T: Clone + Debug,
{
    fn push(&mut self, x: T) {
        self.add(self.size(), x);
    }
    fn pop(&mut self) -> Option<T> {
        self.remove(self.size() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayStack;
    use interface::list::List;
    use interface::queue::Stack;

    #[test]
    fn capacity_test() {
        // [Some(b), Some(r), Some(e), Some(d), None, None]
        let mut list: ArrayStack<char> = ArrayStack::with_capacity(6);
        assert_eq!(list.capacity(), 6);
        assert_eq!(list.size(), 0);
        list.add(0, 'b');
        list.add(1, 'r');
        list.add(2, 'e');
        list.add(3, 'd');
        assert_eq!(list.capacity(), 6);
        assert_eq!(list.size(), 4);

        list.add(2, 'e');
        assert_eq!(list.capacity(), 6);
        assert_eq!(list.size(), 5);
        list.add(5, 'r');
        assert_eq!(list.capacity(), 6);
        assert_eq!(list.size(), 6);
        list.add(5, 'e');
        assert_eq!(list.capacity(), 12);
        assert_eq!(list.size(), 7);

        assert_eq!(list.remove(4), Some('d'));
        assert_eq!(list.capacity(), 12);
        assert_eq!(list.size(), 6);
        assert_eq!(list.remove(4), Some('e'));
        assert_eq!(list.capacity(), 12);
        assert_eq!(list.size(), 5);
        assert_eq!(list.remove(4), Some('r'));
        assert_eq!(list.capacity(), 8);
        assert_eq!(list.size(), 4);
        assert_eq!(list.set(2, 'i'), Some('e'));
        // [Some(b), Some(r), Some(i), Some(e)]
        assert_eq!(list.get(0), Some(&'b'));
        assert_eq!(list.get(1), Some(&'r'));
        assert_eq!(list.get(2), Some(&'i'));
        assert_eq!(list.get(3), Some(&'e'));
    }

    #[test]
    fn list_test() {
        let mut list: ArrayStack<i32> = ArrayStack::new();
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

    #[test]
    fn stack_test() {
        let mut stack: ArrayStack<i32> = ArrayStack::new();
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.get(0), None);

        stack.push(2);
        assert_eq!(stack.get(0), Some(&2));
        assert_eq!(stack.size(), 1);

        stack.push(1);
        assert_eq!(stack.get(0), Some(&2));
        assert_eq!(stack.get(1), Some(&1));
        assert_eq!(stack.size(), 2);

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.get(0), Some(&2));
        assert_eq!(stack.size(), 1);

        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.get(0), None);
    }
}
