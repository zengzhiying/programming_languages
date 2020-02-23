package main

import (
	"fmt"
	"os"
	"time"
	"path/filepath"

	"github.com/gxlog/gxlog"
	"github.com/gxlog/gxlog/iface"
	"github.com/gxlog/gxlog/logger"
	"github.com/gxlog/gxlog/writer"
	"github.com/gxlog/gxlog/formatter/text"
	"github.com/gxlog/gxlog/writer/file"
)

var log = gxlog.Logger()

func main() {
	gxlog.Formatter().EnableColoring()
	log.Tracef("log example: %s", "gxlog")

	// 日志级别, 默认最小
	log.SetLevel(iface.Trace)
	
	textFmt := text.New(text.Config{
		Coloring: true,
		Header:   text.CompactHeader,
	})
	log.SetSlotFormatter(logger.Slot0, textFmt)
	log.Trace("green")
	log.Warn("yellow")
	log.Error("red")
	log.WithMark(true).Error("magenta")

	consoleHeader := "{{time}} {{level}} {{msg}}\n"
	fileHeader := "{{time}} {{level}} - {{file:1}}:{{line}} {{pkg}}.{{func}} {{msg}}\n"
	textFmt.SetHeader(consoleHeader)
	log.Trace("Sys content")
	log.Debug("Sys content")
	log.Info("Sys content")
	log.Error("Sys content")
	log.Fatal("Sys content")   // 包含Panic信息

	// file log
	// gxlog.Formatter().SetHeader(header)
	wt, err := file.Open(file.Config{
		Path: filepath.Dir(os.Args[0]) + "/logs",
		// Base: filepath.Base(os.Args[0]).<pid>
		DateStyle: file.DateDash,
		TimeStyle: file.TimeDot,
		MaxFileSize: 1024*1024*1024,
		ErrorHandler: writer.Report,
		// 默认false: <Path>/<Date>/<Base>.<Time>.<Ext>, true: <Path>/<Base>.<Date>.<Time>.<Ext>
		NoDirForDays: false,
	})
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer wt.Close()

	log.CopySlot(logger.Slot1, logger.Slot0)
	log.SetSlotWriter(logger.Slot1, wt)
	fileFmt := text.New(text.Config{
		Coloring: false,
		Header: fileHeader,
	})
	log.SetSlotFormatter(logger.Slot1, fileFmt)
	log.Info("File content")
	log.Warn("File content")
	log.Error("File content")

	// 日志性能统计
	t1 := time.Now().Unix()
	for i := 0; i < 1000000; i++ {
		log.Info("Time test.")
	}
	t2 := time.Now().Unix()
	// 纯日志文件100w/5s, 日志文件+tmux命令行100w/8s, 日志文件+前台命令行20w/5s
	log.Infof("100w条日志耗时: %ds", (t2 - t1))
}
