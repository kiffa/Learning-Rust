fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn main() {
    // 打印程序开始执行语句
    // 定义变量使用 let，与JS、Vue相同
    let _greet_word = "Let's Go";
    println!("{}", _greet_word);

    let a = 10;  // 定义静态变量a, 令其值为0，编译器会自动识别其数据类型为i32（整数32位）
    let b: i32 = 20; // 定义静态变量b，并强制数据类型为i32并令其值为20

    let mut c = 30i32; // 定义动态变量c，令其值为30并使其类型为i32
    let d = 30_i32; // 定义变量d，作用与前者相同，但是可读性更佳

    //c是可变的，mut是mutable的缩写.
    // 带有 mut 标识的变量为动态变量，其数值可以改变，没有的则相反

    let e = add(add(a, b), add(c, d));
    print!("(a+b)+(c+d)={}", e);
}