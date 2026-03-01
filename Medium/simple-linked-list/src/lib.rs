use std::env::current_exe;

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        if let None = self.head {
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

        let mut current =  &mut self.head;

        // we cant do 👇becuase we need container of the data not the insider
        // let mut current =  self.head.as_mut();
        // type mistmach 
        
        //   while let Some(node)= current {
        //             current=&mut node.next
        //     }

        // type matched but it is not owned we should borrow not move
        //   while let Some(node)= *current {
        //             current=&mut node.next
        //     }

        // now well passed but we cant borrow it as the mutate beucase we just read borrow
        // while let Some(node)= current.as_ref() {
        //         current=&mut node.next
        // }

        // rust doesnt know borrow end after the loop just becuase you do body
        // while let Some(node)= current.as_mut() {
        //         current=&mut node.next
        // }
        
        // *current = Some(Box::new(Node {
        //     data: element,
        //     next: None,
        // }))


        while current.is_some() {
            current = &mut current.as_mut().unwrap().next
        }
        *current = Some(Box::new(Node { data: element, next: None }));
        
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut current = &mut self.head;

        if current.is_none() {
            return None;
        }

        while current.as_ref().unwrap().next.is_some() {
            current = &mut current.as_mut().unwrap().next
        }
        current.take().map(|node| node.data)
    }

    pub fn peek(&self) -> Option<&T> {
        let mut current: &Option<Box<Node<T>>> = & self.head;
        if current.is_none() {
            return None;
        }
        
        // no moving out becuase it just borrowing not 
        while let Some(node) = current {
            if node.next.is_none() {
                return Some(&node.data);
            }
            current = &node.next;
        }
        None
    }

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

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        todo!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        todo!()
    }
}
impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        todo!()
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
