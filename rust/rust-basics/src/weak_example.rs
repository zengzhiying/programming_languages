use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, next) => Some(next),
            List::Nil => None,
        }
    }
}

pub fn weak_example() {
    // 循环引用导致内存泄漏的例子
    let a = List::Cons(5, RefCell::new(Rc::new(List::Nil)));
    // 将 a 用引用计数包装下
    let a = Rc::new(a);
    // a 的引用计数为 1
    println!("a->{:?}, a rc: {}", a.tail(), Rc::strong_count(&a));
    // b -> a
    let b = List::Cons(10, RefCell::new(Rc::clone(&a)));
    let b = Rc::new(b);
    // b 的引用计数为 1，a 的引用计数为 2
    println!("b->{:?}, b rc: {}, a rc: {}", b.tail(), Rc::strong_count(&b), Rc::strong_count(&a));

    // 利用 RefCell 实现: a -> b
    let v = a.tail();
    if let Some(n) = v {
        *n.borrow_mut() = Rc::clone(&b);
    }

    // 引用计数都为 2
    println!("a rc: {}, b rc: {}", Rc::strong_count(&a), Rc::strong_count(&b));

    // 循环引用会导致 stack overflow
    // println!("a->{:?}", a.tail());
    // println!("b->{:?}", b.tail());
}