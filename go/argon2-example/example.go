package main

import (
	"fmt"
	"crypto/rand"
	"crypto/subtle"
	"crypto/sha256"

	"golang.org/x/crypto/argon2"
)

func argon2IDEncode(password, salt []byte) []byte {
	// time: 1 memory: 64*1024
	// threads: 4  返回密钥长度: 32
	return argon2.IDKey(password, salt, 1, 64 * 1024, 4, 32)
}

func argon2IDVerify(key, password, salt []byte) bool {
	if len(key) != 32 {
		return false
	}
	value := argon2IDEncode(password, salt)
	return subtle.ConstantTimeCompare(value, key) == 1
}

func main() {
	plaintext := []byte("argon2id123456")
	// 正常可以将密码sha归一化
	h := sha256.New()
	h.Write(plaintext)
	password := h.Sum(nil)
	fmt.Printf("pass: %x\n", password)
	// 一步生成sha256
	password2 := sha256.Sum256(plaintext)
	fmt.Println(subtle.ConstantTimeCompare(password, password2[:]) == 1)
	// 生成盐
	salt := make([]byte, 20)
	n, err := rand.Read(salt)
	if err != nil {
		fmt.Println("salt err:", err)
		return
	}
	fmt.Println("generate sale length:", n)
	key := argon2IDEncode(password, salt)
	fmt.Printf("key: %x\n", key)

	fmt.Println(argon2IDVerify(key, password, salt))
	fmt.Println(argon2IDVerify(key, []byte("arango2id12345678"), salt))
	salt2 := make([]byte, 20)
	_, err = rand.Read(salt2)
	if err != nil {
		fmt.Println("salt2 err:", err)
	}
	fmt.Println(argon2IDVerify(key, password, salt2))
}
