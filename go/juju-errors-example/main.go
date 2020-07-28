package main

import (
	"os"
	"fmt"

	"github.com/juju/errors"
)

func main () {
	f, err := os.Open("file.go")
	fmt.Println(f)
	if err != nil {
		// 普通
		fmt.Println("1",err)
		// juju
		// errors.New("error1")
		err1 := errors.Trace(err)
		fmt.Println("2",err1)
		fmt.Println("3",errors.Annotate(err, "more context"))
		fmt.Printf("4: %v\n", errors.Cause(err))
		fmt.Println(errors.ErrorStack(err1))
	}

	err = errors.Errorf("original")
	// 为已有的错误继续追加上下文信息 会记录当前的行号、功能等, 便于对一类错误的不同位置进行定位
	err = errors.Annotate(err, "context")
	err = errors.Annotate(err, "more context")
	fmt.Println(err.Error())
	fmt.Println(errors.ErrorStack(err))
	// 返回一行
	fmt.Println(errors.Details(err))


	// err2 := errors.NewErr("err2: error!")
	// fmt.Println("err2", err2.Error())
	// fmt.Println(err2.Message())
	// fmt.Println(err2.Location())

	// fmt.Println("stack trace:", err2.StackTrace())
}
