fn main() {
    let mut s1 = String::from("hello");
    takes_ownership(&mut s1);
    println!("{s1}");
}

fn takes_ownership(some_string: &mut String) {
    some_string.push_str(", world!");
    println!("took ownership of {some_string}");
}
