use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let mut map = HashMap::new();

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using and see what compiler error you get

    //let _foo = field_name;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let entry = scores.entry(String::from("Blue"));
    entry.or_insert(50);

    scores.entry(String::from("Yellow")).or_insert(50);

    // for (k, v) in scores {
    //     println!("{k}: {v}");
    // }

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;

        print!("{word}: {:?}; ", map.get(word).copied().unwrap());
    }

}