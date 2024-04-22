fn main() {
    use std::collections::HashMap;
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    scores.insert(String::from("yellow"), 25);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    println!("{:?}", scores);
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(60);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
