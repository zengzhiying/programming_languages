package main

/*
#cgo CFLAGS: -Iinclude
// 编译好的动态链接库放在lib目录下
#cgo LDFLAGS: -Llib -lvideo
#include "video.h"
*/
import "C"

import "fmt"
import "unsafe"

func main() {
    s := C.CString("Hello World!")
    fmt.Printf("%v %T \n", s, s)
    C.video_proc(s)
    C.video_proc1(unsafe.Pointer(s))
}
