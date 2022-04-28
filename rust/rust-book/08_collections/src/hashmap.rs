use std::collections::HashMap;


// const TEAMS: HashMap<i32, i32> = HashMap::new();

fn get_scores() -> HashMap<String, i32> {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores
}

pub fn collect_example () {
    let teams = vec![String::from("Blue"), String::from("Red")];
    let scores = vec![10, 50];

    // let init_data = vec![(String::from("Yellow"), 30), (String::from("Green"), 40)];
    let scores: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();
    println!("{:?}", scores);
}

pub fn string_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("blue");

    let mut map_refs: HashMap<&String, &String> = HashMap::new();
    map_refs.insert(&field_name, &field_value);
    println!("Maps ref: {:?}", map_refs);
    println!("Name, value: {} - {}", field_name, field_value);

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(field_name, field_value);
    println!("MAP: {:?}", map);
    // println!("{}", field_name);  // not possible, because field_name is now owned by the hash map

}

pub fn accessing_values_in_hash_map() {
    let scores = get_scores();

    let score = scores.get("Blue");
    let team_name = String::from("Yellow");
    let score_yellow = scores.get(&team_name);
    println!("Scores are: {} - {}", score.unwrap_or(&-1), score_yellow.unwrap_or(&-1));
}

pub fn iteration() {
    let scores = get_scores();
    
    println!("Iteration!");
    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }
}

pub fn updating_hash_maps() {
    let mut scores = get_scores();
    // update existing key
    scores.insert(String::from("Yellow"), 30);
    println!("{:?}", scores);

   scores.entry(String::from("Yellow")).or_insert(20);
   scores.entry(String::from("Red")).or_insert(69);
   println!("{:?}", scores);

   let text = "hello world wonderfurl world";
   let mut map = HashMap::new();
   for word in text.split_whitespace() {
       let count = map.entry(word).or_insert(0);
       *count += 1;
   }
   println!("{:?}", map);
}