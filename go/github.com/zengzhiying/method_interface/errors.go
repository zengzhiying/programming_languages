package main

import "fmt"
import "time"
import "math"
import "strconv"

type MyError struct {
    When time.Time
    What string
}

//  自定义err输出
func (e *MyError) Error() string {
    return fmt.Sprintf("at %v, %s", e.When, e.What)
}

// 自定义异常返回
func run() error {
    return &MyError{
        time.Now(),
        "it didn't work",
    }
}

type ErrNegativeSqrt float64

func (e ErrNegativeSqrt) Error() string {
    // 这里实现了error接口输出后, 输出一定不要直接包含e, 否则相当于再次调用e.Error(), 导致无限递归循环
    return fmt.Sprintf("cannot Sqrt negative number: %v\n", float64(e))
}

func Sqrt(x float64) (float64, error) {
    if x < 0 {
        return 0, ErrNegativeSqrt(x)
    }
    return math.Sqrt(x), nil
}



func main() {
    if err := run(); err != nil {
        fmt.Println(err)
    }


    // 使用其他库函数的错误返回
    i, err := strconv.Atoi("42")
    if err != nil {
        fmt.Printf("Convert error! err: %v\n", err)
        return
    }
    fmt.Printf("Convert ok! %d\n", i)

    // i, err = strconv.Atoi("ab")
    // if err != nil {
    //     fmt.Printf("Convert error! err: %v\n", err)
    //     return
    // }
    // fmt.Printf("Convert ok! %d\n", i)


    fmt.Println(Sqrt(2))
    fmt.Println(Sqrt(-2))
    // 外部调用error
    fmt.Println(ErrNegativeSqrt(-2).Error())
}
