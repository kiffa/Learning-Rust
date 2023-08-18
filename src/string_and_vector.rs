fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好！";
    let english = "World, Hello";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}


fn main() {
    let _greet_word = "Let's Go";
    greet_world();
    let penguin_data = "\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data
   ";

    let records = penguin_data.lines(); //按行读取并传输给records

    for (i, record) in records.enumerate() {
        // 对读取到的数据使用for循环，遍历每一行数据
        // i为下标，record为对应的下标数据
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        let test_fields: Vec<_> = record.split(',').collect();
        println!("test fields: {:?}", test_fields);

        // 将元素按照 ， 分割后再重新拼接，并赋值给数组
        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();
        println!("fields: {:?}", fields);

        // 输出debug信息
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0]; // 取数组第一个元素的值

        // 条件判断，当可以将值转换为32位浮点数的时候输出信息
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{},{}cm", name, length);
        }
    }
}