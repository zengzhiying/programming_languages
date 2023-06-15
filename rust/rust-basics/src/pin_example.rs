use std::{ptr::NonNull, marker::PhantomPinned, pin::Pin};


// slice 是保存指向 data 的引用，因此 Unmovable 属于子引用结构体
// 不过由于 Rust 所有权规则，这无法直接通过普通引用方式来实现
struct Unmovable {
    data: String,
    // NonNull 表示非空的指针
    slice: NonNull<String>,
    // 作为标记，表示不是先 Unpin trait，去掉也不影响
    _pin: PhantomPinned
}

impl Unmovable {
    fn new(data: String) -> Pin<Box<Self>> {
        let um = Unmovable{
            data,
            // 先创建悬空但有效的指针，用于延迟化分配
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(um);
        // 通过 from 获取 NonNull 指针
        let slice = NonNull::from(&boxed.data);
        let um_pin_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
        unsafe {
            // 修改字段
            Pin::get_unchecked_mut(um_pin_ref).slice = slice;
        }

        boxed

    }
}

pub fn pin_example() {
    let unmoved = Unmovable::new("hello".to_string());
    let unmoved1 = unmoved;
    // 无论实例怎么转移，实例的 slice 永远指向其 data
    assert_eq!(unmoved1.slice, NonNull::from(&unmoved1.data));
    println!("passed.")
}
