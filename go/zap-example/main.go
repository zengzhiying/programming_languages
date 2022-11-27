package main

import (
	"encoding/json"
	"os"
	"path/filepath"
	"time"

	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
	"gopkg.in/natefinch/lumberjack.v2"
)

func main() {
	// 基本用法
	logger, err := zap.NewProduction()
	if err != nil {
		panic(err)
	}

	defer logger.Sync()

	sugar := logger.Sugar()
	sugar.Infow("Fields", "url", "http://163.com", "attempt", 3, "time", 2*time.Second)

	sugar.Infof("Log %s", "zap")

	// 日志目录自动创建
	logDir := "./logs"
	if _, err := os.Stat(logDir); err != nil && !os.IsExist(err) {
		if err = os.MkdirAll(logDir, os.ModePerm); err != nil {
			panic(err)
		}
	}

	// 基本配置 也可以基于结构体配置
	logConfJSON := []byte(`{
		"level": "debug",
		"encoding": "json",
		"outputPaths": ["stdout", "./logs/zap.log"],
		"errorOutputPaths": ["stderr"],
		"encoderConfig": {
			"messageKey": "message",
			"levelKey": "level",
			"levelEncoder": "lowercase",
			"timeKey": "time",
			"timeEncoder": "iso8601"
		}
	}`)
	var cfg zap.Config
	if err := json.Unmarshal(logConfJSON, &cfg); err != nil {
		panic(err)
	}

	log := zap.Must(cfg.Build())
	defer log.Sync()

	log.Info("Basic Log info.")
	log.Sugar().Debugf("Basic Log %s", "debugf")
	log.Error("Basic Log error.")

	logPath := filepath.Join(logDir, "lumber.log")

	// 日志分段
	lumberjackLogger := &lumberjack.Logger{
		Filename:   logPath,
		MaxSize:    1, // 单位: MB
		MaxBackups: 3,
		MaxAge:     30, // 删除时间, 单位: 天
		Compress:   false,
	}

	writerSyncer := zapcore.NewMultiWriteSyncer(
		zapcore.AddSync(lumberjackLogger),
		zapcore.AddSync(os.Stdout),
	)
	encoderConfig := zap.NewProductionEncoderConfig()
	encoderConfig.TimeKey = "time"
	encoderConfig.EncodeTime = zapcore.ISO8601TimeEncoder
	// 自定义实现时间格式
	// encoderConfig.EncodeTime = func(t time.Time, pae zapcore.PrimitiveArrayEncoder) {
	// 	pae.AppendString(t.Format("2006-01-02 15:04:05"))
	// }
	encoderConfig.FunctionKey = "func"
	// 级别大写
	encoderConfig.EncodeLevel = zapcore.CapitalLevelEncoder
	logEncoder := zapcore.NewJSONEncoder(encoderConfig)
	// logEncoder := zapcore.NewConsoleEncoder(encoderConfig)
	core := zapcore.NewCore(logEncoder, writerSyncer, zapcore.DebugLevel)
	zapLog := zap.New(core, zap.AddCaller())
	zap.ReplaceGlobals(zapLog)

	// 测试回滚
	for i := 0; i < 10000; i++ {
		zapLog.Sugar().Infof("Log segment for: %d", i)
		zapLog.Sugar().Warnln("File and Console write log.")
	}

	defer zapLog.Sync()
}
