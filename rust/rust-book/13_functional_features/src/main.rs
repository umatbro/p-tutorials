mod iterators;
use std::fmt::Display;
use std::{collections::HashMap, hash::Hash};
use iterators::{iter1, iterator_adaptors};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum ShirtColor {
    Red,
    Blue,
}

impl Display for ShirtColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ShirtColor::Blue => write!(f, "Blue"),
            ShirtColor::Red => write!(f, "Red"),
        }
    }
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn shirt_giveaway(&self, shirt_color: Option<ShirtColor>) -> ShirtColor {
        match shirt_color {
            Some(v) => v,
            None => self.get_most_common(),
        }
    }

    fn get_most_common(&self) -> ShirtColor {
        let mut count: HashMap<ShirtColor, u32> = HashMap::new();
        for i in &self.shirts {
            *count.entry(*i).or_insert(0) += 1;
        }

        let (top_shirt, top_shirt_count) = count
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap_or((&ShirtColor::Blue, &0));
        println!("Top shirt is {} with count {}", top_shirt, top_shirt_count);

        *top_shirt
    }

    // methods from the book
    #[warn(dead_code)]
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                &ShirtColor::Red => num_red += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }

    #[warn(dead_code)]
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    
}

fn main() {
    let inventory = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Blue,
        ],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = inventory.shirt_giveaway(user_pref1);
    println!("The user with preference {:?} gets {}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = inventory.shirt_giveaway(user_pref2);
    println!("The user with preference {:?} gets {}", user_pref2, giveaway2);
    // inventory.get_most_common();

    // mutable borrow showcase
    let mut list = vec![1, 2, 3];
    println!("Before closure declaration: {:?}", list);
    let borrows_mutably = move || {
        list.push(7);
        list
    };


    // println!("After defining closure: {:?}", list);
    let list = borrows_mutably();
    println!("After calling closure: {:?}", list);

    println!("Iter functions");
    iter1();
    iterator_adaptors();
}
