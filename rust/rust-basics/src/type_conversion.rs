
pub fn type_conversion_example() {
    // 基本类型转换的原则 尽量小类型往大类型变换, 否则可能会出现损失
    // 另外注意转换不具有传递性, 例如: t as T1 as T2 成立并不能保证 t as T2 成立
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    // 转换字符为整数 97
    let c = 'a' as u8;
    println!("{},{},{}", a, b, c);

    // 内存地址获取并转换为指针
    let mut nums = [1,2];
    let p = nums.as_mut_ptr();
    // 转换指针为数字表示 由于数字直接 Copy 因此修改不影响原来的 p
    let mut address = p as usize;
    address += 4;
    let p1 = address as *mut i32;
    unsafe {
        println!("{}, {}", *p, *p1);
        *p1 += 1;
    }
    println!("{:?}", nums);

    // 可以使用 TryInto 实现更加可控的转换
    let v = 1500_i16;
    // 直接转换如果失败会直接 panic
    let v32: i32 = v.try_into().unwrap();
    println!("i16 {} -> i32 {}", v, v32);
    let v8: i8 = match v.try_into() {
        Ok(_v) => _v,
        Err(e) => {
            println!("err: {:?}", e.to_string());
            0
        }
    };
    println!("{}", v8);
}
