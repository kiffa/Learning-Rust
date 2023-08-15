use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    // 定于答案数字变量， 使用rand包中thread_rag().gen_range() 生成随机数,与Python中的range类似
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is :{secret_number}");

    loop {
        println!("Please input your guess:");
        // mut 声明该变量可变
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // let guess: u32 = guess.trim().parse().expect("Please type a number !");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

        println!("Your guessed:{guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small !"),
            Ordering::Greater => println!("Too Big !"),
            Ordering::Equal => {
                println!("You Win !");
                break;
            }
        }
    }
}

