use std::fmt::Display;
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {} ...)", self.summarize_author())
    }
}
pub trait CurDisplay {
    fn summarize(&self) -> String {
        format!("(Display default ...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl CurDisplay for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 特征作为参数
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// 特征绑定语法
pub fn notify_bind<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

// 多个特征
pub fn notify_two(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize())
}

// 多个特征绑定
pub fn notify_two_bin<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize())
}
// 使用+号指定多个特征边界
#[allow(unused_variables)]
pub fn notify_two_brige_add(item: &(impl Summary + Display)) {
    println!("......")
}

// 使用模版指定多个特征边界
#[allow(unused_variables)]
pub fn notify_two_brige_templete<T: Summary + Display>(item: &T) {
    println!("......")
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn some_function<T: Display + Clone, U: Clone + Summary>(t: &T, u: &U) -> i32 {
    2
}

// 使用where进行清晰的绑定
#[allow(unused)]
fn some_function_where<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Summary,
{
    2
}

// 返回实现特征类型
pub fn returns_summarizabel() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

#[allow(unused)]
struct Pair<T> {
    x: T,
    y: T,
}

#[allow(unused)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[allow(unused)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
