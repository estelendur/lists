pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

// yay type aliases!
type Link = Option<Box<Node>>;

impl Drop for List {
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

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::{Link, List, Node};

    fn hello(list: &Link) -> () {
        match list {
            None => println!("I'm an empty list node thing"),
            Some(node) => println!("I have an integer: {}", node.elem),
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
    fn big_stack() {
        let mut list = List::new();
        for i in 1..1000000 {
            list.push(i);
        }

        // stack overflow in drop?
    }
}
