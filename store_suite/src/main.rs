fn main() {
    println!("Hello, world!");
    basic_vector();
    basic_vector_define();
    how_to_read_vector_elem();
    rust_check_vector_outof_range();
    vector_change_borrow();
    for_range_vector();
    for_range_vector_mutable();
    vector_store_more_enum();
    use_hash_map();
    convert_vec_to_hashmap();
}

fn basic_vector() {
    println!("basic_vector");
    let mut v: Vec<u32> = Vec::new();
    v.push(23);
    v.push(23);
    for elem in v.iter() {
        print!("{} ", elem);
    }
    println!("");
}

fn basic_vector_define() {
    println!("basic_vector_define");
    let v = vec![1, 2, 3, 4, 5];
    for elem in v.iter() {
        print!("{} ", elem);
    }
    println!("");
}

fn how_to_read_vector_elem() {
    let v = vec![1, 2, 3, 4, 5, 6, 7];
    let thrid: &i32 = &v[2];
    println!("The third element is {}", thrid);
    match v.get(2) {
        Some(thrid) => {
            println!("The third element is {}", thrid);
        }
        None => {
            println!("The third element is empty.");
        }
    }
}

fn rust_check_vector_outof_range() {
    let v = vec![1, 2, 3, 3, 3, 4];
    // rust会在编译时检查是否存在此索引，不存在时会告警
    // let item_is_not_exist: &i32 = &v[100];
    let item_is_not_exist = v.get(100);
    println!("{:?}", item_is_not_exist)
}

#[allow(unused)]
fn vector_change_borrow() {
    let mut v = vec![1, 2, 3, 3, 4];
    let first = &v[0];
    // 由于后面有借用vector,此时还没有归还借用，导致无法进行元素填充
    v.push(2333);
    // println!("The first element is : {}", first);
}

fn for_range_vector() {
    let v = vec![1, 2, 3, 3, 3];
    for i in v {
        print!("{} ", i)
    }
}

fn for_range_vector_mutable() {
    let mut v = vec![1, 2, 3, 3, 3, 1, 4];
    for i in &mut v {
        *i += 20;
    }
}

fn vector_store_more_enum() {
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }
    let low = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(2.3),
        SpreadSheetCell::Text(String::from("hello world")),
    ];
    for elem in low {
        print!("{:?} ", elem);
    }
}

fn use_hash_map() {
    use std::collections::HashMap;
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    // 这里使用field_name已经move啦
    map.insert(field_name, field_value);

    // let result = map.get(&field_name);
    let result = map.get(&String::from("Favorite color"));
    println!("{:?}", result)
}

fn convert_vec_to_hashmap() {
    use std::collections::HashMap;
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    scores.insert(String::from("Red"), 30);
    // 存在则不更新值，不存在则更新
    scores.entry(String::from("Blue")).or_insert(1000);
    println!("{:?}", scores);

    for (team, score) in scores {
        println!("{}:{}", team, score);
    }
}
