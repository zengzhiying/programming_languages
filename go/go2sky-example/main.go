package main

import (
	"context"
	"fmt"
	"io"
	"net/http"
	"os"
	"time"

	"github.com/SkyAPM/go2sky"
	sky_plugin "github.com/SkyAPM/go2sky-plugins/gin/v3"
	sky_http "github.com/SkyAPM/go2sky/plugins/http"
	"github.com/SkyAPM/go2sky/reporter"
	"github.com/gin-gonic/gin"
)

// 插件参考: https://github.com/SkyAPM/go2sky-plugins
func main() {
	report, err := reporter.NewGRPCReporter("localhost:11800")
	if err != nil {
		fmt.Printf("New reporter error: %s\n", err)
		return
	}
	defer report.Close()

	if len(os.Args) == 2 && os.Args[1] == "server" {
		// gin http server
		trace, err := go2sky.NewTracer("gin-http-server", go2sky.WithReporter(report), go2sky.WithSampler(0.6))
		if err != nil {
			fmt.Printf("New server trace error: %s\n", err)
			return
		}
		gin.SetMode(gin.ReleaseMode)
		r := gin.New()
		r.Use(sky_plugin.Middleware(r, trace))

		// 设置全局 trace
		// go2sky.SetGlobalTracer(trace)
		r.GET("/user/:name", func(c *gin.Context) {
			name := c.Param("name")
			// 内部可以获取使用
			// trace := go2sky.GetGlobalTracer()
			c.String(200, "Hello %s", name)
		})

		r.Run(":8998")
	}

	trace, err := go2sky.NewTracer("message-example", go2sky.WithReporter(report))
	if err != nil {
		fmt.Printf("New trace error: %s\n", err)
		return
	}
	for i := 0; i < 100; i++ {
		// local span
		fmt.Printf("span number: %d\n", i)
		span, _, err := trace.CreateLocalSpan(context.Background())
		if err != nil {
			fmt.Printf("Create local span error: %s\n", err)
			return
		}
		span.SetOperationName("message1")
		span.SetPeer("Redis")
		// Redis id is 7
		span.SetComponent(7)
		// 设置各类标签属性等
		// span.Tag(go2sky.TagMQQueue, "mq-1")
		// span.Tag(go2sky.Tag("custom"), "redis-mq")
		time.Sleep(300 * time.Millisecond)
		span.End()

		span, ctx, err := trace.CreateLocalSpan(context.Background())
		if err != nil {
			fmt.Printf("Create local span error: %s\n", err)
			return
		}
		span.SetOperationName("message2")
		span.SetPeer("Kafka")
		// Kafka id is 27
		span.SetComponent(27)
		subSpan, _, err := trace.CreateLocalSpan(ctx)
		if err != nil {
			fmt.Printf("Create sub local span error: %s\n", err)
			return
		}
		subSpan.SetOperationName("message2-1")
		subSpan.SetPeer("Kafka-producer")
		time.Sleep(500 * time.Millisecond)
		subSpan.End()

		subSpan, _, err = trace.CreateLocalSpan(ctx)
		if err != nil {
			fmt.Printf("Create sub local span error: %s\n", err)
			return
		}
		subSpan.SetOperationName("message2-2")
		subSpan.SetPeer("Kafka-flush")
		time.Sleep(200 * time.Millisecond)

		subSpan.End()
		span.End()

		time.Sleep(time.Second)

		// http client
		client, err := sky_http.NewClient(trace)
		if err != nil {
			fmt.Printf("New http client error: %s\n", err)
			return
		}

		request, err := http.NewRequest(http.MethodGet, "http://127.0.0.1:8998/user/gin", nil)
		if err != nil {
			fmt.Printf("New http request error: %s\n", err)
			return
		}

		response, err := client.Do(request)
		if err != nil {
			fmt.Printf("Http request error: %s\n", err)
			return
		}
		if body, err := io.ReadAll(response.Body); err != nil {
			fmt.Printf("Response body read error: %s\n", err)
		} else {
			fmt.Printf("Response body: %s\n", string(body))
		}

		response.Body.Close()

	}
}
