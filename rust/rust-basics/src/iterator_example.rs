
#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // shoes.into_iter() 会获取 shoes 的所有权，调用后将不可再使用
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

pub fn iterator_example() {
    let v1 = vec![1, 2, 3];
    // 创建 v1 的一个迭代器，不可变引用、不获取所有权
    // 迭代器是懒惰（lazy）的，在使用它之前都不会有任何实质性的效果
    let v1_iter = v1.iter();
    // 使用迭代器无需用下标遍历，这可以消除很多潜在的问题
    for v in v1_iter {
        println!("Got: {}", v);
    }

    // 可以为任何自定义的结构实现迭代器，而不仅是 vector 这样的的线性结构，只要实现 Iterator 这样的 trait 即可
    // 具体的 trait 定义如下，主要就是实现其 next 方法，其中的 Item 属于关联类型（associated type），
    // Item 类型就是迭代器返回的元素类型，并被包装到 Some 中，当迭代器结束时会返回 None
    // pub trait Iterator {
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item>;
    // }

    // 例如我们可以对 v1 的迭代器直接调用 next 方法
    // 注意 v1_iter 迭代器本身必须是可变的，因为调用 next 方法相当于消费数据，所以需要修改迭代器中关于数据的偏移状态
    // 但是这不会对 v1 造成任何影响，因为 v1.iter() 是获取的不可变引用迭代器，只是迭代器本身是可变的，所以后面获取的是每个元素的地址
    // 但是使用 for 迭代 v1_iter 不需要设置迭代器为可变，因为这些操作都是隐式在后台执行的，相当于执行时 v1_iter 在后台是可变的
    // 如果要得到可变引用的迭代器可以使用 iter_mut()，如果要得到能返回元素所有权的迭代器则可以使用 into_iter()
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // 类似于上面调用 next 方法来消费迭代器，常见的还有一些其他方法可以消费，
    // 比如 sum 可以自动调用 next 遍历迭代器并对元素累加获得结果
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    // 不允许再使用迭代器，因为 sum 会获取其所有权
    // println!("{:?}", v1_iter);

    // 可以在迭代器上调用 map 等函数式处理方法，并使用闭包来处理元素
    // 但是调用 map 后表达式依然是惰性的，并没有执行，需要使用消费或收集的方法来触发，
    // 比如 sum, collect 等，不过获取结果时必须明确注明类型，编译器无法推断
    let sum_value: i32 = v1.iter().map(|x| x + 1).sum();
    println!("map sum value: {}", sum_value);
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("map collect vec: {:?}", v2);

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_shoes = shoes_in_size(shoes, 10);
    assert_eq!(
        in_shoes,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );

}
