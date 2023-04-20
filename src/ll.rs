pub struct LinkedList<T> {
    head: Link<T>,
}

// No overhead for putting Box into Option, because of the null pointer optimization
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, val: T) {
        let new = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(new);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.val
        })
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut cur = self.head.as_deref();
        for _ in 0..index {
            cur = cur.and_then(|n| n.next.as_deref());
        }
        cur.map(|n| &n.val)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let mut cur = self.head.as_deref_mut();
        for _ in 0..index {
            cur = cur.and_then(|n| n.next.as_deref_mut());
        }
        cur.map(|n| &mut n.val)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.val)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { ll: self }
    }

    // we can omit lifetime here, the lifetime elision is applied, but to not
    // hide that Iter contains a lifetime, we can use explicitly elided
    // lifetime '_
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

pub struct IntoIter<T> {
    ll: LinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ll.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // since Option<&> is copy it was just coppied and not moved
        self.next.map(|n| {
            // here we let compiler do the deref coercion by using the turbofish
            // operator ::<> to let rust know what type we want, this could
            // be also solved with as_deref()
            self.next = n.next.as_ref().map::<&Node<T>, _>(|node| &node);
            &n.val
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|n| {
            self.next = n.next.as_deref_mut();
            &mut n.val
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::LinkedList;

    #[test]
    fn linked_list() {
        let mut ll = LinkedList::new();
        assert_eq!(ll.pop(), None);

        ll.push(String::from("first"));
        ll.push(String::from("second"));
        ll.push(String::from("third"));

        assert_eq!(ll.pop(), Some(String::from("third")));
        assert_eq!(ll.pop(), Some(String::from("second")));

        ll.push(String::from("fourth"));
        ll.push(String::from("fifth"));

        assert_eq!(ll.pop(), Some(String::from("fifth")));
        assert_eq!(ll.pop(), Some(String::from("fourth")));

        assert_eq!(ll.pop(), Some(String::from("first")));
        assert_eq!(ll.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut ll = LinkedList::new();
        let expected = vec![4, 3, 2, 1];

        ll.push(1);
        ll.push(2);
        ll.push(3);
        ll.push(4);

        for (i, val) in ll.into_iter().enumerate() {
            assert_eq!(expected[i], val)
        }
    }

    #[test]
    fn iter() {
        let mut ll = LinkedList::new();
        let expected = vec![&4, &3, &2, &1];

        ll.push(1);
        ll.push(2);
        ll.push(3);
        ll.push(4);

        for (i, val) in ll.iter().enumerate() {
            assert_eq!(expected[i], val)
        }
        // also can do it second time
        for (i, val) in ll.iter().enumerate() {
            assert_eq!(expected[i], val)
        }
    }

    #[test]
    fn iter_mut() {
        let mut ll = LinkedList::new();
        let mut one = 1;
        let mut two = 2;
        let mut three = 3;
        let expected = vec![&mut three, &mut two, &mut one];

        ll.push(1);
        ll.push(2);
        ll.push(3);

        for (i, val) in ll.iter_mut().enumerate() {
            assert_eq!(expected[i], val)
        }

        // also can do it second time
        for (i, val) in ll.iter_mut().enumerate() {
            assert_eq!(expected[i], val)
        }
    }
}
