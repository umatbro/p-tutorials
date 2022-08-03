use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List: {:?}", list);

    use_list();
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn from(value: i32) -> Self {
        Self {value, next: None}
    }

    fn set_next(&mut self, node: Node) {
        self.next = Some(Box::from(node));
    }
}

struct LinkedList {
    root: Option<Node>
}

impl LinkedList {
    fn new() -> Self {
        Self {root: None}
    }

    fn insert(&mut self, value: i32) {
    //     let new_node= Node::from(value);
        
    //     if self.root.is_none() {
    //         println!("Root is none");
    //         self.root = Some(new_node);
    //         return;
    //     }

    //     println!("Root is {}", self.root.as_ref().unwrap().value);

    //     let mut next = &self.root.as_mut().unwrap().next;
    //     // println!("First next: {:?}", next);
    //     if next.is_none() {
    //        *next.unwrap() = &Some(Box::from(new_node)); 
    //     }

    //     while !next.is_some() {
    //         next = &next.as_ref().unwrap().next;
    //         println!("Next value is {:?}", next);
    //     }
    //     println!("No more next value ({:?})", next);
    //     next = &Some(Box::from(new_node));
    }
}

fn use_list() {
    let mut list = LinkedList::new();
    let mut parent = Node::from(3);
    let mut child = Node::from(5);
    parent.set_next(child);
    child.set_next(Node::from(2));
    println!("Parent: {:?}", parent);
    // list.insert(2);
    // list.insert(3);
    // list.insert(6);
}