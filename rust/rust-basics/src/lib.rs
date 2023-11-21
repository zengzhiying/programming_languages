mod structure_type;

pub struct Guess {
    #[allow(dead_code)]
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    // #[test] 表示这个函数是一个测试函数
    // 也可以有非测试函数
    #[test]
    fn it_works() {
        let result = 2 + 2;
        println!("result: {}", result);
        assert_eq!(result, 4);
    }

    // 引入父模块中的公共结构
    use super::structure_type::Rectangle;

    #[test]
    // 想要在全部测试中排除指定的测试可以添加下面的注解
    #[ignore]
    fn rectangle_contain() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);
        // assert! 断言是否为 true，如果是 false 则测试会 panic
        // assert!(larger.contain(&smaller));
        // 后面还支持添加 format! 参数，在断言失败时打印更多详细的信息
        assert!(larger.contain(&smaller), "Rectangle `{:?}` not contain `{:?}`", larger, smaller);
    }

    use super::Guess;

    // 有时候我们想测试代码中 panic 部分是否会被执行到，这时候就需要添加 #[should_panic] 来测试 panic 了
    #[test]
    // #[should_panic]
    // 指定 panic 中必须存在字串，否则会认为测试不通过
    #[should_panic(expected = "must be between 1 and 100")]
    fn greater_than_100() {
        // 200 不符合范围，肯定会 panic，因此单元测试可以正常通过
        Guess::new(200);
    }


    #[test]
    // 还可以对测试函数添加返回值 Result 这样就可以在调用时很方便地使用 ? 操作符直接返回内容
    // 如果要断言 Err 可以使用 assert!(value.is_err())
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

}
