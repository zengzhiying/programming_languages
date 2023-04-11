use std::ops::Deref;

// 定义智能指针
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// 定义后无法自动解引用 还必须实现 Deref trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn deref_example() {
    // 常规解引用方式
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // 数字不能直接和引用比较
    // assert_eq!(5, y);
    assert_eq!(5, *y);

    // 智能指针解引用
    let x = Box::new(1);
    let y = *x + 1;
    assert_eq!(2, y);

    // 自定义智能指针解引用
    let x = MyBox::new(10);
    // *x 等价于 *(x.deref())  Rust 编译器自动帮我们做了
    assert_eq!(10, *x);

    // 隐式 deref 转换
    let s = String::from("auto deref");
    // &String -> &str
    // &s 为 &String 类型，而 String 实现了 Deref trait 可以自动转换为 &str 类型
    // 必须使用 &s 的引用类型实参的方式来触发 Deref
    display(&s);
    // 隐式转换可以连续进行
    let s = MyBox::new(String::from("auto deref"));
    // &MyBox 自动 deref 为 &String, 然后 &String 自动 deref 为 &str，该过程由编译器完成
    display(&s);
    // 手动写法
    display(&(*s)[..]);

    // 赋值必须手动解引用 不会自动 deref
    let s1 = &s;
    display(s1);
    // 方法调用会自动 deref，由于是 to_string 引用方式调用因此所有权不转移，且返回的字符串是拷贝出来一份新的
    // MyBox -> &String (to_string)，Box<T>, Rc<T> 方法调用时会自动解引用为 &T
    let s2 = s.to_string();
    display(&s2);

    // v: T, 如果 T: Deref<Target=U> ==> &foo deref &U
    // 比如: v: MyBox<T>, MyBox<T>: Deref<Target=T> ==> &v deref &T
    // 智能指针和多重引用都会自动转换为单个引用，例如下面的 &&&s2 会自动转换为 &s2
    display(&&&s2);

    // 整体规则：
    // T: Deref<Target=U>, &T deref &U 或者 &mut T deref &U
    // T: DerefMut<Target=U>, &mut T deref &mut U
    // 可变既可以到可变也可以到不可变，不可变只能转换为不可变，这样可以保证引用的安全性

    // 通常只建议为自定义的智能指针实现 Deref trait，其他类型不建议实现

}

fn display(s: &str) {
    println!("{}", s);
}