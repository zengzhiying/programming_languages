package main

import "fmt"
import "strings"

type Person struct {
    Name string
    Age int
}

type IPAddr [4]byte

// 绑定Person结构体string接口
func (p Person) String() string {
    return fmt.Sprintf("%v, age: %v", p.Name, p.Age)
}

func (ip IPAddr) String() string {
    var ipNum [len(ip)]string
    for i := 0; i < len(ipNum); i++ {
        ipNum[i] = fmt.Sprintf("%d", ip[i])
    }
    return strings.Join(ipNum[:], ".")
}

func main() {
    a := Person{"zhangsan", 42}
    z := Person{"lisi", 52}
    // 输出结构体的时候, 就是调用绑定的string接口函数来打印值
    fmt.Println(a, z)


    hosts := map[string]IPAddr {
        "lookback": {127, 0, 0, 1},
        "googleDNS": {8, 8, 8, 8},
    }
    for name, ip := range hosts {
        fmt.Printf("%v: %v\n", name, ip)
    }
}
