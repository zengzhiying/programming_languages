package main
import (
    "fmt"
    "io/ioutil"
    "gopkg.in/yaml.v2"
)

type Config struct {
    A string
    B []interface{} `yaml:",flow"`
    C struct {
        UserId int `yaml:"user_id"`
        Age int
        UserName string `yaml:"user_name"`
        Vip bool
        Nick []string `yaml:"nick,flow"`
    }
    // 不存在的项, 默认值
    D int `yaml:"d"`
}

func main() {
    fileName := "demo.yaml"
    contents, err := ioutil.ReadFile(fileName)
    if err != nil {
        fmt.Println("配置文件读取失败！", err)
        return
    }
    // fmt.Printf(string(contents))
    configs := Config{}
    err = yaml.Unmarshal(contents, &configs)
    if err != nil {
        // 错误置默认值, 其余字段正常解析
        fmt.Printf("解释yaml错误: %v\n", err)
    }
    fmt.Printf("configs: %v\n", configs)
    fmt.Printf("A: %s\n", configs.A)
    fmt.Printf("B: %s, %d, %d\n", 
        configs.B[0].(string), configs.B[1].(int), configs.B[2].(int))
    fmt.Printf("C: user_id: %d, age: %d, user_name: %s, vip: %b, nick1: %s\n",
        configs.C.UserId, configs.C.Age, configs.C.UserName, configs.C.Vip,
        configs.C.Nick[0])
}
