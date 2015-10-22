fn main() {
    println!(">迭代器测试样例<");

    let mut v = vec![1, 2, 3, 4, 5];

    let v_len = v.len();
    println!("v.len() = {}", v_len);

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    for i in 0 .. v_len {
        println!("as array v[{}] = {}", i, v[i]);
    }

    // 以下代码会影响 v 的作用域,上面3行代码放到 i in v 块后,编译不过.
    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }

    let mut range = 0 .. 10;

    loop {
        match range.next() {
            Some(x) => {
                println!("current is: {}", x);
            },
                None => {break}
        }
    }

    for x in 10 .. 20 {
        println!(" -> {}", x);
    }

    let nums = vec![1, 2, 3];
    for i in 0 .. nums.len() {
        println!("index: {}, value: {}", i, nums[i]);
    }

    for num in &nums {
        println!("value: {}", num);
    }
}
