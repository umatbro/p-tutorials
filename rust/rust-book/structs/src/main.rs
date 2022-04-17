#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// Tuple structs
#[derive(Debug)]
struct Point(i32, i32, i32);
#[derive(Debug)]
struct Color(i32, i32, i32);

// unit-like structs
#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    let user = build_user(String::from("anotheruser@mail.com"), String::from("another"));
    print_type_of(&user);
    // println!("{:?}", user);

    // struct update syntax
    let user2 = User {
        email: String::from("ehh@example.com"),
        username: user.username[..].to_string(),
        ..user
    };
    print_type_of(&user);
    println!("user: {:?}, user2: {:?}", user, user2);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
    println!("{:?} {:?} {:?}", black, origin, subject);

    let rectangle = Rectangle {
        width: 10,
        height: 20,
    };
    let area = area(&rectangle);
    println!("Rectangle's area is {}", area);
    println!("Rectangle: {:?}", rectangle);
    let area2 = area_that_takes_ownership(rectangle);
    println!("Area 2: {}", area2);
    let r2 = Rectangle {width: 12, height: 13};
    println!("Area from method: {}", r2.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(10);
    println!("Square produced: {:?}", square);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn area(rectange: &Rectangle) -> u32 {
    rectange.height * rectange.width
}

fn area_that_takes_ownership(r: Rectangle) -> u32 {
    area(&r)
}