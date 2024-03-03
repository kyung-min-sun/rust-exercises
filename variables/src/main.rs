fn main() {
    fibonacci_number(9);
    another_function(32, "unit")
}

fn fibonacci_number(n: i32) -> i32 {
    let mut fib_0 = 0;
    let mut fib_1 = 1;
    if n == 0 {
        return fib_0;
    }
    for _ in 1..n {
        println!("{fib_1}");
        let fib_2 = fib_0 + fib_1;
        fib_0 = fib_1;
        fib_1 = fib_2;
    }
    fib_1
}

fn another_function(x: i32, unit: &str) {
    println!("value is {x} {unit}");

    let y = {
        let x = 3;
        x + 1
    };

    println!("{y}");

    let number = 6;

    match number {
        i32::MIN..=0 => println!("test"),
        1 => println!("hi"),
        2..=i32::MAX => println!("test"),
    }

    let number = if true { 5 } else { 6 };
    println!("{number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    const A: [i32; 4] = [10, 20, 30, 40];
    for index in 1..A.len() {
        println!("{index}")
    }

    println!("{result}");
}
