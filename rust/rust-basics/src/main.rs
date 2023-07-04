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
mod object_lifetime;
mod print_example;
mod box_example;
mod deref_example;
mod drop_example;
mod rc_example;
mod cell_example;
mod weak_example;
mod self_referential;
mod pin_example;
mod concurrency;

use concurrency::multithreading;
use concurrency::thread_local_example;

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

    // lifetime
    println!("=============== life time =======================");
    object_lifetime::lifetime_example();

    // print
    println!("==================== print =======================");
    print_example::print_example();

    // box 智能指针
    println!("==================== Box ===========================");
    box_example::box_example();

    // deref 自动解引用
    println!("==================== Deref ============================");
    deref_example::deref_example();

    // drop 资源回收
    println!("===================== Drop ============================");
    drop_example::drop_example();

    // RC & ARC 引用计数指针与多线程共享的引用计数指针
    println!("====================== Rc & Arc ========================");
    rc_example::rc_example();
    rc_example::arc_example();

    // Cell & RefCell 在不可变引用基础上修改其可变性
    println!("====================== Cell & RefCell ===================");
    cell_example::cell_example();
    cell_example::refcell_example();

    // Weak 解决循环引用问题
    println!("====================== Weak =============================");
    weak_example::weak_example();

    // 结构体子引用实现
    println!("=============== Struct self referential =================");
    self_referential::unsafe_example();

    // Pin 在内存中固定值，防止被移动
    println!("==================== Pin ======================");
    pin_example::pin_example();

    // 多线程
    println!("=================== Multithreading ==================");
    multithreading::thread_example();

    // 线程屏障 Barrier
    println!("=================== Threading barrier ====================");
    multithreading::thread_barrier_example();

    // 线程局部变量
    println!("=================== Threading local variable =================");
    thread_local_example::threading_local_variable();

    // 第三方 thread_local 库
    println!("=================== Three-party thread_local ======================");
    thread_local_example::thread_local_three_party_example();
}
