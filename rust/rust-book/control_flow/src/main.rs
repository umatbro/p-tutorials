fn main() {
    let x = 73;
    if x >= 5 {
        println!("Var is gte 5");
    } else {
        println!("Var is lower than 5");
    }
    elif();
    loops();
    countdown();
}

fn elif() {
    let cond: bool = false;
    let var = if cond { "prawda" } else { "nieprawda" };
    println!("{}", var);
}

fn loops() -> i32 {
    let mut counter = 0;
    let result = loop {
        println!("Hello from loop ({})", counter);
        counter += 1;
        if counter > 10 {
            break counter;
        }
    };

    let a = [1, 3, 5, 7];
    for element in a {
        println!("{}", element);
    }
    return result;
}

fn countdown() {
    for num in (1..4).rev() {
        println!("{}!", num)
    }
    println!("LIFTOFF!!");
}
