use std::collections::HashMap;

pub fn hashmap_example() {
    // 只有实现了 std::cmp::Eq trait 的类型才可以作为 key
    // 标准库中的 HashMap 默认使用的函数是 SipHash 可以指定第三方的散列函数例如 xxHash
    // HashMap 本身是在堆上，符合 Rust 中对象的所有权规则
    let mut m = HashMap::new();
    // 写入数据时会自动推断类型
    // 往 HashMap 中放入变量同样符合所有权转移的原理，多次写入相同的 key 会覆盖
    // 如果往 HashMap 写入引用，则值本身不会被转移至 HashMap，但是要确保 HashMap 在有效时这些引用也必须有效（符合生命周期）
    m.insert("rust", 1);
    m.insert("java", 2);
    m.insert("python", 3);

    // 获取值会返回 Option 类型, 查询不到时返回 None
    let v = m.get("rust");
    if let Some(n) = v {
        println!("rust: {}", n);
    } else {
        println!("None");
    }

    // 直接获取结果，copied 获取 Option<i32> 而不是 Option<&i32>
    let j = m.get("java").copied().unwrap_or(0);
    println!("java: {}", j);

    // 循环遍历，引用方式只读遍历
    for (key, num) in &m {
        println!("{}, {}", key, num);
    }

    // 直接遍历后 m 会被释放
    for (key, num) in m {
        println!("{}, {}", key, num);
    }

    // 所有权发生转移
    // println!("{:?}", m);

    // 提前分配空间
    let mut m: HashMap<&str, i32> = HashMap::with_capacity(10);
    m.insert("rust", 1);
    m.insert("java", 2);
    m.insert("python", 3);
    m.insert("scala", 5);
    m.insert("go", 6);
    // 重复插入会覆盖已有的值
    m.insert("python", 6);
    // 若不存在则插入新值，这样就不需要自己编写逻辑了
    m.entry("php").or_insert(4);
    // 若存在则不影响
    m.entry("scala").or_insert(7);

    // 根据原有值来更新，获取可变引用
    let c = m.entry("rust").or_insert(0);
    *c += 1;

    // 到这里可变引用 c 会被编译器释放，保证符合 Rust 的所有权规则
    for (key, value) in &m {
        println!("{}, {}", key, value);
    }

    // 这里无法使用可变引用
    // println!("c: {}",*c);

    // Vec<tuple> 转 HashMap
    let v = vec![
        ("hello", 10),
        ("page", 20),
        ("in", 30),
        ("and", 50),
        ("use", 5)
    ];
    // 必须显式指定类型告诉编译器
    let score_by_key: HashMap<_, _> = v.into_iter().collect();
    println!("{:?}", score_by_key);

    let text = "hello world wonderful world".to_string();
    let result = word_count(text);
    println!("{:?}", result);

}

// 基于 HashMap 进行 word count 示例
fn word_count(text: String) -> HashMap<String, i32> {
    let mut count = HashMap::new();
    for word in text.split_whitespace() {
        let count = count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    count
}
