fn main() {
    println!("Hello, world!");
    loop_example();
    while_example();
    for_example();
    range_example();
}

fn loop_example() {
    let mut cnt = 0;
    let result = loop {
        cnt += 1;
        if cnt == 10 {
            // 通过break返回值
            break cnt * 2;
        }
    };
    println!("The result = {}", result);
}

fn while_example() {
    let arr = [10, 20, 30, 40, 50];
    let mut idx = 0;
    while idx < 5 {
        println!("while the value is : {}", arr[idx]);
        idx += 1;
    }
}

fn for_example() {
    let arr = [10, 20, 30, 40, 50];
    for elem in arr.iter() {
        println!("for the value is : {}", elem);
    }
}

fn range_example() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
