

pub fn array_type() {
    // 数组自动推断类型 编译时确定
    let a = [1,2,3];
    // 设置指定的类型和长度
    let b: [u8; 3] = [1,2,3];

    // 使用0填充 长度为3
    let c = [0; 3];
    let d: [u8; 3] = [0; 3];

    // 二维数组，其中每个数组类型必须一致，数组类型包括元素类型和长度
    let arrs = [a, b, c, d];

    for arr in arrs {
        print!("{:?}: ", arr);
        for n in arr.iter() {
            print!("\t {}", n);
        }

        let mut sum = 0;
        for i in 0..arr.len() {
            sum += arr[i];
        }

        println!("\t ∑{:?} = {}", arr, sum);
    }

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    let b = [2, 3];
    assert_eq!(slice, &b);
}
