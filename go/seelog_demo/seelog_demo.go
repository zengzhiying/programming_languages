package main

import "fmt"
import "time"
import log "github.com/cihub/seelog"

func main() {
    defer log.Flush()
    // fmt.Println(time.Now().Format("2006-01-02 15:04:05"))
    logger, err := log.LoggerFromConfigAsFile("seelog.xml")
    // string配置方式
    // logger, _ := log.LoggerFromConfigAsBytes([]byte(testConfig))
    if err != nil {
        fmt.Println("Logger config load error! ", err)
        return
    }
    log.ReplaceLogger(logger)
    // trace
    log.Trace("trace log.")
    log.Tracef("%s", "trace log.")
    // debug
    log.Debug("debug log.")
    log.Debugf("%s", "debug log.")
    // info
    log.Info("info log.")
    log.Infof("%s", "info log.")
    // warning
    log.Warn("warning log.")
    log.Warnf("%s", "warning log.")
    // error
    log.Error("error log.")
    log.Errorf("%s", "error log.")
    // critical 致命错误
    log.Critical("critical log.")
    log.Criticalf("%s", "critical log.")

    fmt.Println("done.")
}
