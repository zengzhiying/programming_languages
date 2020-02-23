package main

import (
	"fmt"
	"encoding/json"

	"github.com/json-iterator/go"
)

type Row struct {
	Id  int  `json:"id"`
	Name  string   `json:"name"`
	Colors  []int `json:"colors"`
	Age  uint8   `json:"age"`
	NickName  string `json:"nick_name"`
}

func main() {
	r := Row{
		Id: 1,
		Name: "小明",
		Colors: []int{2, 3, 6},
		Age: 23,
		NickName: "James",
	}
	fmt.Println(r)
	b, err := json.Marshal(r)
	if err != nil {
		fmt.Println("err:", err)
	}
	fmt.Println(string(b))
	b, err = jsoniter.Marshal(r)
	fmt.Println(string(b))

	var r1 Row
	err = json.Unmarshal(b, &r1)
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println(r1)


	var m map[string]interface{}
	err = json.Unmarshal(b, &m)
	fmt.Println(m)
	fmt.Println(jsoniter.Wrap(m["nick_name"]).ToString())
	fmt.Println(jsoniter.Wrap(m["age"]).ToInt())
	fmt.Println(jsoniter.Wrap(m["other"]).ToString(), jsoniter.Wrap(m["other"]).ToInt())
}
