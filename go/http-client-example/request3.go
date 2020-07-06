package main

import (
	// "os"
	"fmt"
	"time"
	"sync"
	"bytes"
	"io/ioutil"
	"encoding/json"

	"github.com/go-resty/resty/v2"
)



type RespJson struct {
	Code int `json:"code"`
	Size int `json:"size"`
}

var wg sync.WaitGroup

func httpPost() {
	defer wg.Done()
	respJson := &RespJson{}
	client := resty.New()
	resp, err := client.R().
		SetFormData(map[string]string{
			"key": "abcd",
			"value": "371",
		}).
		// setheader && setbody => setformdata
		// SetHeader("Content-Type", "application/x-www-form-urlencoded").
		// SetBody("key=abcd&value=371").
		// 如果返回是json or xml并且code在200~299之间, 就可以按照结构体直接解析 不用从resp再解析了
		SetResult(respJson).
		Post("http://cloud2:9237/post_text")
	if err != nil {
		fmt.Println("http post text error!", err)
		return
	}
	if resp.StatusCode() != 200 {
		fmt.Printf("http post text response code error! %d\n", resp.StatusCode())
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
	imageBytes, err := ioutil.ReadFile("./example.jpg")
	if err != nil {
		fmt.Println("file read error!", err)
		return
	}
	client := resty.New()
	// f, err := os.Open("./example.jpg")
	// if err != nil {
	// 	fmt.Println("file open error!", err)
	// 	return
	// }
	// defer f.Close()
	resp, err := client.R().
		// SetFile("image_file", "./example.jpg").
		// SetFileReader("image_file", "image.jpg", f).
		SetFileReader("image_file", "image.jpg", bytes.NewReader(imageBytes)).
		SetContentLength(true).
		Post("http://cloud2:9237/post_file")
	
	if err != nil {
		fmt.Println("http post file error!", err)
		return
	}
	if resp.StatusCode() != 200 {
		fmt.Printf("http post file response code error! %d\n", resp.StatusCode())
		return
	}
	respJson := &RespJson{}
	err = json.Unmarshal(resp.Body(), respJson)
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

	//0.300 + 0.231 + 0.276 + 0.323 + 0.240 avg: 0.274
	//1.053 + 0.367 + 0.267 + 1.073 + 1.285 avg: 0.809
}


