use crate::garden::vegetables::Asparus;
pub mod garden;

fn main() {
    println!("Hello, world!");
    let plant = Asparus { public: 3 };
    println!("I'm growing {:?}", plant);
    crate::garden::vegetables::showme();
}
