use interface::queue::{Queue, Stack};
use std::cell::RefCell;
use std::fmt::Debug;
use std::mem;
use std::rc::Rc;

// ref: https://rust-unofficial.github.io/too-many-lists/index.html
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

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

    fn new_link(x: T, next: Link<T>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Node { element: x, next })))
    }
}

/// Singly-Linked List represents an implementation of List.
#[derive(Debug)]
pub struct SLList<T>
where
    T: Debug,
{
    head: Link<T>,
    tail: Link<T>,
    n: usize,
}

impl<T> Default for SLList<T>
where
    T: Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SLList<T>
where
    T: Debug,
{
    /// Generate empty SLList
    pub fn new() -> Self {
        SLList {
            head: None,
            tail: None,
            n: 0,
        }
    }
    /// Return the number of elements
    pub fn size(&self) -> usize {
        self.n
    }
}

impl<T> Stack<T> for SLList<T>
where
    T: Debug,
{
    fn push(&mut self, x: T) {
        let u = Node::new_link(x, mem::replace(&mut self.head, None)).unwrap();
        if self.n == 0 {
            self.tail = Some(u.clone()); // increment reference counter
        }
        self.head.replace(u);
        self.n += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.n == 0 {
            return None;
        }
        self.n -= 1;
        self.head.take().map(|h| {
            match h.borrow_mut().next.take() {
                Some(n) => self.head = Some(n),
                _ => self.tail = None,
            }
            Rc::try_unwrap(h).ok().unwrap().into_inner().element
        })
    }
}

impl<T> Queue<T> for SLList<T>
where
    T: Debug,
{
    fn add(&mut self, x: T) -> bool {
        let u = Node::new_link(x, None).unwrap();
        if self.n == 0 {
            self.head = Some(u.clone());
        }

        if let Some(t) = self.tail.take() {
            t.borrow_mut().next = Some(u.clone())
        }

        self.tail = Some(u);
        self.n += 1;
        true
    }

    fn remove(&mut self) -> Option<T> {
        if self.n == 0 {
            return None;
        }
        self.n -= 1;
        self.head.take().map(|h| {
            match h.borrow_mut().next.take() {
                Some(n) => self.head = Some(n),
                _ => self.tail = None,
            }
            Rc::try_unwrap(h).ok().unwrap().into_inner().element
        })
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
            (Some(b), Some(f)) => assert_eq!(b.borrow().element, f),
            (b, f) => panic!(
                "assertion mismatch. left: `{:?}` should be equal to right: `{:?}`",
                b, f
            ),
        }
    }

    #[test]
    fn test_stack() {
        use interface::queue::Stack;

        let mut list: SLList<char> = SLList::new();
        list.push('a');
        list.push('b');
        list.push('c');
        list.push('d');
        list.push('e');
        assert_eq!(list.size(), 5);
        assert_inner(&list.head, Some('e'));
        assert_inner(&list.tail, Some('a'));

        assert_eq!(list.pop(), Some('e'));
        assert_eq!(list.size(), 4);
        assert_inner(&list.head, Some('d'));
        assert_inner(&list.tail, Some('a'));
        assert_eq!(list.pop(), Some('d'));
        assert_eq!(list.size(), 3);
        assert_inner(&list.head, Some('c'));
        assert_inner(&list.tail, Some('a'));

        assert_eq!(list.pop(), Some('c'));
        assert_eq!(list.size(), 2);
        assert_inner(&list.head, Some('b'));
        assert_inner(&list.tail, Some('a'));

        assert_eq!(list.pop(), Some('b'));
        assert_eq!(list.size(), 1);
        assert_inner(&list.head, Some('a'));
        assert_inner(&list.tail, Some('a'));

        list.push('x');
        assert_eq!(list.size(), 2);
        assert_inner(&list.head, Some('x'));
        assert_inner(&list.tail, Some('a'));

        assert_eq!(list.pop(), Some('x'));
        assert_eq!(list.size(), 1);
        assert_inner(&list.head, Some('a'));
        assert_inner(&list.tail, Some('a'));

        assert_eq!(list.pop(), Some('a'));
        assert_eq!(list.size(), 0);
        assert_inner(&list.head, None);
        assert_inner(&list.tail, None);

        assert_eq!(list.pop(), None);
        assert_eq!(list.size(), 0);
        assert_inner(&list.head, None);
        assert_inner(&list.tail, None);
    }

    #[test]
    fn test_queue() {
        use interface::queue::Queue;

        let mut list: SLList<char> = SLList::new();
        list.add('a');
        assert_eq!(list.size(), 1);
        assert_eq!(list.remove(), Some('a'));
        list.add('a');
        list.add('b');
        list.add('c');
        list.add('d');
        list.add('e');
        assert_eq!(list.size(), 5);
        assert_inner(&list.head, Some('a'));
        assert_inner(&list.tail, Some('e'));

        assert_eq!(list.remove(), Some('a'));
        assert_eq!(list.size(), 4);

        assert_inner(&list.head, Some('b'));
        assert_inner(&list.tail, Some('e'));
        assert_eq!(list.remove(), Some('b'));
        assert_eq!(list.size(), 3);
        assert_inner(&list.head, Some('c'));
        assert_inner(&list.tail, Some('e'));

        assert_eq!(list.remove(), Some('c'));
        assert_eq!(list.size(), 2);
        assert_inner(&list.head, Some('d'));
        assert_inner(&list.tail, Some('e'));

        assert_eq!(list.remove(), Some('d'));
        assert_eq!(list.size(), 1);
        assert_inner(&list.head, Some('e'));
        assert_inner(&list.tail, Some('e'));

        list.add('x');
        assert_eq!(list.size(), 2);
        assert_inner(&list.head, Some('e'));
        assert_inner(&list.tail, Some('x'));

        assert_eq!(list.remove(), Some('e'));
        assert_eq!(list.size(), 1);
        assert_inner(&list.head, Some('x'));
        assert_inner(&list.tail, Some('x'));

        assert_eq!(list.remove(), Some('x'));
        assert_eq!(list.size(), 0);
        assert_inner(&list.head, None);
        assert_inner(&list.tail, None);

        assert_eq!(list.remove(), None);
        assert_eq!(list.size(), 0);
        assert_inner(&list.head, None);
        assert_inner(&list.tail, None);
    }
}
