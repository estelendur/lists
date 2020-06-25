// TOOD: https://rust-unofficial.github.io/too-many-lists/second-into-iter.html

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// yay type aliases!
type Link<T> = Option<Box<Node<T>>>;

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to None
            // so no unbounded recursion occurs.
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

#[cfg(test)]
mod test {
    use super::{Link, List, Node};
    use std::fmt::Display;

    fn hello<T: Display>(list: &Link<T>) -> () {
        match list {
            None => println!("I'm an empty list node thing"),
            Some(node) => println!("I have an element: {}", node.elem),
        }
    }

    #[test]
    fn test_hello() {
        let new_list = None;
        hello(&new_list);
        let new_node = Node {
            elem: 25,
            next: new_list,
        };
        let newnew_list = Some(Box::new(new_node));
        hello(&newnew_list);
    }

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

    #[test]
    fn basics_str() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push("1");
        list.push("2");
        list.push("3");

        // Check normal removal
        assert_eq!(list.pop(), Some("3"));
        assert_eq!(list.pop(), Some("2"));

        // Push some more just to make sure nothing's corrupted
        list.push("4");
        list.push("5");

        // Check normal removal
        assert_eq!(list.pop(), Some("5"));
        assert_eq!(list.pop(), Some("4"));

        // Check exhaustion
        assert_eq!(list.pop(), Some("1"));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn peek_string() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push("1".to_string());
        list.push("2".to_string());
        list.push("3".to_string());

        assert_eq!(list.peek(), Some(&"3".to_string()));
        list.peek_mut().map(|value| *value = "42".to_string());

        assert_eq!(list.peek(), Some(&"42".to_string()));
        assert_eq!(list.pop(), Some("42".to_string()));
    }

    #[test]
    fn big_stack() {
        let mut list = List::new();
        for i in 1..1000000 {
            list.push(i);
        }

        // stack overflow in drop?
    }
}
