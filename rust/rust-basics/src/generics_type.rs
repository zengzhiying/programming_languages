use std::ops::Add;
use std::time::Duration;

pub fn generics_add() {
    let f = add(3.2, 2.8);
    let i = add(10, 20);
    let d = add(
        Duration::new(10, 0), 
        Duration::new(20, 0)
    );

    println!("{}, {}, {:?}", f, i, d);
}

// 定义泛型函数同时获取 i32 和 char 类型 Vector 的最大值
pub fn generics_max() {
    let numbers = vec![34, 50, 25, 100, 65];
    let max_value = max(&numbers);
    let max_value2 = max2(&numbers);
    println!("max value: {}, {}", max_value, max_value2);

    let chars = vec!['y', 'm', 'a', 'q', 'u'];
    let max_char = max(&chars);
    let max_char2 = max2(&chars);
    println!("max char: {}, {}", max_char, max_char2);
}

// 下面的方法只要类型实现了std::ops::Add trait，那么就可以实现累加, 底层是调用 i.add(j)
fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

// 泛型: fn 方法名<类型声明>(参数可使用声明的类型) -> 返回值可使用声明的类型, 
// 这里 <> 中采用 T 声明类型，类型声明中必须给出类型约束, 即必须实现哪些 trait
// 计算最大值必须同时实现比较和赋值的 trait
fn max<T: std::cmp::PartialOrd + Copy>(l: &[T]) -> T {
    let mut v = l[0];
    for &item in l {
        if item > v {
            v = item;
        }
    }

    return v;
}

// 如果使用引用类型，则不需要实现 Copy trait
fn max2<T: std::cmp::PartialOrd>(l: &[T]) -> &T {
    let mut v = &l[0];
    for item in l {
        if item > v {
            v = item;
        }
    }

    return v;
}


// ================== 结构体泛型 ===================
#[derive(Debug)]
#[allow(dead_code)]
// 结构体同样使用 <T> 声明类型 T，并且在同一个结构体实例中，T 必须保持一致
struct Point<T> {
    x: T, 
    y: T
}

// 方法中使用泛型
// impl<T> 提前声明，这样 Rust 就知道结构体中的类型 T 属于泛型而不是某一个具体类型，
// Point<T> 是直接使用结构体的定义, 整体可以看作一种类型
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 可以专门声明只为 Point<f32> 类型实现方法
// 这意味着 Point<f32> 会有方法 distance，而其他的类型的 Point<T> 则没有该方法
impl Point<f32>  {
    fn distance(&self) -> f32 {
        let d = self.x.powi(2) + self.y.powi(2);
        d.sqrt()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
// 定义不同的类型可以在 <> 中声明多个类型
// 此时 T 和 U 既可以相同也可以不同
struct PointTU<T, U> {
    x: T,
    y: U
}

impl<T, U> PointTU<T, U> {
    // 方法中使用的泛型类型可以和外层声明的类型不一样, 也就是可以单独声明类型
    // 这样可以使用声明的类型创建一个新的实例，即 PointTU<V, W>，这个只想对于方法本身使用
    // self 必须符合外层 impl 中的类型声明
    fn mixup<V, W>(self, other: PointTU<V, W>) -> PointTU<T, W> {
        PointTU { x: self.x, y: other.y }
    }
}

pub fn struct_generics() {
    // 在同一个结构体实例中  T 必须保持一致
    let p = Point{x: 5, y: 6};
    println!("{:?}", p);
    let p1 = Point{x: 1.2, y: 2.3};
    println!("{:?}, x: {}", p1, p1.x());
    let p2: Point<f32> = Point { x: 1.1, y: 2.2 };
    println!("{:?}, distance: {}", p2, p2.distance());

    // 此时 T 和 U 既可以相同也可以不同
    let pt = PointTU{x: 3, y: 8.2};
    println!("{:?}", pt);
    let pt1 = pt.mixup(PointTU{x: 1.2, y: 9.8});
    println!("{:?}", pt1);
}


// ================= 枚举泛型: 错误处理常用的枚举写法 ===================
#[derive(Debug)]
// 类似于标准库中的 Option 与 Result
// 假如我们代码中定义了多个结构体和枚举，但是其实只有类型不相同的时候，就可以尝试使用枚举消除重复代码
enum ResultNumber<T, E> {
    Ok(T),
    Err(E)
}

pub fn enum_generics() {
    let mut r1: ResultNumber<i32, &str> = ResultNumber::Ok(68);
    let mut r2: ResultNumber<i32, &str> = ResultNumber::Err("get result error!");
    match_result(& mut r1);
    match_result(& mut r2);
    println!("{:?}", r1);
}

fn match_result(r: & mut ResultNumber<i32, &str>) {
    match r {
        ResultNumber::Ok(v) => {
            println!("value: {}", v);
            *v += 1;
        },
        ResultNumber::Err(err) => {
            println!("error: {}", err);
        }
    }
}

// ================== const 泛型 =====================

// 数组长度必须是常量, 因此通过 const 泛型可以处理长度问题
// 这样就不需要必须传入切片了
fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("array: {:?}, size: {}", arr, N);
}

pub fn const_generics() {
    let arr1 = [1, 2, 3];
    print_array(arr1);
    let arr2 = [1.1, 2.1];
    print_array(arr2);
}
