
// 动态大小类型 会报错
// enum List {
//     Node(i32, List),
//     Nil,
// }

// 使用 Box 包装即可表示固定大小的类型，实现循环引用
enum List {
    Node(i32, Box<List>),
    Nil,
}

// 如果将多个不同的对象实例放到同一个数组中, 可以通过同一个 trait + Box<dyn > 来实现

trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32
}

struct Select {
    id: u32
}

// Button 和 Select 结构体都实现 Draw trait
impl Draw for Button {
    fn draw(&self) {
        println!("button: {}", self.id);
    }
}

impl Draw for Select {
    fn draw(&self) {
        println!("select: {}", self.id);
    }
}

pub fn box_example() {
    // a 存储在栈上
    let a = 3;
    // 使用 Box 将数字存储到堆上 当超过生命周期后会自动 Drop 掉
    // 因为 Box<T> 实现了 Drop trait
    let a = Box::new(a);
    println!("a = {}", a);
    // 但是无法直接运算 不能自动 Deref 解引用
    // let b = a + 1;

    // 栈上创建长度为 1000 的数组
    let arr = [0; 1000];
    // 并未发生所有权转移 而是实际拷贝了一份数据
    let arr1 = arr;
    println!("arr len: {}, arr1 len: {}", arr.len(), arr1.len());

    // 使用智能指针创建堆上数组
    let arr = Box::new([0; 1000]);
    // 所有权直接转移
    let arr1 = arr;
    // arr 不在拥有所有权，因此无法再使用了
    // println!("arr len: {}", arr.len());
    println!("arr1 len: {}", arr1.len());

    let l = List::Node(32, Box::new(List::Node(64, Box::new(List::Nil))));
    list_iter(l);


    // 包装不同类型到同一个 Vec 中
    let elements: Vec<Box<dyn Draw>> = vec![Box::new(Button{id: 1}), Box::new(Select{id: 2})];
    for e in elements {
        // 调用方法时可自动解引用
        e.draw();
    }

    let arr = vec![Box::new(1), Box::new(2)];
    // 需要用只读引用获取 否则会因为没有实现 Copy 而报所有权错误
    let (first, second) = (&arr[0], &arr[1]);
    // 取值需要用 ** 第一次是解 Box 引用，第二次是取 Box 值
    let sum_val = **first + **second;
    // 打印时会自动解出来值
    println!("{} + {} = {}", first, second, sum_val);
    
}

fn list_iter(l: List) {
    match l {
        List::Node(i, v) => {
            println!("i: {}", i);
            list_iter(*v);
        },
        List::Nil => {
            println!("List Nil.")
        }
    }
}
