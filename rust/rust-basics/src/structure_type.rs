
// 通常结构体的字段不能包含引用, 每个结构体都有其完整的数据
// 如果包含引用则需要用到生命周期定义, 因为要确保结构体的作用范围小于其中属性的作用范围, 
// 否则在使用结构体的时候会出现属性无效的混乱情况, 因此如果不定义生命周期是无法在结构体中使用引用的
#[derive(Debug)]
struct Buffer {
    name: String,
    data: Vec<u8>
}

// 元组结构体 
// 只关心结构体的名字 字段比较简单
// 元组结构体没有实现 Display trait 可以加注解
#[derive(Debug)]
struct Point(i32, i32, i32);

// 单元结构体
// 没有字段, 类似于空的接口, 主要用来绑定方法使用
#[derive(Debug)]
struct Equal;

fn new_buffer(name: String, mut data: Vec<u8>) -> Buffer {
    data.push(8);
    data.push(9);
    data.push(10);
    // Buffer { name: name, data: data }
    // 如果字段名称和变量名称一致则也可以忽略 不一致的再单独写
    Buffer { name, data }
}

fn print_buffer(buf: &mut Buffer) {
    let j = &mut buf.data;

    j.push(3);

    println!("buffer: {:?} name: {} data: {:?}", buf, buf.name, buf.data);
}

pub fn structure_type() {
    // 如果后续要修改结构体的值 则必须使用 mut 声明
    let mut buf = new_buffer(String::from("hello"), Vec::new());
    print_buffer(&mut buf);
    println!("buf: {:?}", buf);
    buf.name = String::from("struct");
    println!("buf: {:?}", buf);
    // 结构体字段较多时可以换行输出 更加美观一些
    println!("buf: {:#?}", buf);
    // 还可以使用 dbg! 不过会转移所有权并且返回所有权, 可以使用引用或者接收所有权
    // buf = dbg!(buf);
    dbg!(&buf);

    // 结构体更新 有变化的手动列出其余的从原来的结构体获取
    let buf2 = Buffer{name: String::from("update"), ..buf};
    println!("buf2: {:?}", buf2);
    // 此时 buf 中的 data 所有权已经发生变化无法被访问, 但是可以单独访问 buf.name
    println!("buf.name: {:?}", buf.name);

    let p = Point(2, 6, 0);
    println!("point: {:?}", p);

    let e = Equal;
    println!("equal: {:?}", e);
}
