package main
/*
// 基本方式调用: 内嵌方式
#include <stdio.h>
int add(int a, int b) {
    return a + b;
}
*/
import "C"

import "fmt"

func main() {
    s := C.add(2, 6)
    fmt.Println(s)
}
