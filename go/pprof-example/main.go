package main

import (
	"bytes"
	"fmt"
	"math/rand"
	"os"
	"runtime/pprof"

	// "github.com/pkg/profile"

	"net/http"
	_ "net/http/pprof"

	ginpprof "github.com/gin-contrib/pprof"
	"github.com/gin-gonic/gin"
)

const letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

func main() {
	// 普通程序 pprof 不能和 web 服务同时开启
	// profile()

	r := gin.Default()
	// 使用 gin pprof 中间件
	// 也可以使用原生 http
	// go func() {
	// 	err := http.ListenAndServe(":9090", nil)
	// 	if err != nil {
	// 		fmt.Printf("Start pprof server error: %s\n", err)
	// 	}
	// }()
	// http://localhost:8080/debug/pprof
	// 每次启动时仅分析 1 次 默认等待采样 30s 可以使用 seconds=60 修改采样的时间
	// 远程采集信息开服务分析: go tool pprof -http :8888 localhost:9090/debug/pprof/profile
	ginpprof.Register(r)
	r.GET("/", func(ctx *gin.Context) {
		ctx.JSON(http.StatusOK, gin.H{
			"value": "hello",
		})
	})
	r.GET("/pprof", func(c *gin.Context) {
		n := 10
		for i := 1; i <= 5; i++ {
			fmt.Printf("fib(%d)=%d\n", n, fib(n))
			n += 3 * i
		}

		var v string
		for i := 0; i < 128; i++ {
			p := randomString(letters, 100)
			v = repeat(p, 100)
		}

		c.String(http.StatusOK, v)
	})
	r.Run(":8080")
}

func profile() {
	// 生成文件使用 go tool pprof cpu.profile 进入调试
	// top: 查看热点
	// list <func>: 进入方法调用
	// help: 查看帮助
	f, err := os.OpenFile("cpu.profile", os.O_CREATE|os.O_RDWR, 0644)
	if err != nil {
		panic(err)
	}

	defer f.Close()

	pprof.StartCPUProfile(f)

	defer pprof.StopCPUProfile()

	n := 10
	for i := 1; i <= 5; i++ {
		fmt.Printf("fib(%d)=%d\n", n, fib(n))
		n += 3 * i
	}

	// 内存分析
	f1, err := os.OpenFile("mem.profile", os.O_CREATE|os.O_RDWR, 0644)
	if err != nil {
		panic(err)
	}
	defer f1.Close()

	var v string
	for i := 0; i < 128; i++ {
		p := randomString(letters, 100)
		v = repeat(p, 100)
	}

	fmt.Println(v)

	pprof.Lookup("heap").WriteTo(f1, 0)

	// 借助第三方库进行实时分析
	// defer profile.Start().Stop()

	// 启用 Memory 设置采样率
	// defer profile.Start(profile.MemProfile, profile.MemProfileRate(1)).Stop()

	// 根据 profile 生成火焰图
	// apt install graphviz
	// go tool pprof -http :8080 cpu.profile
	// go tool pprof -svg cpu.profile > cpu.svg
}

func fib(n int) int {
	switch n {
	case 0:
		return 0
	case 1, 2:
		return 1
	default:
		return fib(n-1) + fib(n-2)
	}
}

func randomString(s string, l int) string {
	var buf bytes.Buffer
	for i := 0; i < l; i++ {
		buf.WriteByte(s[rand.Intn(len(s))])
	}
	return buf.String()
}

func repeat(s string, n int) string {
	var r string
	for j := 0; j < n; j++ {
		r += s
	}
	return r
}
