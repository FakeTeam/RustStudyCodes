use String;
fn main() {
    let s = String::from("hello");
    //这里已经移动了 相当于std::move()
    takes_ownship(s);
    let x = 5;
    makes_copy(x);
    //这里的值是copy的
    makes_copy(x);

    let s1 = gives_ownship();
    let s2 = String::from("hello");
    //s2被移动到函数 作用域消失
    let s3 = takes_and_gives_back(s2);
    println!("s1 {}  s3 {}", s1, s3);
}

fn takes_ownship(some_string: String) {
    println!("take {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("make {}", some_integer);
}

fn gives_ownship() -> String {
    let some_string = String::from("hello ");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
