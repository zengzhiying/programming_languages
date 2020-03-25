package main

import "os"
import "io"
import "fmt"
// import "net/http"
import "github.com/gin-gonic/gin"

func main() {
	r := gin.Default()
	r.GET("/ping", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"message": "pong",
		})
	})
    // 返回图片流
	r.GET("/image", func(c *gin.Context) {
		fp, err := os.Open("./example.jpg")
		if err != nil {
			fmt.Println("文件打开错误: ", err)
			return
		}
		defer fp.Close()
		buff := make([]byte, 1024)
		fileBody := make([]byte, 0)
		for {
			length, err := fp.Read(buff)
			if err == io.EOF || length == 0 {
				break
			}
			fileBody = append(fileBody, buff[:length]...)
		}
		c.Data(200, "image/jpg", fileBody)
	})
	
    // 读取本地文件返回图片流
	r.GET("/photo", func(c *gin.Context) {
		c.File("./example.jpg")
	})
	r.Run(":8080") // listen and serve on 0.0.0.0:8080 (for windows "localhost:8080")
}