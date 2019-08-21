
package main

import (
	"fmt"
	"net"
)


func handleConn(conn net.Conn) {
	if conn == nil {
		return
	}
	//读取所有数据
	bytes := make([]byte, 0)
	buf := make([]byte, 4096)

	for {
		cnt, err := conn.Read(buf)
		// fmt.Printf("%d %q\n", cnt, buf)
		bytes = append(bytes, buf[:cnt]...)
		if err != nil || cnt < len(buf) || string(buf[cnt - 4:cnt]) == "\r\n\r\n" {
			conn.Write([]byte("reading. \n"))
			conn.Close()
			//if err != nil {
			//	fmt.Println("Connection close error!")
			//}
			fmt.Println("Connection has closed!")
			break
		}
	}
	// fmt.Printf("%q", bytes)
	if len(bytes) == 0 {
		return
	}
    fmt.Printf("Request content: %s\n", string(bytes))
}


func main()  {

	fmt.Println("Http Server started!")

	server, err := net.Listen("tcp", "0.0.0.0:9090")
	if err != nil {
		panic(err)
	}

	defer server.Close()

	for {
		conn, err := server.Accept()
		// fmt.Println("accept.")
		if err != nil {
			continue
		}
		go handleConn(conn)
	}


}
