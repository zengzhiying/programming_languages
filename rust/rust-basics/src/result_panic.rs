use std::{fs::{File, self}, io::{ErrorKind, Write, BufReader, BufRead, Read, self}};


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

    let user = read_user_from_file("user");
    match user {
        Ok(username) => println!("username: {}", username),
        Err(e) => println!("read user error: {:?}", e)
    };

    let user = read_user_from_file2("user");
    match user {
        Ok(username) => println!("username: {}", username),
        Err(e) => println!("read user error: {:?}", e)
    };

    // 也可以直接使用 fs 下面的函数 直接返回内容
    let user = fs::read_to_string("user");
    match user {
        Ok(username) => println!("username: {}", username),
        Err(e) => println!("read user error: {:?}", e)
    }

    let arr = vec![2,6,9];
    let arr_first = array_first(&arr);
    if let Some(first) = arr_first {
        println!("array first: {}", first);
    }
    
}

// 错误传播 也就是将调用函数的错误传递给被调用函数
fn read_user_from_file(name: &str) -> Result<String, io::Error> {
    let f = File::open(name);
    let mut f = match f {
        Ok(f) => f,
        // 错误直接返回
        Err(e) => return Err(e)
    };

    let mut s = String::new();
    // match 中可直接返回, 注意不要加最后的分号, 整体为 1 个表达式
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

// 使用 ? 简化错误传播相关的代码, 使用 dyn 进行自动类型提升
fn read_user_from_file2(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 使用 ? 会自动处理错误, 因此我们只关心 ok 的返回就好
    // let mut f = File::open(name)?;
    let mut s = String::new();
    // 继续使用 ?
    // f.read_to_string(&mut s)?;
    // 还可以链式调用
    File::open(name)?.read_to_string(&mut s)?;
    // 早期版本还可以使用 try! 但是已经废弃了
    Ok(s)
}

// ? 还可以用于 Option 类型的返回
fn array_first(arr: &[i32]) -> Option<&i32> {
    // ? 表达的结果不能直接返回 必须通过变量承接并包装
    // 而且一旦使用了 ? 则必须保证函数要返回
    let v = arr.get(0)?;
    Some(v)
}
