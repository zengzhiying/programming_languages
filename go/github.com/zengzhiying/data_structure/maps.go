package main

import "fmt"

type Vertex struct {
    latitude, longitude float64
}

// 通过结构体构造map, 默认空map为nil, 此时未分配空间不能使用
var m map[string]Vertex

func main() {
    if m == nil {
        m = make(map[string]Vertex)
    }
    m["beijing"] = Vertex{
        120.389102, 60.292912,
    }
    m["shanghai"] = Vertex{
        121.23, 58.192,
    }
    fmt.Println(m)
    fmt.Println(m["shanghai"])

    var m1 = map[string]Vertex{
        "k1": Vertex{122, 82.2},
    }
    fmt.Println(m1)

    var m2 = map[string]Vertex{
        // 顶级元素只有同一种类型value的可以省略如下
        "k2": {123, 29},
    }
    fmt.Println(m2)


    m3 := make(map[string]int)
    // 插入或赋值
    m3["age"] = 23
    fmt.Println(m3["age"])
    m3["test"] = 28
    // 删除元素
    delete(m3, "test")
    // 检测key是否存在并赋值
    elem, ok := m3["test"]
    if ok {
        fmt.Printf("test 存在, value: %d\n", elem)
    } else {
        // 不存在默认是0值
        fmt.Println("test 不存在, value:", elem)
    }
    elem, ok = m3["age"]
    if !ok {
        fmt.Println("age 不存在, value:", elem)
    } else {
        fmt.Printf("age 存在, value: %d\n", elem)
    }

    // map range形式遍历
    for k, v := range m {
        fmt.Printf("key: %v, value: %v\n", k, v)
    }
}
