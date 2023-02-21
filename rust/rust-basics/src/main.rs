mod basic_type;
mod string_type;
mod complex_type;
mod control;
mod generics_type;
mod array_slice_type;
mod structure_type;
mod enum_type;
mod option_type;
mod struct_enum_method;
mod trait_example;
mod trait_object;
mod vector_type;
mod hashmap_type;
mod type_conversion;
mod result_panic;

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

    // 枚举类型
    println!("============ Enum type =============");
    enum_type::enum_type();

    // Option
    println!("=========== Option type =============");
    option_type::option_type();

    // struct method
    println!("=========== struct & enum method ============");
    struct_enum_method::struct_method();
    struct_enum_method::enum_method();
    

    // generics 实现
    println!("============ generics ============");
    generics_type::generics_add();
    generics_type::generics_max();
    generics_type::struct_generics();
    generics_type::enum_generics();
    generics_type::const_generics();

    // trait 
    println!("============== trait ================");
    trait_example::example();

    // trait object
    println!("============== trait object =================");
    trait_object::trait_object();

    // Vector type
    println!("================ vector type =================");
    vector_type::vector_type_example();

    // HashMap type
    println!("================ hash map type ==================");
    hashmap_type::hashmap_example();

    // type conversion
    println!("================ type conversion =================");
    type_conversion::type_conversion_example();

    // result panic
    println!("=============== result panic ===================");
    result_panic::result_panic_example();
}

