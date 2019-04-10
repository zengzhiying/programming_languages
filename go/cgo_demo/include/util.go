package include

/*
// go调用c/c++源代码的情况下, 必须在同一个目录才可以直接调用
// 外部调用只需要使用go的函数中转一下即可
#include "util.h"
*/
import "C"

import "fmt"

func GoSum(a, b int) int {
    s := C.sum(C.int(a), C.int(b))
    fmt.Printf("%T\n", s)
    return int(s)
}
