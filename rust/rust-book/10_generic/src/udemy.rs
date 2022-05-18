#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    let point = Point {
        x: format!("1"),
        y: format!("4"),
    };
    println!("{:?}", point)
}
