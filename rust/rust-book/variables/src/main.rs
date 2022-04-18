const THREE_HOURS_IN_SECONDS: u16 = 3 * 3_600;

fn main() {
    data_types();
    statements_and_expressions();
    println!("Plus one {}", plus_one(5));
}

#[allow(dead_code)]
fn scope_and_shadowing() {
    println!("The const val is {}", THREE_HOURS_IN_SECONDS);
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("Scope and shadowing");
    let x = 2;
    let x: u32 = x + 1;
    {
        let x = x.pow(3);
        println!("X in scope {}", x);
    }
    println!("X out of inner scope {}", x);
}

fn data_types() {
    #[allow(unused_variables)]
    let x: u16 = "42".parse().expect("This is not a number");
    let heart_eyed_cat: char = '😻';

    println!("Heej {}", heart_eyed_cat);
    let tuple: (i32, f64, u8) = (-10, 1.66, 8);
    println!("Tupla {:?}", tuple);

    let (_, x, _) = tuple;
    println!("middle el from tuple: {}", x);
    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("Tuple elements: {}, {}, {}", x, y, z);

    let array_of_strings: [&str; 5] = ["1", "2", "3", "4", "5"];
    println!("Array {:?}", array_of_strings);
}

fn statements_and_expressions() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("Y value from expr is {}", y);
}

fn plus_one(v: u32) -> u32 {
    v + 1
}
