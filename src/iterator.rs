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

    //let nums = vec![1, 2, 3];
    let nums = (1..10).collect::<Vec<i32>>();;
    for i in 0 .. nums.len() {
        println!("nums[{}] = {}", i, nums[i]);
    }

    for num in &nums {
        println!("value: {}", num);
    }

    let items = [1, 2, 3];

    for item in items.iter() {
        println!("items->item: {}", item);
    }

    let greater_than_forty_two = (0..100)
        .find(|x| *x > 42);
    match greater_than_forty_two {
        Some(_) => println!("We got some numbers! {:?}", greater_than_forty_two),
        None => println!("No numbers found :("),
    }

    let sum = (1..4).fold(0, |sum, x| sum + x);
    println!("sum is {}", sum);

    for i in (1..).take(5) {
        println!("take: {}", i);
    }

    for i in (1..15).filter(|&x| x % 2 == 0) {
        println!("{}", i);
    }

    let filter = (1..1000)
        .filter(|&x| x % 2 == 0)
        .filter(|&x| x % 3 == 0)
        .take(5)
        .collect::<Vec<i32>>();
    println!("filter: {:?}", filter);
}
