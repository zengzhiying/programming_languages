package main

import (
    "fmt"
    "image"
)


func main() {
    // 通过image接口创建图像
    m := image.NewRGBA(image.Rect(0, 0, 100, 100))
    fmt.Println(m.Bounds())
    fmt.Println(m.At(0, 0).RGBA())
}
