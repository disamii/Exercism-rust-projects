use std::{env::current_exe, os::unix::net::UnixDatagram};

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

// impl<T> SimpleLinkedList<T> {
//     pub fn new() -> Self {
//         SimpleLinkedList { head: None }
//     }

//     pub fn is_empty(&self) -> bool {
//         if let None = self.head {
//             return true;
//         }
//         false
//     }

//     pub fn len(&self) -> usize {
//         let mut count = 0;
//         let mut current = self.head.as_ref();
//         while let Some(node) = current {
//             count += 1;
//             current = node.next.as_ref();
//         }
//         count
//     }

//     pub fn push(&mut self, element: T) {

//         let mut current =  &mut self.head;

//         // we cant do 👇becuase we need container of the data not the insider
//         // let mut current =  self.head.as_mut();
//         // type mistmach

//         //   while if let Some(node) = current {
//         //             current=&mut node.next
//         //     }

//         // type matched but it is not owned we should borrow not move
//         //   while let Some(node)= *current {
//         //             current=&mut node.next
//         //     }

//         // now well passed but we cant borrow it as the mutate because we just read borrow
//         // while let Some(node)= current.as_ref() {
//         //         current= &mut node.next
//         // }

//         // rust doesnt know borrow end after the loop just becuase you do body
//         // while let Some(node)= current.as_mut() {
//         //         current=&mut node.next
//         // }

//         // *current = Some(Box::new(Node {
//         //     data: element,
//         //     next: None,
//         // }))
//                 // current.as_mut() → Option<&mut Box<Node<T>>>
//                 // .unwrap() → gets &mut Box<Node<T>> inside Some
//                 // .next → mutable reference to the next node (Option<Box<Node<T>>>)
//                 // &mut ... → now current points to the next node
//                 // Effectively moves current forward in the list mutably, one node at a time
//             // steps how we reached here
//             // 1. current=current.next....type mismatch we need mutable
//             // 2. current=&mut current.next....it didnt have next becuase it  is option
//             // 3. current=&mut current.un_wrap().next.....unwrap() moves the Option → cannot borrow curren
//             // 3. current=&mut current.as_mut().un_wrap().next....

//         while current.is_some() {
//             current = &mut current.as_mut().unwrap().next
//         }
//         *current = Some(Box::new(Node { data: element, next: None }));

//     }

//     pub fn pop(&mut self) -> Option<T> {
//         let mut current = &mut self.head;

//         if current.is_none() {
//             return None;
//         }

//         while current.as_ref().unwrap().next.is_some() {
//             current = &mut current.as_mut().unwrap().next
//         }
//         current.take().map(|node| node.data)
//     }

//     pub fn peek(&self) -> Option<&T> {
//         let mut current: &Option<Box<Node<T>>> = & self.head;
//         if current.is_none() {
//             return None;
//         }
//         // automatic dereferencing in patterns
//         // The Option is borrowed (&Option)
//         // Some(node) pattern matches → node is the value inside Some
//         // But you cannot move it (ownership rule) → Rust implicitly borrows it
//         // So node becomes &Box<Node<T>>
//         while let Some(node) = current {
//             if node.next.is_none() {
//                 return Some(&node.data);
//             }
//             current = &node.next;
//         }
//         None
//     }

// pub fn push(&mut self, element: T) {
//     let new_node = Box::new(Node {
//         data: element,
//         next: self.head.take(), // old head becomes next
//     });
//     self.head = Some(new_node); // new node becomes head
// }

// pub fn pop(&mut self) -> Option<T> {
//     self.head.take().map(|node| {
//         self.head = node.next;
//         node.data
//     })
// }

// pub fn peek(&self) -> Option<&T> {
//     if self.head.is_none() {
//         return None;
//     }
//     self.head.as_ref().map(|node| &node.data)
// }

//     #[must_use]
//     pub fn rev(self) -> SimpleLinkedList<T> {
//     }

// }

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }
    pub fn is_empty(&self) -> bool {
        if self.head.is_none() {
            return true;
        }
        false
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref();
        }
        count
    }

    pub fn push(&mut self, element: T) {
        let mut current: &mut Option<Box<Node<T>>> = &mut self.head;

        while current.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }
        *current = Some(Box::new(Node {
            data: element,
            next: None,
        }));
    }

    pub fn push_to_head(&mut self, element: T) {
        let new_data = Some(Box::new(Node {
            data: element,
            next: self.head.take(),
        }));
        self.head = new_data;
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut current = &mut self.head;
        if current.is_none() {
            return None;
        }
        while current.as_ref().unwrap().next.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }
        current.take().map(|node| node.data)
    }

    pub fn pop_from_head(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        let mut current = self.head.as_ref();
        if current.is_none() {
            return None;
        }
        while let Some(node) = current {
            if node.next.is_none() {
                return Some(&node.data);
            }
            current = node.next.as_ref();
        }
        None
    }

    pub fn peek_from_head(&self) -> Option<&T> {
        if self.head.is_none() {
            return None;
        }
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut current = self.head;
        let mut prev: Option<Box<Node<T>>> = None;
        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }
        SimpleLinkedList { head: prev }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut s = SimpleLinkedList::new();
        for item in _iter {
            s.push(item);
        }
        return s;
    }
}
impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v: Vec<T> = Vec::new();
        let mut current = _linked_list.head.take(); // take ownership of head

        while let Some(mut node) = current {
            v.push(node.data);      // move data out
            current = node.next.take(); // move next out
        }

        v

    }
}
// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.
