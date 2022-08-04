use crate::List::{Cons, Nil};
use std::{ops::Deref, char::MAX};
use std::iter::Iterator;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List: {:?}", list);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let v = 5;
    let w = Box::from(x);
    assert_eq!(5, v);
    assert_eq!(5, *w);

    let c1 = CustomSmartpointer { data: String::from("asdf")};
    let c2 = CustomSmartpointer { data: String::from("qwerty")};

    use_list();
    let z = 6;
    let my_box = MyBox(z);
    assert_eq!(6, z);
    assert_eq!(6, *my_box);
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

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
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

#[derive(Debug)]
struct LinkedList<'a> {
    root: Option<Box<Node>>,
    _curr_iter: Option<&'a Box<Node>>,
}

impl <'a> LinkedList <'a> {
    fn new() -> Self {
        Self {root: None, _curr_iter: None}
    }

    fn insert(&mut self, value: i32) {
        let new_node= Node::from(value);
        if self.root.is_none() {
            self.root = Some(Box::from(new_node));
            println!("Root has been set to {:?}", self.root);
            return;
        }
        // let mut root = self.root.as_mut().unwrap();
        let mut current = self.root.as_mut().unwrap();
        println!("first current is {:?}", current);
        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
            println!("New current is {:?}", current);
        }
        current.next = Some(Box::from(new_node));
    }
}

impl<'a> Iterator for LinkedList<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.root.is_none() {
            return None;
        }
        None
        // self._curr_iter = Some(self.root.as_deref().unwrap());
        // Some(self.root.as_ref().expect("Root must be set").value)
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
    list.insert(69);
    list.insert(2137);
    list.insert(1338);
    list.insert(42);
    println!("Linkded list: {:?}", list);
    // list.insert(6);
    for x in list {
        println!("List item: {}", x);
    }
}

#[test]
fn insert_root() {
    let mut ll = LinkedList::new();
    ll.insert(1);
    assert_eq!(ll.root.unwrap().value, 1);
}
