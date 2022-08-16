package main

import (
	"fmt"
	"reflect"
)

type any = interface{}

type Rows struct {
	Names   []string
	Ages    []int
	Genders []uint8
	Numbers []uint64
	Notes   []string
	Count   int
	Meta    []byte
}

func checkElemLenght(v any, excludeFields []string) bool {
	// Exclude fields
	fields := make(map[string]any, len(excludeFields))
	for _, field := range excludeFields {
		fields[field] = nil
	}

	kv := reflect.ValueOf(v)
	tv := reflect.TypeOf(v)

	if kv.Kind() != reflect.Struct && kv.Kind() != reflect.Ptr {
		return false
	}

	if kv.Kind() == reflect.Ptr {
		if kv.IsNil() {
			return false
		}
		if kv.Elem().Kind() != reflect.Struct {
			return false
		}

		kv = kv.Elem()
		tv = tv.Elem()
	}

	elemLen := -1

	for i := 0; i < kv.NumField(); i++ {
		if _, ok := fields[tv.Field(i).Name]; ok {
			continue
		}

		if kv.Field(i).Kind() != reflect.Slice && kv.Field(i).Kind() != reflect.Array {
			continue
		}

		l := kv.Field(i).Len()

		if elemLen == -1 {
			elemLen = l
			continue
		}

		if elemLen != l {
			return false
		}
	}

	return true
}

func main() {

	rows := Rows{
		Names:   []string{"hello", "world"},
		Ages:    []int{23, 26},
		Genders: []uint8{1, 2},
		Numbers: []uint64{889, 182},
		Notes:   []string{"", ""},
		Count:   2,
	}

	fmt.Println(checkElemLenght(&rows, []string{"Meta"}))

	a := 3
	fmt.Println(checkElemLenght(&a, nil))

}
