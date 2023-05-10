// 所有权问题
// struct SelfRefer<'a> {
//     name: String,
//     value: &'a str,
// }

// 重新定义结构体
#[derive(Debug)]
struct SelfRefer {
    name: String,
    // 如果不修改可以定义为 *const，需要修改则定义为 *mut
    value: *const String,
    value2: *mut String
}

pub fn unsafe_example() {
    let s = "self refer".to_string();
    // let sr = SelfRefer{
    //     name: s,
    //     // 直接使用会有所有权问题，因为 s 的所有权已经转移了
    //     value: &s,
    // };
    let mut sr = SelfRefer {
        name: s,
        // 先使用 null 值
        value: std::ptr::null(),
        value2: std::ptr::null_mut(),
    };
    // 然后再设置值
    sr.value = &sr.name;
    sr.value2 = &mut sr.name;
    println!("sr: {:?}", sr);
    // 如果想获取子引用值需要使用 unsafe 操作
    
    let sr_value = unsafe {
        // 无法直接返回值 因为没实现 Copy trait
        &*sr.value
    };
    println!("sr value: {}", sr_value);

    // 通过 String 直接修改
    sr.name.push_str(". yes");
    // 通过 *mut 指针间接修改底层值
    unsafe {
        (*sr.value2).push_str("!");
    }
    // 最后底层的数据都是一样的
    println!("sr: {:?} v1: {}, v2: {}", sr, unsafe {&*sr.value}, unsafe {&*sr.value2});
}