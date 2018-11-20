package main

/**
 * go 基本数据类型
 */

import (
	"fmt"
	"math/cmplx"
)

// 变量声明也可以分组
var (
    // bool
	ToBe   bool       = false
    // uint64 
	MaxInt uint64     = 1<<64 - 1
    // 复数, 对应float64
	z      complex128 = cmplx.Sqrt(-5 + 12i)
)

func main() {
	fmt.Printf("Type: %T Value: %v\n", ToBe, ToBe)
	fmt.Printf("Type: %T Value: %v\n", MaxInt, MaxInt)
	fmt.Printf("Type: %T Value: %v\n", z, z)
}

/*
基本类型:
    bool
    string
    int int8 int16 int32 int64
    uint uint8 uint16 uint32 uint64 uintptr
    byte - uint8的别名
    rune - int32的别名, 一个Unicode码点
    float32 float64
    complex64 complex128

int, uint 和 uintptr 在 32 位系统上通常为 32 位宽，在 64 位系统上则为 64 位宽。 
当你需要一个整数值时应使用 int 类型，除非你有特殊的理由使用固定大小或无符号的整数类型。

没有明确初始值的变量声明会被赋予它们的 零值。
数值类型为 - 0
布尔类型为 - false
字符串为 - "", 空字符串
 */
