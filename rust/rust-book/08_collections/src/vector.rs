pub fn vec_basics() {
    let v: Vec<i32> = Vec::new();
    println!("Empty vec: {:?}", v);

    let v = vec![1, 3, 4];
    println!("This is vec macro result: {:?}", v);

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(3);
    add_to_vec(&mut v, Some(3));
    add_to_vec(&mut v, None);
    println!("Mutable vec: {:?}", v);
}

pub fn reading_elements_of_vectors() {
    //There are two ways to reference a value stored in a vector: 
    // * via indexing 
    //* using the get method. 
    let v = vec![1, 3, 5, 6];
    let third: &i32 = &v[2];
    println!("Third: {}", third);

    let i = 10;

    match v.get(i) {
        Some(val) => println!("Third value is {}", val),
        None => println!("Index {} not found. Vector size is {}", i, v.len()),
    }
}

fn add_to_vec(vec: &mut Vec<i32>, num: Option<i32>) {
    match num {
        Some(value) => vec.push(value),
        None => (),
    }
}

pub fn iter() {
    let mut v = vec![100, 200, 300];
    for i in &mut v {
        *i += 2;
    }
    println!("After mutation: {:?}", v);
}

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub surname: String,
}

impl Person {
    pub fn change_name(&mut self, name: &str) -> &Self {
        self.name = String::from(name);
        self
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn iter_enum() {
    let enum_vec = vec![
        SpreadsheetCell::Int(32),
        SpreadsheetCell::Float(3.6),
        SpreadsheetCell::Text(String::from("Analysis")),
    ];
    println!("Enum vec: {:?}", enum_vec);
}