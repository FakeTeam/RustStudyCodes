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
    mutli_get_mutable_ref();
    // let mut no_slience_str = String::from("hello world");
    let no_slience_str = String::from("hello world");
    let _first = slience_first_word(&no_slience_str[..]);
    let first = slience_first_word(&no_slience_str[0..6]);
    let first_world = no_slience_first_word(&no_slience_str);
    // 这里没有所有权拉
    // no_slience_str.clear();
    println!("first world  {} {}", first_world, first);
}

fn mutli_get_mutable_ref() {
    let mut s = String::from("hello world");
    let r1 = &mut s;
    // 不允许多次 mutable引用
    // let r2  = &mut s;
    println!("cur r1 {} ", r1);
}

fn no_slience_first_word(s: &String) -> &str {
    // 字符转转换成byte 数组
    let bytes = s.as_bytes();

    // 迭代枚举
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn slience_first_word(s: &str) -> &str {
    // 字符转转换成byte 数组
    let bytes = s.as_bytes();

    // 迭代枚举
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
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
