use std::ops::Add;
use std::fmt;
mod methods_with_the_same_name;
mod outline_print;
mod functions;
mod macros;
use outline_print::OutlinePrint;
use std::ops::Deref;

fn main() {
    assert_eq!(
        Point {x: 10, y: 5} + Point { x: 42, y: -1 },
        Point {x: 52, y: 4},
    );
    let start_point = Point {x: 10, y: 20};
    let vector = (2, -8);
    let result = start_point + vector;
    assert_eq!(result, Point{x: 12, y: 12});

    methods_with_the_same_name::main();

    result.outline_print();
    let w = Wrapper(vec!["1".to_string(), "b".to_string(), "D".to_string()]);
    for i in w.iter() {
        println!("{}", i);
    }

    let answer = functions::do_twice(functions::add_one, 5);
    println!("Did add twice and the result is {}", answer);
    functions::closures_and_fns();
    functions::enum_as_fn();
    let _ = functions::returns_closure();
    macros::example();
    let room = Meters::<f32>(1.5);
    println!("My room is {:?} meters wide.", room);
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, other: (i32, i32)) -> Self::Output {
        Point {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    } 
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {}


struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

/// If we wanted the new type to have every method the inner type has, implementing the Deref trait 
/// (discussed in Chapter 15 in the “Treating Smart Pointers Like Regular References with the Deref Trait” section - <https://doc.rust-lang.org/book/ch15-02-deref.html#treating-smart-pointers-like-regular-references-with-the-deref-trait>) 
/// on the Wrapper to return the inner type would be a solution. 
/// If we don’t want the Wrapper type to have all the methods of the inner type—for example, 
/// to restrict the Wrapper type’s behavior—we would have to implement just the methods we do want manually.
impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

#[derive(Debug)]
struct Meters<T=i32>(T);
