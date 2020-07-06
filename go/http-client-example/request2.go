package main

import (
	"os"
	"fmt"
	"time"
	"sync"
	// "io/ioutil"
	"encoding/json"

	"github.com/parnurzeal/gorequest"
)



type RespJson struct {
	Code int `json:"code"`
	Size int `json:"size"`
}

var wg sync.WaitGroup

func httpPost() {
	defer wg.Done()
	request := gorequest.New()
	resp, body, errs := request.Post("http://cloud2:9237/post_text").
		// Query("key=abcd").
		// Query("value=371&a=6").
		Send("key=abcd&value=371").
		End()
	if errs != nil {
		fmt.Println("http post text error!", errs)
		return
	}
	if resp.StatusCode != 200 {
		fmt.Printf("http post text response code error! %d\n", resp.StatusCode)
		return
	}
	respJson := &RespJson{}
	err := json.Unmarshal([]byte(body), respJson)
	if err != nil {
		fmt.Println("http post text json decode error!", err)
		return
	}
	if respJson.Code != 200 {
		fmt.Println("http post text json code error!", respJson.Code)
	}
	// request end
}

func httpFilePost() {
	defer wg.Done()
	// 读取字节流性能损耗略大
	// imageBytes, err := ioutil.ReadFile("./example.jpg")
	request := gorequest.New()
	f, err := os.Open("./example.jpg")
	resp, body, errs := request.Post("http://cloud2:9237/post_file").
		Type("multipart").
		// SendFile(imageBytes, "image.jpg", "image_file").
		// SendFile("./example.jpg", "image.jpg", "image_file").
		SendFile(f, "image.jpg", "image_file").
		EndBytes()
	defer f.Close()
	if errs != nil {
		fmt.Println("http post file error!", errs)
		return
	}
	if resp.StatusCode != 200 {
		fmt.Printf("http post file response code error! %d\n", resp.StatusCode)
		return
	}
	respJson := &RespJson{}
	err = json.Unmarshal(body, respJson)
	if err != nil {
		fmt.Println("http post file json decode error!", err)
		return
	}
	if respJson.Size != 52082 {
		fmt.Println("file size error!", respJson.Size)
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

	// somaxconn = 128 - connection reset by peer
	//1.116 + 3.142 + 1.144 + 3.189 + 1.157 avg: 1.95
	//3.194 + 3.149 + 7.161 + 3.144 + 3.150 avg: 3.96
	// somaxconn = 1024 没有错误
	// 0.123
	// 0.139
}


