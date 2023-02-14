#![allow(unused, dead_code)]

fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s  = String::from("initial contents");

    // strings are UTF-8 enconded
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo ");
    s.push_str("bar ");
    s.push('🙄');
    let ch: char = '🙃';
    s.push(ch);

    println!("{s}");

    // using a slice after appending its contents to s1
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = "world!".to_string();

    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // The signature of + operator is something like
    // fn add(self, s: &str) -> String
    // thats is because we pass &str at second argument
    // obs: the compiler can coerce the &String argument into a &str[..]

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

   // let s = s1 + "-" + &s2 + "-" + &s3;

    let s = format!("{s1}-{s2}-{s3}");

    println!("{s}");


}