

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
}
