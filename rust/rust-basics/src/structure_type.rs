
#[derive(Debug)]
struct Buffer {
    name: String,
    data: Vec<u8>
}

fn new_buffer(name: String, mut data: Vec<u8>) -> Buffer {
    data.push(8);
    data.push(9);
    data.push(10);
    Buffer { name: name, data: data }
}

fn print_buffer(buf: &mut Buffer) {
    let j = &mut buf.data;

    j.push(3);

    println!("buffer: {:?} name: {} data: {:?}", buf, buf.name, buf.data);
}

pub fn structure_type() {
    let mut buf = new_buffer(String::from("hello"), Vec::new());
    print_buffer(&mut buf);
    println!("buf: {:?}", buf);
}
