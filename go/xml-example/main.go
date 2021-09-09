package main

import (
	"bytes"
	"fmt"
	"io/ioutil"

	"github.com/beevik/etree"
	"golang.org/x/text/encoding/simplifiedchinese"
	"golang.org/x/text/transform"
)

func UTF8ToGBK(v []byte) ([]byte, error) {
	r := transform.NewReader(bytes.NewReader(v), simplifiedchinese.GBK.NewEncoder())
	return ioutil.ReadAll(r)
}

func GBKToUTF8(v []byte) ([]byte, error) {
	r := transform.NewReader(bytes.NewReader(v), simplifiedchinese.GBK.NewDecoder())
	return ioutil.ReadAll(r)
}

func main() {
	// 生成xml
	doc := etree.NewDocument()
	doc.CreateProcInst("xml", `version="1.0" encoding="utf-8"`)
	person := doc.CreateElement("Person")
	person.CreateComment("This is Person Info.")
	id := person.CreateElement("IdNumber")
	id.CreateAttr("id", "372928")

	name := person.CreateElement("Name")
	name.CreateAttr("name", "zzy")
	name.CreateComment("This is the nickname.")
	nickName := name.CreateElement("NickName")
	nickName.CreateAttr("nick", "monchickey")

	age := person.CreateElement("Age")
	age.SetText("23")

	doc.Indent(2)
	out, err := doc.WriteToBytes()
	if err != nil {
		fmt.Println(err)
	}
	fmt.Print(string(out))

	// 解析xml
	xmlContent := `<?xml version="1.0" encoding="GB2312"?>
<response code="200">
  <protocol>HTTP</protocol>
  <number>8837891</number>
  <status>OK</status>
  <name>小明</name>
  <content>我是小明</content>
  <hobbys>
    <hobby id="1">乒乓球</hobby>
	<hobby id="2">跑步</hobby>
	<hobby id="3">追剧</item>
  </hobbys>
</response>`

	out, err = UTF8ToGBK([]byte(xmlContent))
	if err != nil {
		panic(err)
	}

	utf8Content, err := GBKToUTF8(out)
	if err != nil {
		panic(err)
	}

	fmt.Println()

	doc = etree.NewDocument()
	err = doc.ReadFromBytes(utf8Content)
	if err != nil {
		panic(err)
	}

	response := doc.SelectElement("response")
	if attr := response.SelectAttr("code"); attr != nil {
		fmt.Println(response.Tag, attr.Value)
	}

	proto := response.SelectElement("protocol")
	fmt.Println("proto", proto.Text())

	name = response.SelectElement("name")
	fmt.Println("name:", name.Text())

	content := response.SelectElement("content")
	fmt.Println("content:", content.Text())

	hobbys := response.SelectElement("hobbys")
	if hobbys == nil {
		panic("error")
	}

	for _, hobby := range hobbys.SelectElements("hobby") {
		fmt.Printf("兴趣%s: ", hobby.SelectAttrValue("id", "null"))
		fmt.Println(hobby.Text())
	}

}
