pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}

fn hello(list: &Link) -> () {
    match list {
        Link::Empty => {
            println!("I'm an empty list node thing")
        },
        Link::More(node) => {
            println!("I have an integer: {}", node.elem)
        }
    }
}

#[test]
fn test_hello() {
    let newList = Link::Empty;
    hello(&newList);
    let newNode = Node {
        elem: 25,
        next: newList,
    };
    let newnewList = Link::More(Box::new(newNode));
    hello(&newnewList);
}
