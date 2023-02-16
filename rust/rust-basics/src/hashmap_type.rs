use std::collections::HashMap;



pub fn hashmap_example() {
    // 只有实现了 std::cmp::Eq trait 的类型才可以作为 key
    // 标准库中的 HashMap 默认使用的函数是 SipHash 可以指定第三方的散列函数例如 xxHash
    let mut m = HashMap::new();
    // 写入数据时会自动推断类型
    // 往 map 中放入变量同样符合所有权转移的原理
    m.insert("rust", 1);
    m.insert("java", 2);
    m.insert("python", 3);

    // 获取值 会返回 Option 类型, 查询不到时返回 None
    let v = m.get("rust");
    if let Some(n) = v {
        println!("rust: {}", n);
    } else {
        println!("None");
    }

    // 循环遍历
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
    // 若不存在则插入新值
    m.entry("php").or_insert(4);
    // 若存在则不影响
    m.entry("scala").or_insert(7);

    for (key, value) in &m {
        println!("{}, {}", key, value);
    }

    // Vec<tuple> 转 HashMap
    let v = vec![
        ("hello", 10),
        ("page", 20),
        ("in", 30),
        ("and", 50),
        ("use", 5)
    ];
    // 必须显示指定类型告诉编译器
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
