#![allow(unused)]
fn main() {
    use String;
    enum IpAddrKind {
        V4,
        V6,
    }
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    // 指定枚举类型
    enum IpAddrEnum {
        V4(String),
        V6(String),
    }
    // 指定枚举为不同的类型
    enum IpAddrDiffEnum {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    struct IpAddr4 {}
    struct IpAddr6 {}
    // 使用结构体作为枚举类型
    enum IpaddrStructEnum {
        V4(IpAddr4),
        V6(IpAddr6),
    }
    // 嵌入不同类型
    enum Message {
        // 根本没有与之对应的数据
        Quit,
        // 像结构体一样命名字段
        Move { x: i32, y: i32 },
        // 包括一个String
        Write(String),
        // 包括一个三个i32值的元组
        ChangeColor(i32, i32, i32),
    }
    struct QuitMessage;
    struct MOveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String);

    struct ChangeColorMessage(i32, i32, i32);

    impl Message {
        fn call(&self) {
            println!("call me !");
        }
    }

    enum Options<T> {
        None,
        Some(T),
    }

    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    fn route(_ip_kind: IpAddrKind) {}
    route(four);
    route(six);
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let lookback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let lookback = IpAddrEnum::V6(String::from("::1"));

    let home = IpAddrDiffEnum::V4(127, 0, 0, 1);
    let lookback = IpAddrDiffEnum::V6(String::from("::1"));

    let m = Message::Write(String::from("Show me your monster."));
    m.call();

    let some_number: Option<u8> = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    match_example_without_option();
    match_example_with_option();
    match_example_with_default();
    match_example_with_placeholder();
}

fn match_example_without_option() {
    #[derive(Debug)]
    enum UsState {
        Alabma,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
    value_in_cents(Coin::Quarter(UsState::Alabma));
}

fn match_example_with_option() {
    fn push_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => {
                println!("option match {:?}", x);
                None
            }
            Some(i) => {
                println!("option match {:?}", x);
                Some(i + 1)
            }
        }
    }
    let five = Some(5);
    let six = push_one(five);
    let none = push_one(None);
}

fn match_example_with_default() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    fn add_fancy_hat() {
        println!("add_fancy_hat");
    }
    fn remove_fancy_hat() {
        println!("remove_fancy_hat");
    }
    fn move_player(num_spaces: u8) {
        println!("move_player {}", num_spaces);
    }
}

fn match_example_with_placeholder() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    fn add_fancy_hat() {
        println!("add_fancy_hat");
    }
    fn remove_fancy_hat() {
        println!("remove_fancy_hat");
    }
    fn reroll() {
        println!("reroll");
    }
}
