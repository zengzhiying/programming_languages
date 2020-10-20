package main

import (
	// "bytes"
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"io"
	// "os"
)

func main() {
	aesGCMEncrypter()

	aesGCMDecrypt()
}

func aesGCMEncrypter() {
	// 密钥 长度16字节 - AES128, 32 - AES-256
	// 这个key可以通过sha256对用户密码进行计算得到
	key, _ := hex.DecodeString("6368616e676520746869732070617373776f726420746f206120736563726574")
	plaintext := []byte("exampleplaintext")

	block, err := aes.NewCipher(key)
	if err != nil {
		panic(err)
	}

	nonce := make([]byte, 12)
	if _, err := io.ReadFull(rand.Reader, nonce); err != nil {
		panic(err)
	}

	fmt.Printf("random: %x\n", nonce)

	aesgcm, err := cipher.NewGCM(block)
	if err != nil {
		panic(err)
	}

	ciphertext := aesgcm.Seal(nil, nonce, plaintext, nil)

	fmt.Printf("%x\n", ciphertext)
}

func aesGCMDecrypt() {
	key, _ := hex.DecodeString("6368616e676520746869732070617373776f726420746f206120736563726574")
	ciphertext, _ := hex.DecodeString("d4a4ce3d36b912f73a71235b32a17d32390c4af5")
	nonce, _ := hex.DecodeString("0b7639120d8c6c7fe8cc0699")

	block, err := aes.NewCipher(key)
	if err != nil {
		panic(err)
	}

	aesgcm, err := cipher.NewGCM(block)
	if err != nil {
		panic(err)
	}

	plaintext, err := aesgcm.Open(nil, nonce, ciphertext, nil)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%s\n", plaintext)
}
