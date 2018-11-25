package main

import (
    "fmt"
    "math"
    //"github.com/gomodule/redigo/redis"
    // other
    "github.com/zengzhiying/being_import"
    // current
    "github.com/zengzhiying/basics/include"
)

func main() {
    fmt.Printf("Now you have %g problems.\n", math.Sqrt(7))
    fmt.Println(being_import.PPi)
    fmt.Println(include.Kaa)
}
