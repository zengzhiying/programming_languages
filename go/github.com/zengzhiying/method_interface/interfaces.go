package main

import "fmt"

func main() {
    // 空接口, 可保存任何值
    var i interface {}
    describe(i)

    i = 42
    describe(i)

    i = "hello"
    describe(i)

    // 类型断言
    var i1 interface {} = "hello"

    // 如果i1保存了string的值, 则会把值赋给s, 反之则运行时会有异常
    s := i1.(string)
    fmt.Println(s)

    // 下面会引发panic
    // f := i1.(float)
    // fmt.Println(f)

    // 通过返回值判断断言是否成功, 接收返回值的时候程序不会抛出异常
    num, ok := i1.(int)
    fmt.Println(num, ok)
    s, ok = i1.(string)
    fmt.Println(s, ok)
    f, ok := i1.(float64)
    fmt.Println(f, ok)

    do_type(21)
    do_type("hello")
    do_type(true)

    var ii interface{}
    ii = 56
    editInter(ii)
    describe(ii)

    po := get_var()
    fmt.Println(*po)
}

// 接口本身是一个符号指向内存区域, 函数传参接口符号本身会复制一份
// 内部修改接口指向, 不影响外部变量
func editInter(i interface{}) {
    i = "hello"
}

func describe(i interface {}) {
    fmt.Printf("%v, %T\n", i, i)
}

func do_type(i interface {}) {
    // switch实现类型选择, i.(type) 和具体的i.(T)实质上是一样的
    switch v := i.(type) {
    case int:
        // 此时v是一个int变量
        fmt.Printf("%v is int.\n", v)
    case string:
        // 此时v是string类型
        fmt.Printf("%q is string. len: %d\n", v, len(v))
    default:
        // 此时v和i类型和值都相同
        fmt.Printf("未知类型: %T\n", v)
    }
}

// 逃逸分析
func get_var() *int {
    i := 40
    return &i
}
