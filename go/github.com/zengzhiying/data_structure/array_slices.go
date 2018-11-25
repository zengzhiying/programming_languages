package main

import "fmt"
import "strings"

func main() {
    var a [2]string
    a[0] = "Hello"
    a[1] = "World"
    fmt.Println(a[0], a[1])
    fmt.Println(a)

    primes := [6]int{2, 3, 5, 6, 7, 8}
    fmt.Println(primes)

    // 切片, 和python操作类似
    var s []int = primes[1:4]
    fmt.Println(s)
    // 但是注意数组切片不存放任何数据, 切片共享底层数据, 修改切片底层的数据会被修改
    s[1] = 9
    fmt.Println(primes)

    // 切片定义, 相当于数组去掉元素个数
    q := []int{1, 2, 3, 5, 6}
    fmt.Println(q)
    r := []bool{true, false, true}
    fmt.Println(r)
    st := []struct {
        i int
        b bool
    }{
        {2, true},
        {3, true},
        {5, false},
    }
    fmt.Println(st)

    // 切片默认值可以留空, 和python操作类似
    fmt.Println(primes[:2])
    fmt.Println(primes[2:])
    fmt.Println(primes[:])

    // 计算切片长度和容量
    s1 := []int{2, 5, 6, 7, 9, 10, 11}
    fmt.Println(len(s1), cap(s1))   // 默认都是7
    // 切为0
    s1 = s1[:0]
    fmt.Println(len(s1), cap(s1))  // 实际长度为0, 容量仍为7
    // 扩充(还原)一部分长度
    s1 = s1[:4]
    fmt.Println(len(s1), cap(s1))  // 实际长度为4, 容量仍为7
    // 删除开始2个元素, 必须先扩充然后才能在扩大后的范围内切片
    s1 = s1[2:]
    fmt.Println(len(s1), cap(s1))  // 实际长度为2, 容量为5, 这样删除之后将不可还原了
    // 总结: 切片操作只可滑动后面不改变容量, 起始点动了内存会被释放

    // 空切片不占用任何空间, 实质上为nil
    var s2 []int
    s1 = s2
    fmt.Println(s1, len(s1), cap(s1))
    if s1 == nil {
        fmt.Println("nil!")
    }


    // 用make动态创建切片
    ms := make([]int, 5)   // 默认元素值均为0, 长度和容量均为5
    fmt.Println(ms, len(ms), cap(ms))

    // 创建切片并指定长度和容量
    ms = make([]int, 0, 5)  // 长度为0, 容量为5
    fmt.Println(ms, len(ms), cap(ms))
    ms = ms[:cap(ms)]
    fmt.Println(ms, len(ms), cap(ms))

    // 切片元素可包含任何数据类型, 也包括切片
    board := [][]string {
        []string {"_", "_", "_"},
        []string {"_", "_", "_"},
        []string {"_", "_", "_"},
    }
    board[0][0] = "X"
    board[2][2] = "O"
    board[1][2] = "X"
    board[1][0] = "O"
    board[0][2] = "X"
    for i := 0; i < len(board); i++ {
        fmt.Printf("%s\n", strings.Join(board[i], " "))
    }

    // 向切片追加新元素, Go会自动分配空间和扩容
    var ap []int
    printSlice(ap)
    // 在原有的切片上追加新的切片, 返回包含原切片和新加的元素的切片, 原有空间会重复使用
    ap = append(ap, 0)
    printSlice(ap)
    ap = append(ap, 1)
    printSlice(ap)
    ap = append(ap, 2, 3, 4)
    printSlice(ap)
    ap = append(ap, 5, 6)
    printSlice(ap)
    // 切片的range形式遍历
    for i, v := range ap {
        // i为下标值, v为对应切片元素的副本
        fmt.Printf("index: %d, value: %d\n", i, v)
    }

    // 忽略下标或者value
    pow := make([]int, 10)
    // 仅迭代下标赋值
    for i := range pow {
        pow[i] = 1 << uint(i)
    }
    // 使用下划线忽略下标
    for _, value := range pow {
        fmt.Printf("%d ", value)
    }
    fmt.Println()

    es := make([]int, 3)
    // var es = []int{0, 0, 0}
    es[0] = 1
    es[1] = 2
    es[2] = 3
    fmt.Println(es)
    editSlice(es)
    editSlicePoint(&es)
    fmt.Println(es)

    var es2 [3]int
    es2[0] = 1
    es2[1] = 2
    es2[2] = 3
    fmt.Println(es2)
    editArray(es2)
    fmt.Println(es2)
}

// 函数内部修改数组无效, 数组是在栈上, 参数传递值
// 这个和C不一样, C数组是传递指针
// 如果传递指针类型, 则可以修改
func editArray(s [3]int) {
    s[1] = 6
}

// 只要是切片类型, 普通函数都可以直接修改
func editSlice(s []int) {
    s[1] = 5
    // 函数传参和c一样是传递符号的一个副本, 
    // 这里初始化给副本不会影响之前的区域
    s = make([]int, 3)
}

// 函数接收指针修改, 一般不使用
func editSlicePoint(s *[]int) {
    (*s)[2] = 6
}

func printSlice(s []int) {
    fmt.Printf("%v, len=%d, cap=%d\n", s, len(s), cap(s))
}
