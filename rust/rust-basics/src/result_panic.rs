use std::{fs::File, io::{ErrorKind, Write, BufReader, BufRead, Read}};


pub fn result_panic_example() {
    // 打开文件 返回 Result 类型
    let f = File::open("rs.txt");
    // 错误处理
    let f = match f {
        Ok(f) => f,
        Err(err) => match err.kind() {
            // 文件不存在
            ErrorKind::NotFound => match File::create("rs.txt") {
                Ok(mut f) => {
                    let r = f.write(b"rust.");
                    match r {
                        Ok(size) => println!("write size: {}", size),
                        Err(err) => panic!("write err: {:?}", err)
                    }
                    // 如果文件不存在直接 panic 否则就返回文件句柄
                    File::open("rs.txt").unwrap()
                },
                Err(err) => panic!("Create file error: {:?}", err)
            }
            // 其他错误
            other_err => panic!("Open file error: {:?}", other_err)
        }
    };

    let reader = BufReader::new(f);
    for line in reader.lines() {
        match line {
            Ok(text) => println!("{:?}", text),
            Err(err) => panic!("{:?}", err.to_string())
        }
    }

    // expect 可以收集更详细的信息
    let mut f = File::open("hello.txt").expect("Failed to open hello.txt");
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    println!("read buf: {:?}", buf);
}
