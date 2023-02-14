fn main() {
    let result = pig_latin("The best of the best first apple");
    println!("{result}");
}

fn pig_latin(text: &str) -> String {
    let mut result = String::new();

    for word in text.split_whitespace() {
        let letter = word.chars().nth(0);

        match letter {
            Some(ch) => {
                if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
                    result.push_str(word);
                    result.push_str("-hay");
                } else {
                    let mut s = word.to_string();

                    s.remove(0);
                    s.push('-');
                    s.push(ch);
                    s.push_str("ay ");

                    result.push_str(s.as_str());
                }
            }
            _ => (),
        }
    }
    result
}
