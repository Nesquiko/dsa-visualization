use std::cell::RefCell;
use std::rc::Rc;

pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val,
            next: None,
            prev: None,
        }))
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    // when working with doubly linked list, every node must be pointed at
    // by two other (in case of a head/tail, they are pointed at by the list)
    pub fn push_front(&mut self, val: T) {
        let new_head = Node::new(val);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone()); // new_head +1 from old_head
                new_head.borrow_mut().next = Some(old_head); // old_head +1 from new_head
                self.head = Some(new_head); // old_head -1 from list, new_head +1 from list
                                            // new_head change total = +2 | old_head change total = 0
            }
            None => {
                self.tail = Some(new_head.clone()); // new_head +1 from list
                self.head = Some(new_head); // new_head +1 from list
                                            // new_head change total = +2
            }
        }
    }

    pub fn push_back(&mut self, val: T) {
        let new_tail = Node::new(val);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone()); // new_tail +1 from old_tail
                new_tail.borrow_mut().prev = Some(old_tail); // old_tail +1 from new_tail
                self.tail = Some(new_tail); // old_tail -1 from list.tail, new_tail +1 from list.tail
                                            // new_tail change total = +2 | old_tail change total = 0
            }
            None => {
                self.head = Some(new_tail.clone()); // new_head +1 from list.head
                self.tail = Some(new_tail); // new_head +1 from list.tail
                                            // new_head change total = +2
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // old_head -1 from list.head
        self.head.take().map(|old_head| {
            // new_head -1 from its previous
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take(); // old_head -1 from its next
                    self.head = Some(new_head); // new_head +1 from list.head
                                                // old_head change by -2 | new_head change by 0
                }
                None => {
                    self.tail.take(); // old_head -1 from list.tail
                                      // old_head change by -2
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        // old_tail -1 from list.tail
        self.tail.take().map(|old_tail| {
            // new_tail -1 from its previous
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take(); // old_tail -1 from its next
                    self.tail = Some(new_tail); // new_tail +1 from list.tail
                                                // old_tail change by -2 | new_tail change by 0
                }
                None => {
                    self.tail.take(); // old_tail -1 from list.tail
                                      // old_tail change by -2
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().val
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(DoublyLinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::DoublyLinkedList;

    #[test]
    fn basics() {
        let mut dll = DoublyLinkedList::new();

        // Check empty list behaves right
        assert_eq!(dll.pop_front(), None);

        // Populate list
        dll.push_front(1);
        dll.push_front(2);
        dll.push_front(3);

        // Check normal removal
        assert_eq!(dll.pop_front(), Some(3));
        assert_eq!(dll.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        dll.push_front(4);
        dll.push_front(5);

        // Check normal removal
        assert_eq!(dll.pop_front(), Some(5));
        assert_eq!(dll.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(dll.pop_front(), Some(1));
        assert_eq!(dll.pop_front(), None);
    }
}
