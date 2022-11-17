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

// 只要类型实现了std::ops::Add trait，那么就可以实现累加
// 底层是调用i.add(j)
fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}
