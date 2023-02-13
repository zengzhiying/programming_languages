
trait Draw {
    fn draw(&self);
}

struct Button {
    width: u32,
    height: u32,
    label: String
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button: {}, {}, {}", self.width, self.height, self.label);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox: {}, {}, {:?}", self.width, self.height, self.options);
    }
}

struct Screen {
    // 特征参数写法 Box<dyn Draw> 
    // Box 是存在堆上的对象, 类似于智能指针
    components: Vec<Box<dyn Draw>>
}

// 使用泛型约束 写法 但是无法传不同的参数
// struct Screen<T: Draw> {
//     components: Vec<T>
// }

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 函数参数写法 可以传递实现该特征的所有对象
// dyn 关键字仅用于声明, 创建时无需使用
// dyn 表示动态分发，和泛型的静态分发相对应，也就是在运行时确定需要调用什么方法
fn draw1(x: Box<dyn Draw>) {
    // 自动解引用
    x.draw();
}

// 写法 2
fn draw2(x: &dyn Draw) {
    x.draw();
}

pub fn trait_object() {
    let button = Button{width: 30, height: 20, label: String::from("submit")};
    let select_box = SelectBox{
        width: 50, height: 10, 
        options: vec![String::from("A"), String::from("B"), String::from("C")]
    };

    draw1(Box::new(button));
    draw2(&select_box);

    let button = Button{width: 30, height: 20, label: String::from("submit")};

    // 要用 Box::new 传递进去
    let screen = Screen{components: vec![Box::new(button), Box::new(select_box)]};
    screen.run();
}

