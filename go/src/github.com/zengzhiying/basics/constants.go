package main

import "fmt"

// 常量声明均使用const关键字, 类型可以是字符、字符串、布尔值或数值
// 类型自动判断, 不用指定
const Pi = 3.14

// 数值常量是高精度的值, 同样可以分组声明
// int最大可以存储64位的整数, 但是根据平台不同有时候会更小
const (
    // Create a huge number by shifting a 1 bit left 100 places.
    // In other words, the binary number that is 1 followed by 100 zeroes.
    Big = 1 << 100
    // Shift it right again 99 places, so we end up with 1<<1, or 2.
    Small = Big >> 99
)

func needInt(x int) int { return x*10 + 1 }
func needFloat(x float64) float64 {
    return x * 0.1
}

func main() {
    // 常量不能使用 := 声明
	const World = "世界"
	fmt.Println("Hello", World)
	fmt.Println("Happy", Pi, "Day")

	const Truth = true
	fmt.Println("Go rules?", Truth)


    // 测试输出
    fmt.Println(needInt(Small))
    fmt.Println(needFloat(Small))
    fmt.Println(needFloat(Big))
}
