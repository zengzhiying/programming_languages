package main

import "fmt"

type Vertex struct {
    X int
    Y int
}

// 简便方法定义
type Vertex1 struct {
    X, Y int
}

var (
    v1 = Vertex1{1, 2}
    v2 = Vertex1{X: 1}  // 默认Y为0
    v3 = Vertex1{}  // 空结构体, 默认X, Y均为0, 具体取决于变量的默认值
    po = &Vertex1{1, 3} // 指针可直接赋值
)

// 普通函数内部修改结构体的值无效, 这个和C语言一致属于值传递, 和数组一样
// 参数相当于复制了一份完整的, 注意这点和切片, map不一样. 
func editStruct(st Vertex) {
    st.X = 6
}

// 通常使用结构体指针在函数内部间接修改结构体的数据
func editStructPoint(st *Vertex) {
    st.X = 6
}

func main() {
    fmt.Println(Vertex{1, 2})

    v := Vertex{3, 4}
    v.Y = 7
    fmt.Println(v)

    // 结构体指针
    p := &v
    // 这样太繁琐
    // (*p).X = 37
    // 可以直接支持用普通变量的方式访问
    p.X = 37
    fmt.Println(*p, v)


    fmt.Println(v1, po, v2, v3)

    v_t := Vertex{}
    editStruct(v_t)
    fmt.Println(v_t)
    editStructPoint(&v_t)
    fmt.Println(v_t)

    // new出来的结构体, 返回的都是结构体指针类型, 指向堆内存地址
    v_n := new(Vertex)
    editStructPoint(v_n)
    fmt.Println(*v_n)
}
