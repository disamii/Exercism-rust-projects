// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

pub struct Cursor<'a, T> {
    current: Option<&'a mut Node<T>>,
}
pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        if self.head.is_none() {
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        let mut current = self.head.as_ref();
        let mut count: usize = 0;
        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref();
        }
        count
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.head.as_deref_mut(),
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let mut current = self.head.as_deref_mut();

        while current.is_some() {
            current = current.unwrap().next.as_deref_mut();
        }
        Cursor { current }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head.as_deref(),
        }
    }
}

impl<T> Cursor<'_, T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.current.as_mut().map(|node| &mut node.data)
    }

    pub fn next(&mut self) -> Option<&mut T> {
        let current = self.current.take()?;
        self.current = current.next.as_deref_mut();
        self.current.as_mut().map(|node| &mut node.data)
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        let current = self.current.take()?;
        self.current = current.prev.as_deref_mut();
        self.current.as_mut().map(|node| &mut node.data)
    }

    pub fn take(&mut self) -> Option<T> 
    where
    T: Copy,{
        let current = self.current.take()?;
        self.current = current.next.as_deref_mut().or(current.prev.as_deref_mut());
        Some(current.data)
    }

    pub fn insert_after(&mut self, element: T) {
        let current = self.current.take().unwrap();

        let new = Box::new(Node {
            data: element,
            next: current.next.take(),
            prev: None,
        });
        current.next = Some(new);
        self.current = current.next.as_deref_mut();
    }

    pub fn insert_before(&mut self, _element: T) {
        let current = self.current.take().unwrap();

        let new = Box::new(Node {
            data: _element,
            next: None,
            prev: current.prev.take(),
        });
        current.prev = Some(new);
        self.current = current.prev.as_deref_mut();
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let current = self.current.take()?;
        self.current = current.next.as_deref();
        self.current.as_ref().map(|node| &node.data)
    }
}
