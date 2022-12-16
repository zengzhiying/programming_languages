mod basic_type;
mod string_type;
mod complex_type;
mod control;
mod generics_type;
mod array_slice_type;
mod structure_type;

fn main() {
    println!("Hello, Rust!");

    // 基本类型
    println!("========== Basic type =============");
    basic_type::data_type_basic();

    // 字符串类型
    println!("============ String type ====================");
    string_type::string_basic();
    string_type::text_process();

    // 复数使用
    println!("========== Complex type =================");
    complex_type::complex_type();

    // 分支和循环语法
    println!("========= Control if-else ============");
    control::if_else_control();
    println!("========= Control match ===========");
    control::match_control();
    println!("========== Control for ================");
    control::for_control();
    println!("========== Control while =========");
    control::while_control();
    println!("========== Control for-break =========");
    control::break_control();

    // 数组和切片
    println!("========== Array & Slice type ===========");
    array_slice_type::array_type();

    // 自定义结构体
    println!("============ Struct type =============");
    structure_type::structure_type();
    

    // 自定义 trait 实现
    println!("============ trait ============");
    generics_type::generics_add();

}


