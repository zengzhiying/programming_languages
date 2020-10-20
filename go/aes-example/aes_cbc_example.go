package main

import (
	"bytes"
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"io"
	// "os"
)

func main() {
	aesCBCEncrypt()
	aesCBCDecrypt()
}

// block size为16采用pkcs7填充, 为8采用pcks5填充
func PKCS7Padding(plaintext []byte) []byte {
	// padding: 1 ~ 16
	padding := aes.BlockSize - len(plaintext) % aes.BlockSize
	paddingBytes := bytes.Repeat([]byte{byte(padding)}, padding)
	return append(plaintext, paddingBytes...)
}

func PKCS7UnPadding(ciphertext []byte) []byte {
	length := len(ciphertext)
	padding := int(ciphertext[length - 1])
	return ciphertext[:(length - padding)]
}


func aesCBCEncrypt() {
	// key可以采用密码的sha生成, 比如: sha128/sha256
	key, _ := hex.DecodeString("03c9888423e4af3efc25ddd7f86d575193edb8fdc43bc9f3f5a9cd726c7713a4")
	plaintext := []byte("xkkslslsc")
	plaintext = PKCS7Padding(plaintext)
	block, err := aes.NewCipher(key)
	if err != nil {
		panic(err)
	}

	ciphertext := make([]byte, aes.BlockSize + len(plaintext))
	iv := ciphertext[:aes.BlockSize]
	if _, err := io.ReadFull(rand.Reader, iv); err != nil {
		panic(err)
	}

	mode := cipher.NewCBCEncrypter(block, iv)
	mode.CryptBlocks(ciphertext[aes.BlockSize:], plaintext)

	fmt.Printf("%x\n", ciphertext)
}

func aesCBCDecrypt() {
	key, _ := hex.DecodeString("03c9888423e4af3efc25ddd7f86d575193edb8fdc43bc9f3f5a9cd726c7713a4")
	ciphertext, _ := hex.DecodeString("22b869b85c99d508fa7b67e886ac1a7e1484e03c7216896b2e675ef8c823aa23")

	block, err := aes.NewCipher(key)
	if err != nil {
		panic(err)
	}

	iv := ciphertext[:aes.BlockSize]
	ciphertext = ciphertext[aes.BlockSize:]

	mode := cipher.NewCBCDecrypter(block, iv)
	mode.CryptBlocks(ciphertext, ciphertext)

	plaintext := PKCS7UnPadding(ciphertext)
	fmt.Printf("%s\n", plaintext)
}
