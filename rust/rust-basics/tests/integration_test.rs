// 集成测试必须按照项目名作为顶级来引入 因为每个集成测试文件都是一个独立的 crate
use rust_basics::structure_type::Rectangle;

#[test]
fn rectangle_contain() {
    let larger = Rectangle::new(8, 7);
    let smaller = Rectangle::new(5, 1);
    // assert! 断言是否为 true，如果是 false 则测试会 panic
    // assert!(larger.contain(&smaller));
    // 后面还支持添加 format! 参数，在断言失败时打印更多详细的信息
    assert!(larger.contain(&smaller), "Rectangle `{:?}` not contain `{:?}`", larger, smaller);
}
