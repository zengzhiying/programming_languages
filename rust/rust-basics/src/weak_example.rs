use std::{cell::RefCell, rc::{Rc, Weak}};

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

// weak 例子 团队与个人
// 团队
#[derive(Debug)]
struct Team {
    name: String,
    // 成员 使用 Weak
    members: RefCell<Vec<Weak<Member>>>,
}

// 成员
#[derive(Debug)]
struct Member {
    name: String,
    // 所属团队
    team: Rc<Team>,
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

    // 创建团队
    let dev = Rc::new(Team{
        name: "研发团队".to_string(),
        members: RefCell::new(Vec::new()),
    });
    // 创建成员并关联团队
    let java = Rc::new(Member{
        name: "Java".to_string(),
        team: Rc::clone(&dev),
    });
    let python = Rc::new(Member{
        name: "Python".to_string(),
        team: Rc::clone(&dev),
    });

    // 团队关联成员
    // Weak 只用来访问数据，没有所有权，不增加引用计数，因此不会影响所指向内容的回收
    // Rc<T> 通过 downgrade 方法转换为 Weak<T>
    // Weak<T> 通过 upgrade 方法转换为 Option<Rc<T>>，因为不确定值是否被回收所以使用 Option
    // 正是因为这样，Weak<T> 不会干扰 Rc<T> 的释放，因此不会出现循环引用的情况
    let mut ms = dev.members.borrow_mut();
    ms.push(Rc::downgrade(&python));
    ms.push(Rc::downgrade(&java));

    // 引用计数为 3
    println!("dev rc num: {}", Rc::strong_count(&dev));
    // 可变引用和不可变引用不能同时存在
    drop(ms);

    for m in dev.members.borrow().iter() {
        println!("member: {:?}, {}", m, Weak::strong_count(&m));
        let member = m.upgrade();
        if let Some(mem) = member {
            println!("member name: {}, team: {:?}", mem.name, mem.team.name);
        } else {
            println!("None");
        }
    }

    // 直接删除成员
    drop(java);
    drop(python);

    for m in dev.members.borrow().iter() {
        println!("member: {:?}, {}", m, Weak::strong_count(&m));
        let member = m.upgrade();
        if let Some(mem) = member {
            println!("member name: {}, team: {:?}", mem.name, mem.team.name);
        } else {
            println!("None");
        }
    }
}