fn main() {
    let x = 10;

    println!("{} plus 2 is {}", x, add_two(x));
}

fn add_two(a: u32) -> u32 {
    a+2
}

#[test]
fn test_add_two() {
    let expected = 12;

    let actual = add_two(10);

    assert_eq!(expected, actual);
}

