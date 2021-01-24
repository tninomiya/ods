use interface::queue::Stack;
use std::fmt::Debug;

// ref: https://rust-unofficial.github.io/too-many-lists/index.html
type Link<T> = Option<Box<Node<T>>>;

/// Node owns its data and a reference to a next node.
#[derive(Debug)]
pub struct Node<T>
where
    T: Debug,
{
    element: T,
    next: Link<T>,
}

impl<T> Node<T>
where
    T: Debug,
{
    /// Generate empty Node
    pub fn new(x: T) -> Self {
        Node {
            element: x,
            next: None,
        }
    }

    fn new_link(x: T, next: Link<T>) -> Option<Box<Self>> {
        Some(Box::new(Node { element: x, next }))
    }
}

/// Singly-Linked List represents an implementation of List.
#[derive(Debug)]
pub struct SimpleSLList<T>
where
    T: Debug,
{
    head: Link<T>,
    n: usize,
}

impl<T> Default for SimpleSLList<T>
where
    T: Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SimpleSLList<T>
where
    T: Debug,
{
    /// Generate empty SimpleSLList
    pub fn new() -> Self {
        SimpleSLList { head: None, n: 0 }
    }
    /// Return the number of elements
    pub fn size(&self) -> usize {
        self.n
    }
}

impl<T> Stack<T> for SimpleSLList<T>
where
    T: Debug,
{
    fn push(&mut self, x: T) {
        let u = Node::new_link(x, self.head.take());
        self.head = u;
        self.n += 1;
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(n) => {
                self.head = n.next;
                self.n -= 1;

                Some(n.element)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_inner<T>(a: &Link<T>, e: Option<T>)
    where
        T: Debug + Eq,
    {
        match (a, e) {
            (None, None) => return,
            (Some(b), Some(f)) => assert_eq!(b.element, f),
            (b, f) => panic!(
                "assertion mismatch. left: `{:?}` should be equal to right: `{:?}`",
                b, f
            ),
        }
    }

    #[test]
    fn test_stack() {
        use interface::queue::Stack;

        let mut list: SimpleSLList<char> = SimpleSLList::new();
        list.push('a');
        list.push('b');
        list.push('c');
        list.push('d');
        list.push('e');
        assert_eq!(list.size(), 5);
        assert_inner(&list.head, Some('e'));

        assert_eq!(list.pop(), Some('e'));
        assert_eq!(list.size(), 4);
        assert_inner(&list.head, Some('d'));
        assert_eq!(list.pop(), Some('d'));
        assert_eq!(list.size(), 3);
        assert_inner(&list.head, Some('c'));

        assert_eq!(list.pop(), Some('c'));
        assert_eq!(list.size(), 2);
        assert_inner(&list.head, Some('b'));

        assert_eq!(list.pop(), Some('b'));
        assert_eq!(list.size(), 1);
        assert_inner(&list.head, Some('a'));

        list.push('x');
        assert_eq!(list.size(), 2);
        assert_inner(&list.head, Some('x'));

        assert_eq!(list.pop(), Some('x'));
        assert_eq!(list.size(), 1);
        assert_inner(&list.head, Some('a'));

        assert_eq!(list.pop(), Some('a'));
        assert_eq!(list.size(), 0);
        assert_inner(&list.head, None);

        assert_eq!(list.pop(), None);
        assert_eq!(list.size(), 0);
        assert_inner(&list.head, None);
    }
}
