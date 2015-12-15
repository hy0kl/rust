use std::io::Write;

fn main() {
    println!("不存在文件时,创建,然后写数据;如果存在,清空后再写");

    let mut f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)   // 目标文件不存在时创建新文件.不加此选项,打开不存在文件时会报错
        .truncate(true) // 不加此行,如果文件存在,不清空原文件,会 seek 到文件件首部
        .open("foo.txt")
        .ok()
        .expect("Couldn’t open foo.txt");
    println!("{:?}", f);

    let buf = b"Whatever just study rust.\n";
    let result = f.write(buf).ok().expect("Write fail.");
    println!("ret: {:?}", result);

    // ---

    println!("追加内容");

    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .append(true)   // 追加选项
        .open("foo.txt")
        .ok()
        .expect("Can NOT append data into file.");
    let buf = b"Append some contents.\n";
    let result = f.write(buf)
        .ok()
        .expect("Append contents fail.");
    println!("append op ret: {}", result);
}

