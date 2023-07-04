use std::{cell::{RefCell, Cell}, thread::{LocalKey, self}};
use thread_local::ThreadLocal;
use std::sync::Arc;

// 线程局部变量也可以放在结构体中
struct LocalVar;
impl LocalVar {
    thread_local! {
        static VALUE: RefCell<u32> = RefCell::new(1);
    }
}

// 其他结构体中也可以引用
struct LocalVarRef {
    value: &'static LocalKey<RefCell<u32>>
}
impl LocalVarRef {
    fn new() -> Self {
        Self { value: &LocalVar::VALUE }
    }
}

pub fn threading_local_variable() {
    // 初始化线程局部变量 生命周期为 'static
    thread_local! {static VALUE: RefCell<u32> = RefCell::new(1)};
    // 通过 with 来获取变量值或进行赋值
    VALUE.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    // 每个线程开始时都会拿到设置的初始值 1
    // 多个线程的设置都是局部生效，相互之间不影响
    // 上面的 with 相当于主线程使用局部变量
    let t = thread::spawn(|| {
        VALUE.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });

    t.join().unwrap();

    // 外部再次使用时，拿到的仍然是刚才设置的局部值 2
    // 多个线程的局部变量结果是无法汇总的
    VALUE.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });

    // 也可以使用结构体实现的线程局部变量
    LocalVar::VALUE.with(|f| {
        println!("{:?}", f);
        *f.borrow_mut() += 1;
    });

    let r = LocalVarRef::new();
    r.value.with(|x| {
        // 因为都是在 1 个线程中因此值为 2
        println!("{:?}", x);
    });

    println!("YES");
}

// 第三方 thread_local 库，允许收集多线程累加结果
// https://github.com/Amanieu/thread_local-rs
pub fn thread_local_three_party_example() {
    let reduce = Arc::new(ThreadLocal::new());

    let mut threads = Vec::with_capacity(5);

    for i in 0..5 {
        let reduce_copy = reduce.clone();
        threads.push(thread::spawn(move || {
            let cell = reduce_copy.get_or(|| Cell::new(0));
            cell.set(cell.get() + 1);
            println!("thread {} value: {}", i, cell.get());
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    // 收集 reduce 的值然后求和
    let reduce = Arc::try_unwrap(reduce).unwrap();
    // 不能直接获取
    // let total = reduce.get().unwrap();
    let total = reduce.into_iter().fold(0, |x, y| x + y.get());
    // 这样获取的是 5 
    println!("total: {}", total);
}
