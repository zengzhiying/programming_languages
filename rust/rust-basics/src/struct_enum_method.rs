



#[derive(Debug)]
enum Message {
    Open,
    Write(String),
    Quit(u32)
}

impl Message {
    fn get_message_length(&self) -> usize {
        // if let 对于不可变引用借用, 也只能使用其中变量的引用, 
        // 而不可能转移其内部的所有权使原来对象改变
        if let Message::Write(s) = self {
            return s.len();
        }
        0
    }

    fn get_quit_num(&self) -> u32 {
        if let Message::Quit(n) = self {
            return *n;
        }
        0
    }
}

pub fn enum_method() {
    let msg1 = Message::Open;
    let msg2 = Message::Write(String::from("yes"));
    let msg3 = Message::Quit(7);
    println!(
        "{}, {}, {}", 
        msg1.get_message_length(), 
        msg2.get_message_length(), 
        msg3.get_quit_num());
    println!("{:?}, {:?}, {:?}", msg1, msg2, msg3);
}

