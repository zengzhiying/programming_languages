mod basic_type;
mod compound_type;
mod string_type;
mod complex_type;
mod control;
mod generics_type;
mod slice_type;
mod structure_type;
mod enum_type;
mod option_type;
mod enum_method;
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
mod functional;
mod iterator_example;

use concurrency::multithreading;
use concurrency::thread_local_example;
use concurrency::condition_variables;
use concurrency::initialize_only_once;
use concurrency::channel_example;
use concurrency::flume_example;
use concurrency::mutex_example;
use concurrency::semaphore_example;
use concurrency::atomic_example;
use concurrency::memory_barrier;
use concurrency::send_sync;
use functional::closures;

fn main() {
    println!("Hello, Rust!");

    // 基本类型 - 标量类型
    println!("========== Basic type =============");
    basic_type::data_type_basic();
    // 复合类型 - 元组 & 数组
    println!("=========== Tuple & Array =============");
    compound_type::compound_data_type();

    // 字符串类型
    println!("============ String type ====================");
    string_type::string_basic();
    string_type::text_process();

    // print
    println!("==================== print =======================");
    print_example::print_example();

    // 控制流
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
    println!("========== Control loop-break ===========");
    control::loop_control();

    // 切片类型
    println!("============= Slice type ===========");
    slice_type::slice_type();

    // 复数使用
    println!("========== Complex type =================");
    complex_type::complex_type();

    // 自定义结构体
    println!("============ Struct type & Method =============");
    structure_type::structure_type();
    structure_type::structure_method();

    // 枚举类型
    println!("============ Enum type & Method =============");
    enum_type::enum_type();
    enum_method::enum_method();

    // type conversion
    println!("================ type conversion =================");
    type_conversion::type_conversion_example();

    // Option
    println!("=========== Option enum type =============");
    option_type::option_type();

    // 集合类型 Vector, String, Hash Map
    // Vector type
    println!("================ Vector type =================");
    vector_type::vector_type_example();

    // HashMap type
    println!("================ Hasp Map type ==================");
    hashmap_type::hashmap_example();

    // result panic
    println!("=============== result panic ===================");
    result_panic::result_panic_example();

    // generics（泛型） 实现
    println!("============ generics ============");
    generics_type::generics_max();
    generics_type::generics_add();
    generics_type::struct_generics();
    generics_type::enum_generics();
    generics_type::const_generics();

    // trait 
    println!("============== trait ================");
    trait_example::example();

    // trait object
    println!("============== trait object =================");
    trait_object::trait_object();

    // lifetime
    println!("=============== life time =======================");
    object_lifetime::lifetime_example();

    // functional Closures
    println!("======================= functional Closures ====================");
    closures::closures_example();

    // Iterator
    println!("======================== Iterator ==========================");
    iterator_example::iterator_example();

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

    // 条件变量
    println!("================= Condition variables ======================");
    condition_variables::condition_variables_example();

    // 初始化一次
    println!("================= Initialize once ===================");
    initialize_only_once::initialize_only_once_example();

    // 多线程通信 channel 多生产者单消费者
    println!("================= Channel mpsc =======================");
    channel_example::mpsc_example();

    // 多线程通信 flume 库，多生产者多消费者
    println!("====================== flume mpmc ==========================");
    flume_example::flume_example();

    // 互斥锁使用
    println!("======================= Mutex ==========================");
    mutex_example::mutex_example();

    // 读写锁使用
    println!("======================= RwLock ========================");
    mutex_example::rw_mutex_example();

    // 信号量使用
    println!("======================= Semaphore =======================");
    semaphore_example::tokio_semaphore_example();

    // 原子操作
    println!("======================= Atomic =========================");
    atomic_example::atomic_example();

    // 内存屏障
    println!("======================= Memory Barrier =======================");
    memory_barrier::memory_barrier_example();

    // Send & Sync
    println!("====================== Send & Sync ======================");
    send_sync::send_sync_example();
}
