package main

import (
	"os"
	"fmt"
	"time"
	"sync"
	"net/http"

	"github.com/mozillazg/request"
)

var wg sync.WaitGroup

func httpPost() {
	defer wg.Done()
	c := new(http.Client)
	req := request.NewRequest(c)
	// req.Headers = map[string]string{
	// 	"Content-Type": "application/json",
	// }
	req.Data = map[string]string{
		"key": "abcd",
		"value": "371",
	}
	resp, err := req.Post("http://cloud2:9237/post_text")
	if err != nil {
		fmt.Println("http post text error!", err)
		return
	}
	j, err := resp.Json()
	defer resp.Body.Close()
	if err != nil {
		fmt.Println("http post text response error!", err)
		return
	}
	if !resp.OK() {
		fmt.Printf("http post text response code error!\n")
		return
	}
	_, err = j.Get("code").Int()
	if err != nil {
		fmt.Printf("http post text response get content error! %s\n", err)
		return
	}
	// request end
}

func httpFilePost() {
	defer wg.Done()
	c := new(http.Client)
	req := request.NewRequest(c)
	f, err := os.Open("example.jpg")
	if err != nil {
		fmt.Println("file open error!", err)
		return
	}
	defer f.Close()
	req.Files = []request.FileField{
		request.FileField{"image_file", "image.jpg", f},
	}
	resp, err := req.Post("http://cloud2:9237/post_file")
	if err != nil {
		fmt.Println("http post file error!", err)
		return
	}
	defer resp.Body.Close()
	j, err := resp.Json()
	if err != nil {
		fmt.Println("http post file response error!", err)
		return
	}
	size, err := j.Get("size").Int()
	if err != nil {
		fmt.Println("http post file get response content error!", err)
		return
	}
	if size != 52082 {
		fmt.Println("file size error!", size)
	}
	// request end
}

func main() {
	fmt.Println("test start.")
	wg.Add(2000)
	t1 := time.Now().UnixNano()
	for i := 0; i < 1000; i++ {
		go func() {
			httpPost()
			httpPost()
		}()
	}
	wg.Wait()
	t2 := time.Now().UnixNano()
	postTime := float64(t2 - t1) / 1e9
	fmt.Printf("text post time: %.3f\n", postTime)


	wg.Add(2000)
	t1 = time.Now().UnixNano()
	for i := 0; i < 1000; i++ {
		go func() {
			httpFilePost()
			httpFilePost()
		}()
	}
	wg.Wait()
	t2 = time.Now().UnixNano()
	postTime = float64(t2 - t1) / 1e9
	fmt.Printf("file post time: %.3f\n", postTime)

	//0.106 + 0.293 + 0.320 + 0.279 + 0.122 avg: 0.224
	//0.296 + 0.268 + 0.327 + 0.328 + 0.313 avg: 0.306
}

