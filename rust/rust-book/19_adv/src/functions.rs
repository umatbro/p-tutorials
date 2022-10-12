pub fn add_one(num: i32) -> i32 {
    num + 1
}

pub fn do_twice(action: fn(i32) -> i32, arg: i32) -> i32 {
    action(arg) + action(arg)
}

pub fn closures_and_fns() {
    let numbers = vec![1, 2, 3];
    let as_strings: Vec<String> = numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();
    println!("As strings: {:?}", as_strings);
    println!("Using a function {:?}", list_of_strings);
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

pub fn enum_as_fn() {
    let statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("Statuses: {:?}", statuses);
}

pub fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}