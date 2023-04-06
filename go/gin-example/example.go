package main

import (
	"os"
	"bytes"
	"fmt"
	"io"
	"mime/multipart"

	"github.com/gin-gonic/gin"
)

func CustomMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		if c.PostForm("name") != "zzy" {
			// 保证此次请求结束必须执行Abort
			// 否则还是会进入最后的处理
			c.Abort()
			// c.AbortWithStatus(401)
			return
		}

		defer fmt.Println("return.")

		// 设置一些上下文信息
		c.Set("custom", "ok")

		// 进入具体的处理 如果这行代码不加会在当前函数完全执行完再自动进入处理
		// 注意如果显示执行c.Next()处理完仍然会返回到当前中间件执行
		c.Next()

		fmt.Printf("CustomMiddleware ")
	}
}

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

	// 多文件上传
	r.MaxMultipartMemory = 8 << 20 // 8 MiB

	r.POST("/multiple-upload", func(c *gin.Context) {
		form, err := c.MultipartForm()
		if err != nil {
			fmt.Println(err)
			c.JSON(400, gin.H{"err": err})
			return
		}
		formFiles := form.File
		for key, files := range formFiles {
			fmt.Printf("key: %s number of files: %d\n", key, len(files))
			for _, f := range files {
				fmt.Printf("file: %s size: %d\n", f.Filename, f.Size)
			}
		}
		c.JSON(201, gin.H{"message": "ok"})
	})

	// b := []byte("this is text.")
	// fileHeader, err := MultipartFormHeaderFromBytes(b)
	// if err != nil {
	// 	panic(err)
	// }
	// f, err := fileHeader.Open()
	// if err != nil {
	// 	panic(err)
	// }
	// d, err := io.ReadAll(f)
	// if err != nil {
	// 	panic(err)
	// }
	// fmt.Println(string(d))


	// 中间件
	// curl -XPOST 'localhost:8080/cm' -d 'name=zzy'
	r.POST("/cm", CustomMiddleware(), func(c *gin.Context) {
		fmt.Println("/cm handler.")
		fmt.Println(c.Keys)
		c.String(200, "Custom Middleware!")
	})
	// 中间件分组
	middleGroup := r.Group("/")
	middleGroup.Use(CustomMiddleware())
	{
		middleGroup.POST("/group-cm1", func(c *gin.Context) {
			fmt.Println("/group-cm1 handler")
			c.String(200, "Group Middleware1!")
			return
		})

		middleGroup.POST("/group-cm2", func(c *gin.Context) {
			fmt.Println("/group-cm2 handler")
			if c.PostForm("id") != "" {
				c.String(200, "Group Middleware2, id: " + c.PostForm("id"))
				return
			}
			c.String(200, "Group Middleware2!")
		})
	}

	r.Run(":8080") // listen and serve on 0.0.0.0:8080 (for windows "localhost:8080")
}

// 包装字节切片为 multipart.FileHeader 对象
func MultipartFormHeaderFromBytes(src []byte) (*multipart.FileHeader, error) {
	var buf bytes.Buffer
	writer := multipart.NewWriter(&buf)
	fw, err := writer.CreateFormFile("buffered_form", "buffered_file")
	if err != nil {
		return nil, err
	}
	_, err = io.Copy(fw, bytes.NewReader(src))
	if err != nil {
		return nil, err
	}
	writer.Close()

	reader := multipart.NewReader(bytes.NewReader(buf.Bytes()), writer.Boundary())
	fm, err := reader.ReadForm(128 * 1024 * 1024)
	if err != nil {
		return nil, err
	}
	fileHeaders := fm.File["buffered_form"]
	if fileHeaders != nil {
		return fileHeaders[0], nil
	}
	return nil, fmt.Errorf("file headers is nil")
}
