package main

import (
    "context"
    "flag"
    "log"
    "time"

    "github.com/smallnest/rpcx/protocol"
    "github.com/smallnest/rpcx/client"
)

type Args struct {
    A int
    B int
}

type Reply struct {
    C int
}

var (
    addr = flag.String("addr", "localhost:8972", "server address")
)

func main() {
    flag.Parse()

    d := client.NewPeer2PeerDiscovery("tcp@"+*addr, "")
    opt := client.DefaultOption
    opt.SerializeType = protocol.JSON
    // 超时设置
    // opt.ReadTimeout = 10 * time.Second

    xclient := client.NewXClient("Arith", client.Failtry, client.RandomSelect, d, opt)
    defer xclient.Close()

    args := Args{
        A: 10,
        B: 20,
    }

    reply := &Reply{}
    err := xclient.Call(context.Background(), "Mul", args, reply)
    if err != nil {
        log.Fatalf("failed to call: %v", err)
    }

    log.Printf("%d * %d = %d", args.A, args.B, reply.C)

    // 并发操作
    for i := 0; i < 32; i++ {
        go xclientCall(args, xclient, i)
    }
    time.Sleep(4 * time.Second)
}

func xclientCall(args Args, xclient client.XClient, num int) {
    reply := new(Reply)
    err := xclient.Call(context.Background(), "Mul", args, reply)
    if err != nil {
        log.Fatalf("failed to call: %v", err)
    }
    log.Printf("%d: %d * %d = %d", num, args.A, args.B, reply.C)
}
