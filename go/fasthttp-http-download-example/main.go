package main

import (
	"fmt"
	"net/url"
	"os"
	"path"
	"time"

	"github.com/valyala/fasthttp"
)

func main() {
	if len(os.Args) != 2 {
		return
	}

	imageUrl := os.Args[1]
	if imageUrl == "" {
		return
	}

	u, err := url.Parse(imageUrl)
	if err != nil {
		fmt.Printf("image url parse error: %s\n", err)
		return
	}
	fmt.Printf("scheme: %s, host: %s, path: %s, query: %s, %s\n",
		u.Scheme, u.Host, u.EscapedPath(), u.Query(), u.EscapedFragment())

	client := fasthttp.Client{
		ReadTimeout:               5 * time.Second,
		WriteTimeout:              5 * time.Second,
		MaxConnsPerHost:           512,
		MaxIdleConnDuration:       10 * time.Second,
		MaxIdemponentCallAttempts: 1,
		DisablePathNormalizing:    true,
		// Dial:                      fasthttpproxy.FasthttpHTTPDialer("192.168.0.11:7890"),
	}

	fmt.Printf("image url: %s\n", imageUrl)
	req := fasthttp.AcquireRequest()
	resp := fasthttp.AcquireResponse()
	req.Header.DisableNormalizing()
	req.SetRequestURI(imageUrl)
	req.Header.SetMethod(fasthttp.MethodGet)

	defer fasthttp.ReleaseRequest(req)
	defer fasthttp.ReleaseResponse(resp)

	// if err := client.DoTimeout(req, resp, 10*time.Second); err != nil {
	// 	fmt.Printf("download err: %s\n", err)
	// 	return
	// }

	// 自动进行请求重定向
	if err := client.DoRedirects(req, resp, 5); err != nil {
		fmt.Printf("download err: %s\n", err)
		return
	}

	imageBytes := resp.Body()
	fmt.Printf("image size: %d\n", len(imageBytes))

	if len(imageBytes) == 0 {
		return
	}

	imageName := fmt.Sprintf("%d.jpg", time.Now().UnixMilli())
	imagePath := path.Join("/tmp", imageName)
	f, err := os.Create(imagePath)
	if err != nil {
		fmt.Printf("Create file: %s err: %s\n", imagePath, err)
		return
	}
	defer f.Close()
	if _, err := f.Write(imageBytes); err != nil {
		fmt.Printf("Write file: %s, err: %s\n", imagePath, err)
		return
	}

	fmt.Printf("Saved file: %s\n", imagePath)
}
