use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new();
    let v2 = vec![0];
    let mut k = match v.get(10) {
        Some(value) => value,
        None => &0,
    };
    k = &10;
    println!("{k}");

    let mut map = HashMap::new();
    map.insert(String::from("hello"), String::from("world"));
    map.get("hello");
}
