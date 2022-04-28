mod vector;
mod strings;
mod hashmap;
mod tasks;

fn main() {
    vector::vec_basics();
    let mut person = vector::Person {name: String::from("Domi"), surname: String::from("Sz")};
    person.change_name("Dom");
    println!("Domee: {:?}", person);
    vector::reading_elements_of_vectors();
    vector::iter();
    vector::iter_enum();

    strings::basic();
    strings::concat();
    strings::indexing();

    hashmap::collect_example();
    hashmap::string_ownership();
    hashmap::accessing_values_in_hash_map();
    hashmap::iteration();
    hashmap::updating_hash_maps();

    println!("Performing tasks!");
    tasks::task1(&vec![1,1,1,1, 1, 3, 4, 3, 5,1,3,4]);
    println!("{}", tasks::task2(&String::from("apple")));
    println!("{}", tasks::task2(&String::from("first")));
}
