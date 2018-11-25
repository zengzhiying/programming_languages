package main

import "fmt"

import "golang.org/x/tour/tree"

// 利用轻量级线程和信道的相关原理来比对二叉查找树序列是否相同

// 将二叉树所有的的元素逐个发送到信道ch中
func Walk(t *tree.Tree, ch chan int) {
    // 调用发送函数
    sendValue(t, ch)
    // 关闭信道, 以便取值时捕获到状态
    close(ch)
}

// 递归调用将二叉树的值传送给ch, 前序遍历方式
func sendValue(t *tree.Tree, ch chan int) {
    if t != nil {
        sendValue(t.Left, ch)
        ch <- t.Value
        sendValue(t.Right, ch)
    }
}


// 检测二叉树t1和t2两棵树是否相等
func Same(t1, t2 *tree.Tree) bool {
    ch1 := make(chan int)
    ch2 := make(chan int)
    go Walk(t1, ch1)
    go Walk(t2, ch2)
    // 循环迭代信道, 直至退出, 当信道关闭时, 返回false会自动退出
    for i := range ch1 {
        // 出信道顺序一致, 如果有一个元素不相等, 则说明两棵树不相等
        if i != <- ch2 {
            return false
        }
    }
    return true
}

func main() {
    var ch = make(chan int)
    go Walk(tree.New(2), ch)
    for v := range ch {
        fmt.Println(v)
    }

    fmt.Println(Same(tree.New(1), tree.New(1)))
    fmt.Println(Same(tree.New(1), tree.New(2)))
    fmt.Println(Same(tree.New(2), tree.New(2)))
}
