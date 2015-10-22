use std::io::Write;

fn main() {
    println!("不存在文件时,创建,然后写数据;如果存在,清空后再写");
    let mut f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        // .truncate(true)
        .open("foo.txt")
        .ok()
        .expect("Couldn’t open foo.txt");

    println!("{:?}", f);

    let buf = b"whatever just study rust.\n";
    let result = f.write(buf).ok().expect("Write fail.");

    println!("ret: {:?}", result);

    println!("追加内容");
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("foo.txt")
        .ok()
        .expect("Can NOT append data into file.");
    let buf = b"append some contents.\n";
    let result = f.write(buf).ok().expect("Append contents fail.");
    println!("append op ret: {}", result);
}
