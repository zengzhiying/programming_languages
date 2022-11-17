use num::complex::Complex;

// 复数库 需要在Cargo.toml中导入num依赖
pub fn complex_type() {
    let a = Complex{re: 2.1, im: -3.2};
    let b = Complex::new(11.1, 23.2);
    let r = a + b;
    println!("{} + {}i", r.re, r.im);
}