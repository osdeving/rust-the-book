fn main() {
    let mut v : Vec<i32> = Vec::new();

    v.push(10);
    v.push(23);

    v[0] = 42;
    v[1] = 20;

    println!("{}", v[0]);

    let vec = vec![1, 2, 3];

    println!("{} {} {}", vec[0], vec[1], vec[2]);

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);


    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];

    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }


    //  cannot borrow one immutable element (&v[0])and mutable elment (v.push(6)) at same time.
    //  if v.push(6) to need allocate memory, reference to v[0] would be invalid.
    // {
    //     let mut v = vec![1,2,3,4,5];

    //     let first = &v[0];

    //     v.push(6);

    //     println!("The first element is: {first}");
    // }

    let v = vec![100, 23, 20];

    print!("Elements of vector v: ");
    for i in &v {
        print!("{i}, ");
    }

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    print!("\nElements of vector v after multiply by 50 each element: ");
    for i in &v {
        print!("{i}, ");
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

     print!("\nElements of vector v with different types using enum: ");
    for i in &row {
        match i {
            SpreadsheetCell::Int(value) => print!("{value}, "),
            SpreadsheetCell::Float(value) => print!("{value}, "),
            SpreadsheetCell::Text(value) => print!("{value}, "),
        }
    }
}
