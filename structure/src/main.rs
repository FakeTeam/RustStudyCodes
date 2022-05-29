struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 添加方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[allow(unused)]
fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        email: String::from("admin1@example.com"),
        username: String::from("admin"),
        active: true,
        sign_in_count: 1,
    };
    user1.sign_in_count = 2;
    // 使用字段初始化简写
    let mut user2 = build_user(String::from("admin@example.com"), String::from("admin"));
    user2.active = false;
    // 使用结构更新语法从其他实例创建实例
    let mut user3 = User {
        email: String::from("admin3@example.com"),
        ..user2
    };
    user3.username = String::from("admin1");
    println!("{}", user3.email);

    // 使用没有命名字段的元组结构来创建不同的类型
    let black = Color(1, 2, 2);
    let origin = Point(1, 2, 2);
    // 没有任何字段的类单元结构
    let subject = AlwaysEqual;
    let rect1 = (1, 2);
    println!("area = {}", area(rect1));
    let scale = 2;
    let rect2 = Rectangle {
        height: dbg!(30 * scale),
        width: 5,
    };
    let rect3 = Rectangle {
        height: 1,
        width: 2,
    };
    dbg!(&rect2);
    println!("area method {}", rect2.area());
    let area = area_struct(&rect2);
    println!("area {} ", area);
    println!("can hold {}", rect2.can_hold(&rect3));
}

// 用元组重构
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
