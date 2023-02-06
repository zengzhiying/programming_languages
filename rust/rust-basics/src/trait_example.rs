
// 定义 trait(特征)
trait Summary {
    // fn post(&self) -> String;
    // 也可以定义默认实现
    fn post(&self) -> String {
        String::from("Default post.")
    }
}

#[derive(Debug)]
struct Weibo {
    user: String,
    content: String
}

// 实现特征方法
impl Summary for Weibo {
    fn post(&self) -> String {
        format!("微博用户: {}, 内容: {}", self.user, self.content)
    }
}

#[derive(Debug)]
struct Weixin {
    src: String,
    dest: String,
    message: String
}

impl Summary for Weixin {
    fn post(&self) -> String {
        format!("微信用户 {} 发送至 {}, 内容: {}", self.src, self.dest, self.message)
    }
}

#[derive(Debug)]
struct Twitter {}

// 默认实现方式
impl Summary for Twitter {}

pub fn example() {
    let weibo = Weibo{user: String::from("u1800"), content: String::from("昨天烟花不错.")};
    let weixin = Weixin{src: String::from("wxid01"), dest: String::from("wxid02"), message: String::from("我是马化腾")};
    let twitter = Twitter{};
    println!("{:?}, {}", weibo, weibo.post());
    println!("{:?}, {}", weixin, weixin.post());
    println!("{:?}, {}", twitter, twitter.post());
}
