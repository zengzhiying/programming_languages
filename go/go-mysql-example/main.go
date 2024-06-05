package main

import (
	"database/sql"
	"fmt"
	"reflect"
	"time"

	_ "github.com/go-sql-driver/mysql"
)

func main() {
	db, err := sql.Open("mysql", "root:123456@tcp(192.168.128.80:9030)/test")
	if err != nil {
		fmt.Println(err)
		return
	}
	db.SetConnMaxLifetime(5 * time.Minute)
	db.SetMaxOpenConns(10)
	db.SetMaxIdleConns(10)

	defer db.Close()

	rows, err := db.Query("SELECT * FROM test_cosine")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer rows.Close()

	columns, err := rows.Columns()
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println(columns)
	columnTypes, err := rows.ColumnTypes()
	if err != nil {
		fmt.Println(err)
		return
	}

	for rows.Next() {
		results := make([]interface{}, len(columns))
		for i, columnType := range columnTypes {

			results[i] = reflect.New(columnType.ScanType()).Interface()

			if columnType.DatabaseTypeName() == "DATETIME" || columnType.DatabaseTypeName() == "DATE" {
				var v []uint8
				results[i] = reflect.New(reflect.TypeOf(v)).Interface()
			}

			fmt.Printf("column: %s, type: %s, value type: %v\n",
				columnType.Name(), columnType.ScanType(), columnType.DatabaseTypeName())
		}

		err = rows.Scan(results...)
		if err != nil {
			fmt.Println(err)
		} else {
			fmt.Printf("id: %d, data: %s, name: %s, insert_time: %v\n",
				results[0].(*sql.NullInt64).Int64, results[1].(*sql.NullString).String, results[2].(*sql.NullString).String,
				string(*results[3].(*[]uint8)),
			)
		}
	}
}
