struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.height >= rect.height && self.width >= rect.width
    }
}

fn build_user(email: &str, username: &str) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count: 60,
    }
}

fn main() {
    let mut user = User {
        active: true,
        username: String::from("hello"),
        email: String::from("hello@hello.com"),
        sign_in_count: 60
    };
    let mut user3 = user;

    build_user(&user3.email, &user3.username);

    let mut user2 = User {
        email: String::from("email.com"),
        ..user3
    };

    let black = Color(0, 0, 0);
    let Color(r, g, b) = black;
    let red = black.0;

    let rect1 = Rectangle {height: 10, width: 10};
    let rect2 = Rectangle {height: 50, width: 30};
    let mut rect3 = rect1;
    let rect4 = Rectangle::square(40);

    rect3.height = 100;
    rect3.area();

    println!("{}", rect2.can_hold(&rect3));
    println!("{:#?}", rect3);

    dbg!(&rect3);
    dbg!(&rect4);
}
