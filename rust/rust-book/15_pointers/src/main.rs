use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List: {:?}", list);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let v = 5;
    let w = Box::from(x);
    assert_eq!(5, v);
    assert_eq!(5, *w);

    let c1 = CustomSmartpointer { data: String::from("asdf")};
    let c2 = CustomSmartpointer { data: String::from("qwerty")};

    use_list();
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct CustomSmartpointer {
    data: String,
}

impl Drop for CustomSmartpointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
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
        let res = Box::from(node);
        self.next = Some(res);
    }
}

struct LinkedList<'a> {
    root: Option<Node>,
    _curr_iter: Option<&'a Node>,
}

impl <'a> LinkedList <'a> {
    fn new() -> Self {
        Self {root: None, _curr_iter: None}
    }

    fn insert(&mut self, value: i32) {
        let new_node= Node::from(value);
        
        if self.root.is_none() {
            println!("Root is none");
            self.root = Some(new_node);
            return;
        }
        let mut current = &mut self.root.as_ref().unwrap().next;
        while current.is_some() {
            let mut current = &current.as_ref().unwrap().next;
        }
        *current = Some(Box::new(new_node));
        // println!("Root is {}", self.root.as_ref().unwrap().value);
        let mut root_node = self.root.as_mut().unwrap();
        // let mut curr = self.root.as_ref();
        // let curr = root_node.next;
    //     // println!("First next: {:?}", next);
    //     if next.is_none() {
    //        *next.unwrap() = &Some(Box::from(new_node)); 
    //     }
        while true {
            if root_node.next.is_none() {
                // root_node.set_next(&new_node);
            }
        }
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
    let x = Node::from(2);
    child.set_next(x);
    parent.set_next(child);
    println!("Parent: {:?}", parent);
    // list.insert(2);
    // list.insert(3);
    // list.insert(6);
}

#[test]
fn insert_root() {
    let mut ll = LinkedList::new();
    ll.insert(1);
    assert_eq!(ll.root.unwrap().value, 1);
}