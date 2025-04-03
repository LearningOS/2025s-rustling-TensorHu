// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum Message {
    // 定义 Move 变体，包含 x 和 y 字段
    Move { x: i32, y: i32 },
    // 定义 Echo 变体，包含一个字符串字段
    Echo(String),
    // 定义 ChangeColor 变体，包含三个整数字段
    ChangeColor(u8, u8, u8),
    // 定义 Quit 变体，它是一个无数据的单元变体
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
