use std::thread::current;

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
        // error[E0507]: cannot move out of `self.head` which is behind a shared reference
        // let mut current = self.head;
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
        let mut current  = self.head.as_deref_mut();

        while current.is_some() {
            current = current.unwrap().next.as_deref_mut();
        }
        Cursor { current }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter{
            current:self.head.as_deref()
        }
        
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    pub fn next(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        todo!()
    }

    pub fn insert_after(&mut self, _element: T) {
        todo!()
    }

    pub fn insert_before(&mut self, _element: T) {
        todo!()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        todo!()
    }
}
