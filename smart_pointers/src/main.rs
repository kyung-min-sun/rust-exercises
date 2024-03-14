enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let message = Message::Quit;
    let hey = match message {
        Message::Quit => todo!(),
        Message::Move { x, y } => x == 32,
        Message::Write(_) => todo!(),
        Message::ChangeColor(_, _, _) => todo!(),
    };
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
}
