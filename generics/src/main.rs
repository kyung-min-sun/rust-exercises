struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> Option<&T> {
    let mut largestEl = match list.get(0) {
        Some(value) => value,
        None => return None,
    };
    for el in list {
        if largestEl < el {
            largestEl = el;
        }
    }

    Some(largestEl)
}
fn main() {
   let intList: Vec<i32> = Vec::new(); 
   largest(&intList);
   let p = Point {
    x: 5,
    y: 10
   };
   let x = p.x();
    println!("p.x = {x}");
}
