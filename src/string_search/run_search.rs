use std::env; // 引入标准库-环境包
use std::process; // 引入 标准库-事件处理包

use minigrep::Config; // 从minigrep项目中引入Ccnfig包

fn main() {
    // 文件内字符串查询功能
    // let args: Vec<String> = env::args().collect();

    //当 Result 包含错误时，我们不再调用 panic 让程序崩溃，而是通过 process::exit(1) 来终结进程，
    // 其中 1 是一个信号值(事实上非 0 值都可以)，通知调用我们程序的进程，程序是因为错误而退出的。
    // unwrap_or_else 是定义在 Result<T,E> 上的常用方法，如果 Result 是 Ok，那该方法就类似 unwrap：
    // 返回 Ok 内部的值；如果是 Err，就调用闭包中的自定义代码对错误进行进一步处理
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {} In file {}", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
