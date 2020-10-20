package main

import (
	"fmt"
	"crypto/rand"
	"crypto/sha256"

	"github.com/tvdburgt/go-argon2"
)

// 使用第三方库实现argon2算法
/**
  git clone https://github.com/P-H-C/phc-winner-argon2.git argon2
  cd argon2
  // 最新的版本不支持
  git checkout 20171227
  make
  make install PREFIX=/usr/local/argon2
  ln -s /usr/local/argon2/include/argon2.h /usr/include/
  ln -s /usr/local/argon2/lib/libargon2.so /usr/lib
  go build -o go-argon2-example


  runtime:
  echo /usr/local/argon2/lib >> /etc/ld.so.conf
  ldconfig
  ./go-argon2-example
 */

func main() {
	plaintext := []byte("argon2id123456")
	h := sha256.New()
	h.Write(plaintext)
	password := h.Sum(nil)
	fmt.Printf("pass: %x\n", password)

	// 盐
	salt := make([]byte, 20)
	n, err := rand.Read(salt)
	if err != nil {
		fmt.Println("salt err:", err)
		return
	}
	fmt.Println("generate sale length:", n)

	// 生成字节hash
	ctx := &argon2.Context{
	    Iterations:  2,
	    Memory:      1 << 16,
	    Parallelism: 4,
	    HashLen:     32,
	    // 支持2i, 2d, 2id
	    Mode:        argon2.ModeArgon2id,
	    Version:     argon2.Version10,
	}

	key, err := argon2.Hash(ctx, password, salt)
	if err != nil {
		fmt.Println("argon2 hash err:", err)
		return
	}
	fmt.Printf("key: %x\n", key)

	// 校验hash
	salt2 := make([]byte, 20)
	_, err = rand.Read(salt2)
	if err != nil {
		fmt.Println("salt2 err:", err)
	}
	fmt.Println(argon2.Verify(ctx, key, password, salt))
	fmt.Println(argon2.Verify(ctx, key, []byte("arango2id12345678"), salt))
	fmt.Println(argon2.Verify(ctx, key, password, salt2))

	// 生成编码字符串
	keyEncode, err := argon2.HashEncoded(ctx, password, salt)
	if err != nil {
		fmt.Println("argon2 hash encode err:", err)
		return
	}
	fmt.Println("key encoded:", keyEncode)

	// 验证编码字符串
	fmt.Println(argon2.VerifyEncoded(keyEncode, password))
	fmt.Println(argon2.VerifyEncoded(keyEncode, []byte("arango2id12345678")))
}

