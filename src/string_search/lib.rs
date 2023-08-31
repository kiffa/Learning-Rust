use std::fs;
// 引入文件操作包
use std::env;
//
use std::error::Error; // 引入异常处理包

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 定义字符串搜索函数并开放外部访问
    // 入参为 query:待查询字符， contents：文本主体 返回值为Vector一维数组，元素类型为字符
    // 此处生命周期声明，该函数、入参contents、返回值的声明周期实质上相同
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 大小写不敏感/敏感文本搜索 此处入参与返回值作用与 search函数相同
    //此处生命周期声明使得result变量可在处理结束后返回结果
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试用例1 测试search函数
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    // 测试用例2 测试大小写不敏感用例
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }
}

pub struct Config {
    // 定义配置信息构造体并暴露
    pub query: String,
    // 待查询字符，字符串类型
    pub file_path: String,
    // 文件路径，字符串类型
    pub ignore_case: bool, // 字母大小写敏感配置，布尔类型
}

impl Config {
    // 为Config结构体创建方法
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        // 实例化方法 入参为参数迭代器
        args.next(); // 第一个参数无需处理，跳过

        // 迭代器第二位元素为查询参数
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"), // 错误处理
        };

        // 第三位为文件路径
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };


        // 通过读取环境变量获取是否忽略大小写状态
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // 无异常则直接返回数据
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 运行函数 入参为CONFIG结构体，在返回值中使用 Box<dyn Error> 获取报错信息
    // let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    let contents = fs::read_to_string(config.file_path)?; // 读取路径下的文件内容

    // 获取搜索结果
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    // println!("With text:\n{contents}");
    Ok(())
}
