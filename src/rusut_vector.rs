use std::io;
#[derive(Debug)]
enum SpreadsheetCell {
    Int(u32),
    Float(f64),
    Text(String),
}

fn main() {
    println!("Hello Rust!");
    // let mut v: Vec<i32> = Vec::new(); // create new vevtor
    let mut v = vec![1, 2, 3, 4, 5];    // define a vector which habe 5 numeric elements
    println!("{:?}", v);

    // use two ways to get element
    let third: &u32 = &v[2];
    println!("The third element is {third}");
    let third: Option<&u32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // Iterating vector's elements and change values
    for i in &mut v {
        println!("{i}");
        *i += 50;
    }
    println!("{:?}", v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(1145.14),
    ];

    println!("{:?}", row);
}


