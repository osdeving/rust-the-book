#![allow(unused, dead_code)]

fn main() {
    // &i32 a reference
    // &'a i32 a reference with an explicit lifetime
    // &'a mut i32 a mutable reference with an explicit lifetime

    let string1 = "abcd".to_string();
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

/*

should be related to some parameter


fn foo<'a>(s1: &str, s2: &str) -> &'a str {
    String::from("Hello").as_str()
}

*/

