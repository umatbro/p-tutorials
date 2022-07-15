use std::str::Bytes;

pub fn iter1() {
    let mut v1 = vec![1, 2, 3];
    let _v1_iter = v1.iter(); // immutable references
    let _v1_mut = v1.iter_mut(); // mutable references
    let v1_owned = v1.into_iter(); // takes ownership of the values in the iterator
    
    for val in v1_owned {
        println!("Got: {}", val);
    }
}

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Saver {
    fn save(file: Bytes) -> bool;
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}


#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {size:10, style: String::from("sneaker")},
        Shoe {size: 13, style: String::from("sandal")},
        Shoe {size: 10, style: String::from("boot")},
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe {size:10, style: String::from("sneaker")},
            Shoe {size:10, style: String::from("boot")},
        ]
    )
}



pub fn iterator_adaptors() {
    let numbers = Vec::from([1, 3, 5, 7]);
    let incremented: Vec<_> = numbers.iter().map(|x| x + 4).collect(); 

    let threshold = 10;
    let filtered: Vec<i32> = incremented.into_iter().filter(|x| x < &threshold).collect();
    println!("Filtered {:?}", filtered);
}