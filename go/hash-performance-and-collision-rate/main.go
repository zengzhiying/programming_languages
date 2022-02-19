package main

import (
	"crypto/rand"
	"encoding/binary"
	"encoding/hex"
	"fmt"
	"hash-performance-and-collision-rate/util"
	"io"
	"time"
)

func executorPerformance() {
	var reader = rand.Reader
	var t1, t2 int64
	var deltaT, through float64
	num := 100000000
	// src := make([]byte, 16)
	t1 = time.Now().UnixNano()
	randers := make([][]byte, num)
	for i := 0; i < num; i++ {
		// binary.BigEndian.PutUint64(src, uint64(i))
		b := make([]byte, 16)
		io.ReadFull(reader, b)
		randers[i] = b
	}
	t2 = time.Now().UnixNano()
	putTime := float64(t2-t1) / 1e9
	fmt.Printf("Basic time: %fs\n", putTime)

	t1 = time.Now().UnixNano()
	for i := 0; i < num; i++ {
		// binary.BigEndian.PutUint64(src, uint64(i))
		// io.ReadFull(reader, src)
		util.CRC32(randers[i])
	}
	t2 = time.Now().UnixNano()
	deltaT = float64(t2-t1) / 1e9
	// deltaT -= putTime
	through = float64(num) / deltaT / 1000000
	fmt.Printf("CRC32 time: %fs, throughput: %fM/s\n", deltaT, through)

	t1 = time.Now().UnixNano()
	for i := 0; i < num; i++ {
		// binary.BigEndian.PutUint64(src, uint64(i))
		// io.ReadFull(reader, src)
		util.CRC64(randers[i])
	}
	t2 = time.Now().UnixNano()
	deltaT = float64(t2-t1) / 1e9
	// deltaT -= putTime
	through = float64(num) / deltaT / 1000000
	fmt.Printf("CRC64 time: %fs, throughput: %fM/s\n", deltaT, through)

	t1 = time.Now().UnixNano()
	for i := 0; i < num; i++ {
		// binary.BigEndian.PutUint64(src, uint64(i))
		// io.ReadFull(reader, src)
		util.MD5(randers[i])
	}
	t2 = time.Now().UnixNano()
	deltaT = float64(t2-t1) / 1e9
	// deltaT -= putTime
	through = float64(num) / deltaT / 1000000
	fmt.Printf("MD5 time: %fs, throughput: %fM/s\n", deltaT, through)

	t1 = time.Now().UnixNano()
	for i := 0; i < num; i++ {
		// binary.BigEndian.PutUint64(src, uint64(i))
		// io.ReadFull(reader, src)
		util.SHA1(randers[i])
	}
	t2 = time.Now().UnixNano()
	deltaT = float64(t2-t1) / 1e9
	// deltaT -= putTime
	through = float64(num) / deltaT / 1000000
	fmt.Printf("SHA1 time: %fs, throughput: %fM/s\n", deltaT, through)

	t1 = time.Now().UnixNano()
	for i := 0; i < num; i++ {
		// binary.BigEndian.PutUint64(src, uint64(i))
		// io.ReadFull(reader, src)
		util.FNV_1a64(randers[i])
	}
	t2 = time.Now().UnixNano()
	deltaT = float64(t2-t1) / 1e9
	// deltaT -= putTime
	through = float64(num) / deltaT / 1000000
	fmt.Printf("FNV-1a time: %fs, throughput: %fM/s\n", deltaT, through)

	t1 = time.Now().UnixNano()
	for i := 0; i < num; i++ {
		// binary.BigEndian.PutUint64(src, uint64(i))
		// io.ReadFull(reader, src)
		util.DJB2(randers[i])
	}
	t2 = time.Now().UnixNano()
	deltaT = float64(t2-t1) / 1e9
	// deltaT -= putTime
	through = float64(num) / deltaT / 1000000
	fmt.Printf("DJB2 time: %fs, throughput: %fM/s\n", deltaT, through)

	t1 = time.Now().UnixNano()
	for i := 0; i < num; i++ {
		// binary.BigEndian.PutUint64(src, uint64(i))
		// io.ReadFull(reader, src)
		util.SDBM(randers[i])
	}
	t2 = time.Now().UnixNano()
	deltaT = float64(t2-t1) / 1e9
	// deltaT -= putTime
	through = float64(num) / deltaT / 1000000
	fmt.Printf("SDBM time: %fs, throughput: %fM/s\n", deltaT, through)

	t1 = time.Now().UnixNano()
	for i := 0; i < num; i++ {
		// binary.BigEndian.PutUint64(src, uint64(i))
		// io.ReadFull(reader, src)
		util.SipHash64(randers[i])
	}
	t2 = time.Now().UnixNano()
	deltaT = float64(t2-t1) / 1e9
	// deltaT -= putTime
	through = float64(num) / deltaT / 1000000
	fmt.Printf("SipHash64 time: %fs, throughput: %fM/s\n", deltaT, through)

	t1 = time.Now().UnixNano()
	for i := 0; i < num; i++ {
		// binary.BigEndian.PutUint64(src, uint64(i))
		// io.ReadFull(reader, src)
		util.XXHash64(randers[i])
	}
	t2 = time.Now().UnixNano()
	deltaT = float64(t2-t1) / 1e9
	// deltaT -= putTime
	through = float64(num) / deltaT / 1000000
	fmt.Printf("XXHash64 time: %fs, throughput: %fM/s\n", deltaT, through)
}

func executorCollisionRate() {
	num := 2500000000
	// src := make([]byte, 16)
	var m map[uint64]*[]byte
	var collisionNum int
	var collisionRate float64
	var reader io.Reader = rand.Reader

	// 初始化 随机数生成性能太低
	randers := make([][]byte, num)
	for i := 0; i < num; i++ {
		b := make([]byte, 16)
		io.ReadFull(reader, b)
		randers[i] = b
	}

	// --------------- CRC64 ------------------------
	// 扩容因子默认6.5 槽数缩减5倍同时值采用指针来节省内存空间 
	m = make(map[uint64]*[]byte, num/5)
	collisionNum = 0
	for i := 0; i < num; i++ {
		// binary.LittleEndian.PutUint32(src, uint32(i))
		// io.ReadFull(reader, src)
		h := util.CRC64(randers[i])
		if b1, ok := m[h]; ok {
			collisionNum++
			fmt.Printf("key1: %s, key2: %s", hex.EncodeToString(*b1), hex.EncodeToString(randers[i]))
		} else {
			m[h] = &randers[i]
		}

		if (i+1)%10000000 == 0 {
			fmt.Printf("CRC64 num: %d collisions: %d\n", i+1, collisionNum)
		}
	}
	collisionRate = float64(collisionNum) / float64(num)

	fmt.Printf("CRC64 collisions: %d, collision rate: %f\n", collisionNum, collisionRate)

	// --------------- MD5 8byte ------------------------
	m = make(map[uint64]*[]byte, num/5)
	collisionNum = 0
	for i := 0; i < num; i++ {
		// binary.LittleEndian.PutUint32(src, uint32(i))
		// io.ReadFull(reader, src)
		b := util.MD5(randers[i])[:8]
		h := binary.BigEndian.Uint64(b)
		if b1, ok := m[h]; ok {
			collisionNum++
			fmt.Printf("key1: %s, key2: %s", hex.EncodeToString(*b1), hex.EncodeToString(randers[i]))
		} else {
			m[h] = &randers[i]
		}

		if (i+1)%10000000 == 0 {
			fmt.Printf("MD5 num: %d collisions: %d\n", i+1, collisionNum)
		}
	}
	collisionRate = float64(collisionNum) / float64(num)

	fmt.Printf("MD5 collisions: %d, collision rate: %f\n", collisionNum, collisionRate)

	// --------------- SHA1 8byte ------------------------
	m = make(map[uint64]*[]byte, num/5)
	collisionNum = 0
	for i := 0; i < num; i++ {
		// io.ReadFull(reader, src)
		b := util.SHA1(randers[i])[:8]
		h := binary.BigEndian.Uint64(b)
		if b1, ok := m[h]; ok {
			collisionNum++
			fmt.Printf("key1: %s, key2: %s", hex.EncodeToString(*b1), hex.EncodeToString(randers[i]))
		} else {
			m[h] = &randers[i]
		}

		if (i+1)%10000000 == 0 {
			fmt.Printf("SHA1 num: %d collisions: %d\n", i+1, collisionNum)
		}
	}
	collisionRate = float64(collisionNum) / float64(num)

	fmt.Printf("SHA1 collisions: %d, collision rate: %f\n", collisionNum, collisionRate)

	// --------------- FNV-1a ------------------------
	m = make(map[uint64]*[]byte, num/5)
	collisionNum = 0
	for i := 0; i < num; i++ {
		// io.ReadFull(reader, src)
		h, err := util.FNV_1a64(randers[i])
		if err != nil {
			fmt.Println("FNV-1a Test error!", err)
			break
		}
		if b1, ok := m[h]; ok {
			collisionNum++
			fmt.Printf("key1: %s, key2: %s", hex.EncodeToString(*b1), hex.EncodeToString(randers[i]))
		} else {
			m[h] = &randers[i]
		}

		if (i+1)%10000000 == 0 {
			fmt.Printf("FNV-1a num: %d collisions: %d\n", i+1, collisionNum)
		}
	}
	collisionRate = float64(collisionNum) / float64(num)

	fmt.Printf("FNV-1a collisions: %d, collision rate: %f\n", collisionNum, collisionRate)

	// --------------- DJB2 ------------------------
	m = make(map[uint64]*[]byte, num/5)
	collisionNum = 0
	for i := 0; i < num; i++ {
		// io.ReadFull(reader, src)
		h := util.DJB2(randers[i])
		if b1, ok := m[h]; ok {
			collisionNum++
			fmt.Printf("key1: %s, key2: %s", hex.EncodeToString(*b1), hex.EncodeToString(randers[i]))
		} else {
			m[h] = &randers[i]
		}

		if (i+1)%10000000 == 0 {
			fmt.Printf("DJB2 num: %d collisions: %d\n", i+1, collisionNum)
		}
	}
	collisionRate = float64(collisionNum) / float64(num)

	fmt.Printf("DJB2 collisions: %d, collision rate: %f\n", collisionNum, collisionRate)

	// --------------- SDBM ------------------------
	m = make(map[uint64]*[]byte, num/5)
	collisionNum = 0
	for i := 0; i < num; i++ {
		// io.ReadFull(reader, src)
		h := util.SDBM(randers[i])
		if b1, ok := m[h]; ok {
			collisionNum++
			fmt.Printf("key1: %s, key2: %s", hex.EncodeToString(*b1), hex.EncodeToString(randers[i]))
		} else {
			m[h] = &randers[i]
		}

		if (i+1)%10000000 == 0 {
			fmt.Printf("SDBM num: %d collisions: %d\n", i+1, collisionNum)
		}
	}
	collisionRate = float64(collisionNum) / float64(num)

	fmt.Printf("SDBM collisions: %d, collision rate: %f\n", collisionNum, collisionRate)

	// --------------- SipHash64 ------------------------
	m = make(map[uint64]*[]byte, num/5)
	collisionNum = 0
	for i := 0; i < num; i++ {
		// io.ReadFull(reader, src)
		h := util.SipHash64(randers[i])
		if b1, ok := m[h]; ok {
			collisionNum++
			fmt.Printf("key1: %s, key2: %s", hex.EncodeToString(*b1), hex.EncodeToString(randers[i]))
		} else {
			m[h] = &randers[i]
		}

		if (i+1)%10000000 == 0 {
			fmt.Printf("SipHash64 num: %d collisions: %d\n", i+1, collisionNum)
		}
	}
	collisionRate = float64(collisionNum) / float64(num)

	fmt.Printf("SipHash64 collisions: %d, collision rate: %f\n", collisionNum, collisionRate)

	// --------------- XXHash64 ------------------------
	m = make(map[uint64]*[]byte, num/5)
	collisionNum = 0
	for i := 0; i < num; i++ {
		// io.ReadFull(reader, src)
		h := util.XXHash64(randers[i])
		if b1, ok := m[h]; ok {
			collisionNum++
			fmt.Printf("key1: %s, key2: %s\n", hex.EncodeToString(*b1), hex.EncodeToString(randers[i]))
		} else {
			m[h] = &randers[i]
		}

		if (i+1)%10000000 == 0 {
			fmt.Printf("XXHash64 num: %d collisions: %d\n", i+1, collisionNum)
		}
	}
	collisionRate = float64(collisionNum) / float64(num)

	fmt.Printf("XXHash64 collisions: %d, collision rate: %f\n", collisionNum, collisionRate)
}

func main() {
	// for i := 0; i < 3; i++ {
	// 	executorPerformance()
	// }
	// fmt.Println("---------------------------------")
	// fmt.Println("---------------------------------")
	executorCollisionRate()
}
