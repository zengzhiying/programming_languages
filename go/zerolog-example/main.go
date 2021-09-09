package main

import (
	"os"
	"fmt"
	"time"
	"errors"
	"path/filepath"

	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

func PathIsExist(path string) bool {
	_, err := os.Stat(path)
	return err == nil || os.IsExist(err)
}

func TimestampToString(timestamp int64, timeFormat string) string {
	unixTime := time.Unix(timestamp, 0)
	return unixTime.Format(timeFormat)
}

func main() {
	here := filepath.Dir(os.Args[0])

	zerolog.TimeFieldFormat = "2006-01-02 15:04:05"
	zerolog.SetGlobalLevel(zerolog.InfoLevel)

	log.Debug().Msg("Debug output.")
	log.Info().Msg("Info output.")

	// str
	log.Log().
		Str("foo", "bar").
		Msg("")

	err := errors.New("This is error!")
	log.Error().Err(err).Msg("error!")

	// Fatal会终止程序
	// log.Fatal().Msgf("stop: %s", "program")

	// console write
	consoleWriter := zerolog.ConsoleWriter{Out: os.Stderr}
	log.Logger = log.Output(consoleWriter)
	log.Info().Msg("console write!")

	// 采样输出 日志大的时候比较好用
	sampled := log.Sample(&zerolog.BasicSampler{N: 10})
	sampled.Info().Msg("1.will be logged every 10 messages.")
	sampled.Info().Msg("2.will be logged every 10 messages.")

	// console and file
	logPath := here + "/logs"
	if !PathIsExist(logPath) {
		err = os.Mkdir(logPath, 0755)
		if err != nil {
			log.Fatal().Err(err).Msg("")
		}
	}
	f, err := os.OpenFile("logs/output.log", os.O_RDWR|os.O_CREATE|os.O_APPEND, 0644)
	if err != nil {
		log.Fatal().Err(err).Msg("")
	}
	defer f.Close()
	multi := zerolog.MultiLevelWriter(consoleWriter, f)

	logger := zerolog.New(multi).With().Timestamp().Logger()
	log.Logger = logger
	log.Info().Msgf("Multi info logger!")
	log.Debug().Msgf("Multi debug logger!")
	time.Sleep(time.Second)
	log.Info().Msgf("Multi info logger!")


	time.Sleep(10 * time.Second)


	// 性能测试
	var t1, t2 int64

	log.Logger = log.Output(os.Stdout)
	t1 = time.Now().UnixNano()
	for i := 0; i < 100000; i++ {
		log.Info().Msgf("Test msg is: %d", i)
	}
	t2 = time.Now().UnixNano()
	tm1 := float64(t2 - t1) / 1e6
	

	log.Logger = log.Output(consoleWriter)
	t1 = time.Now().UnixNano()
	for i := 0; i < 100000; i++ {
		log.Info().Msgf("Test msg is: %d", i)
	}
	t2 = time.Now().UnixNano()
	tm2 := float64(t2 - t1) / 1e6
	


	log.Logger = log.Output(f)
	t1 = time.Now().UnixNano()
	for i := 0; i < 100000; i++ {
		log.Info().Msgf("Test msg is: %d", i)
	}
	t2 = time.Now().UnixNano()
	tm3 := float64(t2 - t1) / 1e6
	

	log.Logger = logger
	t1 = time.Now().UnixNano()
	for i := 0; i < 100000; i++ {
		log.Info().Msgf("Test msg is: %d", i)
	}
	t2 = time.Now().UnixNano()
	tm4 := float64(t2 - t1) / 1e6
	

	// default
	log.Logger = zerolog.New(os.Stderr).With().Timestamp().Logger()
	t1 = time.Now().UnixNano()
	for i := 0; i < 100000; i++ {
		log.Info().Msgf("Test msg is: %d", i)
	}
	t2 = time.Now().UnixNano()
	tm5 := float64(t2 - t1) / 1e6


	// stdout / default: 280ms/10w
	// consolewrite:  860ms/10w
	// file: 240ms/10w
	// multi: 1160ms/10w
	log.Info().Msgf("stdout time: %fms", tm1)
	log.Info().Msgf("consoleWriter time: %fms", tm2)
	log.Info().Msgf("file time: %fms", tm3)
	log.Info().Msgf("Multi time: %fms", tm4)
	log.Info().Msgf("default time: %fms", tm5)


	// 日志分段测试
	go logSegment()
	time.Sleep(time.Second)
	for i := 0; i < 100000000; i++ {
		log.Info().Msgf("log -> %d", i)
	}
}

func logSegment() {
	curDate := TimestampToString(time.Now().Unix(), "20060102150405")
	logFile := fmt.Sprintf("logs/segment-%s.log", curDate)
	f, err := os.OpenFile(logFile, os.O_RDWR|os.O_CREATE|os.O_APPEND, 0644)
	if err != nil {
		panic(err)
	}
	log.Logger = log.Output(f)
	for {
		time.Sleep(10 * time.Second)
		tf := f
		curDate := TimestampToString(time.Now().Unix(), "20060102150405")
		logFile := fmt.Sprintf("logs/segment-%s.log", curDate)
		f, err = os.OpenFile(logFile, os.O_RDWR|os.O_CREATE|os.O_APPEND, 0644)
		if err != nil {
			fmt.Println(err)
		} else {
			log.Logger = log.Output(f)
			// 等待其他线程更新变量 否则需要加锁，其他线程可能在关闭文件后还在写入导致少量日志丢失
			time.Sleep(time.Second)
			tf.Close()
		}
	}
}
