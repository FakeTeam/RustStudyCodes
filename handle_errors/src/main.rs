fn main() {
    println!("Hello, world!");
    // use_panic();
    // out_of_range();
    // bad_open();
    println!("{:?}", error_transfer());
    error_transfer_with_operator();
    error_transfer_with_operator_mini();
    bad_open_file_expect();
    match_different_errors();
    catch_open_file_with_result();
    bad_open_match();
}

#[allow(unused)]
fn use_panic() {
    panic!("crash and burn");
}

#[allow(unused)]
fn out_of_range() {
    let v = vec![1, 2, 3];
    v[99];
}

#[allow(unused)]
fn bad_open() {
    use std::fs::File;
    let f = File::open("hello.txt").expect("bad open.");
}

#[allow(unused)]
fn bad_open_match() {
    use std::fs::File;
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file : {:?}", error);
        }
    };
}

#[allow(unused)]
fn match_different_errors() {
    use std::fs::File;
    use std::io::ErrorKind;
    let f = File::open("hello.txt");
    let f = match f {
        // 这里作为返回值
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                // 这里作为返回值
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_err => {
                panic!("Problem opening the file : {:?}", other_err)
            }
        },
    };
}

fn catch_open_file_with_result() {
    use std::fs::File;
    use std::io::ErrorKind;

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file : {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("{:?}", f);
}

#[allow(unused)]
fn bad_open_file_expect() {
    use std::fs::File;
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

#[allow(unused)]
fn error_transfer() {
    use std::fs::File;
    use std::io::{self, Read};
    use String;
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
}

#[allow(unused)]
fn error_transfer_with_operator() {
    use std::fs::File;
    use std::io;
    use std::io::Read;
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
}

#[allow(unused)]
fn error_transfer_with_operator_easy() {
    use std::fs::File;
    use std::io;
    use std::io::Read;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
}

#[allow(unused)]
fn error_transfer_with_operator_mini() {
    use std::fs;
    use std::io;
    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
}

#[allow(unused)]
fn error_transfer_with_option() {
    use std::error::Error;
    use std::fs::File;
    fn read_username_from_file() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt");
        Ok(())
    }
}
