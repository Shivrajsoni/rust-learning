use ::std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Shivraj"), 10);
    scores.insert(String::from("Sidesh"), 23);

    let team_name = String::from("Shivraj");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Score of team is -{}", score);

    for (key, value) in &scores {
        println!("{key} -> {value}");
    }

    scores.entry(String::from("Shivraj")).or_insert(53);
    println!("{scores:?}");
}
