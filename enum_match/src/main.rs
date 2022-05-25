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


struct IpAddr4{

}
struct IpAddr6{

}
// 使用结构体作为枚举类型
enum IpaddrStructEnum{
    V4(IpAddr4),
    V6(IpAddr6),
}


fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
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
}

fn route(_ip_kind: IpAddrKind) {}
