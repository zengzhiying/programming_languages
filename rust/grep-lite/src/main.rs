#![allow(unused_variables)]
use std::string::String;
use std::vec::Vec;
use regex::Regex;
use clap::Parser;
use clap::Arg;
use std::fs::File;
use std::io::{BufReader, BufRead};

// 支持标准输入到管道和文件方式运行
// ./grep-lite --pattern picture --file text.txt
// cat text.txt | ./grep-lite --pattern picture 

/// grep-lite program params
#[derive(Parser, Debug)]
struct Args {
    /// pattern in text
    #[arg(short, long)]
    pattern: String,

    /// pattern in file
    #[arg(short, long)]
    file_name: String
}

fn grep_lines<T: BufRead + Sized>(reader: T, re: Regex, pattern: &String, ctx_lines: usize) {
    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];
    let mut lines: Vec<String> = vec![];
    // 使用 enumerate 可以迭代出下标索引和内容
    for (i, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        
        let substr = re.find(&line);
        match substr {
            // 匹配成功
            Some(_) => println!("{}", line),
            // 匹配失败
            None => (),
        }
        if line.contains(pattern) {
            tags.push(i);
            let v = Vec::with_capacity(2*ctx_lines + 1);
            ctx.push(v);
        }

        lines.push(line);
    }

    if tags.is_empty() {
        return;
    }

    println!("context lines: ");
    for (i, line) in lines.iter().enumerate() {
        for (j, idx) in tags.iter().enumerate() {
            let low = idx.saturating_sub(ctx_lines);
            let upper = idx + ctx_lines;

            if i >= low && i <= upper {
                let selectd_line = String::from(line);
                let current = (i, selectd_line);
                ctx[j].push(current);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i,  ref line) in local_ctx.iter() {
            println!("{}: {}", i + 1, line);
        }
    }
}

fn main() {
    // 匹配上下文两行的内容
    let ctx_lines = 2;
    // let args = Args::parse();
    // let search_term = &args.pattern;
    // let file_name = args.file_name;
    let cmd = clap::command!()
        .arg(Arg::new("pattern")
            .short('p')
            .long("pattern")
            .help("pattern string.")
            .required(true)
        ).arg(Arg::new("file")
            .short('f')
            .long("file")
            .help("input file")
            .default_value("-")
        ).get_matches();
    // println!("{}", args.pattern);
    let search_term = cmd.get_one::<String>("pattern").unwrap();
    let re = Regex::new(search_term).unwrap();

    let input = cmd.get_one::<String>("file").unwrap();
    // 第一行的反斜线可以使编译器忽略掉换行符
    let text = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    // 标准输入为-
    if input == "-" {
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        grep_lines(reader, re, search_term, ctx_lines)
    } else {
        let file = File::open(input).unwrap();
        let reader = BufReader::new(file);
        grep_lines(reader, re, search_term, ctx_lines);
    }

    
}
