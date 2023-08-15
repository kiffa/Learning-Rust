// mod guess_number;

// use std::io;
#[derive(Debug)]
struct RectAngle {
    width: u32,
    height: u32,
}

impl RectAngle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &RectAngle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Hello Struct!");
    let rect1 = RectAngle {
        width: 30,
        height: 50,
    };

    let rect2 = RectAngle {
        width: 10,
        height: 40,
    };

    let rect3 = RectAngle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2?{}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?{}", rect1.can_hold(&rect3));

    let sq = RectAngle::square(3);


    // dbg!(rect1);
    // println!("The area of the retangle is {} square pixels", rect1.area());

    // if rect1.width() {
    //     println!("The rectangle has a nonzero width: it is {}", rect1.width);
    // }
}

