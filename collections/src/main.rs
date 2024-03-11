fn main() {
    let v: Vec<i32> = Vec::new();
    let mut k = match v.get(10) {
        Some(value) => value,
        None => &0,
    };
    k = &10;
    println!("{k}")
}
