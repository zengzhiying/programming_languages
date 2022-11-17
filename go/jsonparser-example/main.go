package main

import (
	"encoding/json"
	"fmt"

	"github.com/buger/jsonparser"
)

func main() {
	value := "\\v"
	m := map[string]string{
		"name":  "中国1",
		"value": value,
	}
	fmt.Println(string(value), len(value))
	s, err := json.Marshal(m)
	if err != nil {
		panic(err)
	}

	fmt.Println(string(s))

	m = map[string]string{}
	err = json.Unmarshal(s, &m)
	if err != nil {
		panic(err)
	}

	fmt.Println(m["value"], len(m["value"]))

	// 带转义字符不要用Get https://github.com/buger/jsonparser/issues/91
	v, err := jsonparser.GetString(s, "value")
	if err != nil {
		panic(err)
	}

	// fmt.Printf("%s, %s, %d\n", v, t, o)
	fmt.Println(v)

	jsonparser.EachKey(s, func(i int, b []byte, vt jsonparser.ValueType, err error) {
		switch i {
		case 0:
			fmt.Println(string(b), len(string(b)))
		case 1:
			b, _ := jsonparser.ParseString(b)
			fmt.Println(b, len(string(b)))
		default:
			fmt.Println("other")
		}
	}, [][]string{{"name"}, {"value"}}...)

}
