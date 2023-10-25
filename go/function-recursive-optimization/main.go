package main

import (
	"fmt"
	"time"
)

// Go 函数递归优化的思路
// 参考：https://geekr.dev/posts/go-recursive-function-and-optimization

type RecursiveFunction func(int) int

// 斐波那契数列默认递归方式
func fibonacci1(i int) int {
	if i <= 2 {
		return i - 1
	}

	return fibonacci1(i-1) + fibonacci1(i-2)
}

// 斐波那契数列递归优化，将结果压到参数中，从而减少重复计算
func fibonacci2(i int) int {
	return subFibonacci2(i, 0, 1)
}

func subFibonacci2(i int, f, l int) int {
	if i < 2 {
		return f
	}

	return subFibonacci2(i-1, l, f+l)
}

// 阶乘计算递归
func factorial1(i int) int {
	if i == 1 {
		return 1
	}
	return i + factorial1(i-1)
}

// 阶乘递归优化
func factorial2(i int) int {
	return subFactorial2(i, 1)
}

func subFactorial2(i, s int) int {
	if i == 1 {
		return s
	}

	return subFactorial2(i-1, i+s)
}

// 统计执行时间
func executeTime(f RecursiveFunction) RecursiveFunction {
	return func(i int) int {
		start := time.Now()
		num := f(i)
		end := time.Since(start)
		fmt.Printf("------ time: %v -------\n", end)
		return num
	}
}

func main() {
	// 斐波那契数列通过将计算压到参数中，从而减少了重复计算，可以带来很大的性能提升
	fmt.Printf("fibonacci1 10: %d\n", executeTime(fibonacci1)(10))
	fmt.Printf("fibonacci1 40: %d\n", executeTime(fibonacci1)(40))

	fmt.Printf("fibonacci2 10: %d\n", executeTime(fibonacci2)(10))
	fmt.Printf("fibonacci2 40: %d\n", executeTime(fibonacci2)(40))

	// 由于 Go 没有尾递归优化，并且阶乘都是调用自身，基本上没有重复计算，所以优化效果不大
	fmt.Printf("factorial1 20: %d\n", executeTime(factorial1)(10000))
	fmt.Printf("factorial2 20: %d\n", executeTime(factorial2)(10000))
}
