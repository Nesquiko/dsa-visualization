use std::rc::Rc;

pub struct ImmutableLinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> ImmutableLinkedList<T> {
    pub fn new() -> Self {
        ImmutableLinkedList { head: None }
    }

    pub fn prepend(&self, val: T) -> ImmutableLinkedList<T> {
        ImmutableLinkedList {
            head: Some(Rc::new(Node {
                val,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> ImmutableLinkedList<T> {
        ImmutableLinkedList {
            head: self.head.as_ref().and_then(|n| n.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.val)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
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

impl<T> Drop for ImmutableLinkedList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::ImmutableLinkedList;

    #[test]
    fn immutable_linked_list() {
        let ill = ImmutableLinkedList::new();
        assert_eq!(ill.head(), None);

        let ill = ill.prepend(1).prepend(2).prepend(3);
        assert_eq!(ill.head(), Some(&3));

        let ill = ill.tail();
        assert_eq!(ill.head(), Some(&2));

        let ill = ill.tail();
        assert_eq!(ill.head(), Some(&1));

        let ill = ill.tail();
        assert_eq!(ill.head(), None);

        let ill = ill.tail();
        assert_eq!(ill.head(), None);
    }

    #[test]
    fn iter() {
        let ill = ImmutableLinkedList::new()
            .prepend(String::from("one"))
            .prepend(String::from("two"))
            .prepend(String::from("three"));

        let mut iter = ill.iter();
        assert_eq!(iter.next(), Some(&String::from("three")));
        assert_eq!(iter.next(), Some(&String::from("two")));
        assert_eq!(iter.next(), Some(&String::from("one")));
    }
}
