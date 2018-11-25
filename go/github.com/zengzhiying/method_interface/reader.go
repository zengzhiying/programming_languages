package main

import "fmt"
import "io"
import "strings"

func main() {
    r := strings.NewReader("Hello, Reader!")

    b := make([]byte, 8)
    for {
        // 使用io接口读取字节到slice b
        n, err := r.Read(b)
        fmt.Printf("n = %v, err = %v, b = %v\n", n, err, b)
        fmt.Printf("b[:n] = %q\n", b[:n])
        if err == io.EOF {
            break
        }
    }
}
