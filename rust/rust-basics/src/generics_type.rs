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

pub fn generics_max() {
    let numbers = vec![34, 50, 25, 100, 65];
    let max_value = max(&numbers);
    println!("max value: {}", max_value);

    let chars = vec!['y', 'm', 'a', 'q', 'u'];
    let max_char = max(&chars);
    println!("max char: {}", max_char);
}

// 泛型: fn 方法名<类型声明>(参数可使用声明的类型) -> 返回值可使用声明的类型, 
// 类型声明中必须给出类型约束, 即必须实现哪些 trait
// 下面的方法只要类型实现了std::ops::Add trait，那么就可以实现累加, 底层是调用 i.add(j)
fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

// T 声明类型必须同时实现 比较和赋值 trait
fn max<T: std::cmp::PartialOrd + Copy>(l: &[T]) -> T {
    let mut v = l[0];
    for &item in l {
        if item > v {
            v = item;
        }
    }

    return v;
}


// 结构体泛型
#[derive(Debug)]
#[allow(dead_code)]
struct Point<T> {
    x: T, 
    y: T
}

#[derive(Debug)]
#[allow(dead_code)]
struct PointTU<T, U> {
    x: T,
    y: U
}

pub fn struct_generics() {
    // 在同一个结构体实例中  T 必须保持一致
    let p = Point{x: 5, y: 6};
    println!("{:?}", p);
    let p1 = Point{x: 1.2, y: 2.3};
    println!("{:?}", p1);

    // 此时 T 和 U 既可以相同也可以不同
    let pt = PointTU{x: 3, y: 8.2};
    println!("{:?}", pt);
}


// 错误处理常用的枚举写法
#[derive(Debug)]
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
