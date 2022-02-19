package util

import (
	"github.com/syndtr/goleveldb/leveldb"
)

func GetLevelDB(dbPath string) (*leveldb.DB, error) {
	return leveldb.OpenFile(dbPath, nil)
}
