use std::fs::{File, OpenOptions, read_dir};
use std::fs;
use serde_json::{Value};
use chrono::{Local};
// use std::path::Path;
use std::io::{BufReader, sink};
use std::path::Path;
// use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{Source};


// 获取本地音乐文件列表
// fn get_music_list() {}

// 加载默认音乐文件路径
// fn load_music_file() {}

// 初始化系统设置
fn init_system_config() {
    let now = Local::now().format("%Y-%m-%d");
    let json = r#"
{
  "setting_date": '',
  "default_dir": "D://ru_music"
}
"#;
}


// 加载系统设置文件
fn load_system_config() {
    let config_path = Path::new("./config");   // 配置文件目录
    let config_name = Path::new("../config/setting.json");
    if !config_path.exists() {
        println!("Config Path is not existed");
        fs::create_dir(config_path).expect("TODO: panic message");
        File::create(config_name).expect("TODO: panic message");
        println!("Config Path has been created");
    } else {
        println!("Config Path is existed")
    }
}

fn play_music() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // OutputStream::try_default()
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file_list = vec![
        // "D:/fortunes.mp3",
        "D:/渡口.flac", "D:/WoLong.mp3"];

    for &files in file_list.iter() {
        let music_file = BufReader::new(File::open(files).unwrap());
        let source = Decoder::new(music_file).unwrap();

        sink.append(source);
        println!("正在播放：{}", &files);
        sink.sleep_until_end();
    }
}


fn main() {
    // load_system_config();
    // let json_file = OpenOptions::new().read(true).write(true).open("./config/setting.json");
    // let fmt = "";
}

fn read_json(raw_json: &str) -> Value {
    let parsed: Value = serde_json::from_str(raw_json).unwrap();
    return parsed;
}