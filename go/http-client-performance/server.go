package main

import (
	"fmt"
	"io"
	"net/http"
	
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

type RespJson struct {
	Code int `json:"code"`
	Size int `json:"size"`
}

func textHandler(c echo.Context) error {
	key := c.FormValue("key")
	if key == "" {
		fmt.Println("key is empty!")
	}
	r := RespJson{
		Code: 200,
		Size: 0,
	}
	return c.JSON(http.StatusOK, &r)
}

func fileHandler(c echo.Context) error {
	file, err := c.FormFile("image_file")
	if err != nil {
		fmt.Println("Form File error!", err)
		return err
	}
	src, err := file.Open()
	if err != nil {
		fmt.Println("File Open error!", err)
		return err
	}
	defer src.Close()
	buf := make([]byte, 4096)
	size := 0
	for {
		n, err := src.Read(buf)
		size += n
		if err == io.EOF {
			// 文件已经读到结尾
			break
		} else if err != nil {
			fmt.Println("file Read error!", err)
			return err
		}

		if n < 4096 {
			// 文件已经读完 可能是下一次返回EOF
			break
		}
	}
	r := &RespJson{
		Code: 200,
		Size: size,
	}
	return c.JSON(http.StatusOK, r)
}

func main() {
	e := echo.New()
	e.Use(middleware.Logger())
	e.Use(middleware.Recover())
	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello, World!")
	})
	e.POST("/post_text", textHandler)
	e.POST("/post_file", fileHandler)
	e.Logger.Fatal(e.Start("cloud2:9237"))
}

