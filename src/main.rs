use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T;
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}


pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("文章{}, 作者：{}", self.title, self.author)
//     }
//     fn summarize_author(&self) -> String {
//
//     }
// }

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{} 发表了微博 {}", self.username, self.content)
    }
}

fn main() {
    // let post_1 = Post {
    //     title: "Rust Language Brifing".to_string(),
    //     author: "Surface".to_string(),
    //     content: "Rust is the BEST!".to_string(),
    // };
    let weibo_1 = Weibo {
        username: "Surface".to_string(),
        content: "X is better than Weibo".to_string(),
    };

    // println!("1 new weibo: {}", weibo_1.summarize());
    // notify(weibo_1);
}