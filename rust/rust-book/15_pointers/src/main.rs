use crate::List::{Cons, Nil};
use std::{ops::Deref, char::MAX};
use std::iter::Iterator;
use std::fmt::Debug;

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
struct Node <T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn from(value: T) -> Self {
        Self {value, next: None}
    }

    fn set_next(&mut self, node: Node<T>) {
        let res = Box::from(node);
        self.next = Some(res);
    }
}

#[derive(Debug)]
struct LinkedList<'a, T> {
    root: Option<Box<Node<T>>>,
    _curr_iter: Option<&'a Box<Node<T>>>,
}

impl <'a, T> LinkedList <'a, T> where T: Debug {
    fn new() -> Self {
        Self {root: None, _curr_iter: None}
    }

    fn insert(&mut self, value: T) {
        let new_node= Node::from(value);
        if self.root.is_none() {
            self.root = Some(Box::from(new_node));
            return;
        }
        // let mut root = self.root.as_mut().unwrap();
        let mut current = self.root.as_mut().unwrap();
        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }
        current.next = Some(Box::from(new_node));
    }
}

impl<'a, T> Iterator for LinkedList<'a, T> {
    type Item = T;

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

    let mut list_of_strings = LinkedList::new();
    list_of_strings.insert(String::from("abc"));
    list_of_strings.insert(String::from("Hej"));
    println!("Linked string list: {:?}", list_of_strings);

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
