#[derive(Debug)]
pub struct Asparus {
    pub public: i32,
}

#[allow(unused)]
#[derive(Debug)]
struct PrivateStruct {
    private_data: u8,
    pub public_data: u32,
}

#[derive(Debug)]
pub struct PublicStruct {}

pub fn showme() {
    println!("show me more thing.")
}
