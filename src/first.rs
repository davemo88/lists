use std::mem;

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    fn pop_node(&mut self) -> Link {
        mem::replace(&mut self.head, Link::Empty)
    }

    fn pop(&mut self) -> Option<i32> {
        match self.pop_node() {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

//    pub fn pop(&mut self) -> Option<i32> {
//        match mem::replace(&mut self.head, Link::Empty) {
//            Link::Empty => None,
//            Link::More(node) => {
//                self.head = node.next;
//                Some(node.elem)
//            }
//        }
//    }
}

impl Drop for List {
    fn drop(&mut self) {
        while let Link::More(_) = self.pop_node() { };
//        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
//        while let Link::More(mut boxed_node) = cur_link {
//            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
//        }
//
   }
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basics() {
        let mut list = List::new();
        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

