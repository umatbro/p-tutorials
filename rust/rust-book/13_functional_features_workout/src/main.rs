use std::hash::Hash;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::fmt::Display;


struct Cacher <'a, K, V, F: Fn(&'a K) -> V> {
    values: HashMap<&'a K, V>,
    calculate: F,
}

impl <'a, K, V, F> Cacher<'a, K, V, F>
where
    F: Fn(&'a K) -> V,
    K: Hash + Eq,
    V: Copy + Display,
{
    pub fn new(calculate: F) -> Self {
        Self { values: HashMap::new(), calculate }
    }

    pub fn value(&mut self, arg: &'a K) -> V {
        if self.values.contains_key(arg) {
            let cached = self.values[arg];
            println!("Using cached value: {}", cached);
            cached
        } else {
            let calculated = (self.calculate)(arg);
            self.values.insert(arg, calculated);
            calculated
        }
    }
}

fn simulated_expensive_calculation(intensity: &u32) -> u32 {
    println!("Calculating slowly");
    thread::sleep(Duration::from_secs(2));
    intensity.clone()
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: &u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num.clone()
    };
    // function can be also provided if the trait is declared properly
    let mut _cacher = Cacher::new(simulated_expensive_calculation);
    let mut cacher = Cacher::new(expensive_closure);

    if intensity < 25 {
        let number_of_situps = intensity + 2;
        println!("Today, do {} pushups!", cacher.value(&intensity));
        println!("Next, do {} situps!", cacher.value(&number_of_situps));
        println!("And {} squats!", cacher.value(&intensity));
        println!("Did you forget how many situps? {}!", cacher.value(&number_of_situps));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cacher.value(&intensity));
        }
    }
}

fn main() {
    // simulated_expensive_calculation(3);
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
