#![allow(unused)]

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Call a message");
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x,
            y,
            radius,
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}


fn add_number<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}


fn main() {
    let rect_1 = Rectangle { width: 30, height: 50 };
    let rect_2 = Rectangle { width: 25, height: 10 };
    let rect_3 = Rectangle { width: 15, height: 45 };
    // println!("The area of the rectangle is {} square pixels", rect_1.area());
    println!("Can rect1 hold rect2 ? {}", rect_1.can_hold(&rect_2));
    println!("Can rect2 hold rect3 ? {}", rect_2.can_hold(&rect_3));
    let m = Message::Write(String::from("Hello Rust World"));
    m.call();
    println!("add i8: {}", add_number(2i8, 3i8));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 2.5, y: 1.7 };
    let p = Point{x:5,y:15};
    println!("p.x = {}", p.x());
}