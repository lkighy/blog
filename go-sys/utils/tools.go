package utils

import (
	"crypto/rand"
	"fmt"
	"math/big"
)

func GenerateVerify() string {
	//rand.Int() rand.Int(rand.Reader, big.NewInt(6))
	dice, err := rand.Int(rand.Reader, big.NewInt(999999))
	if err != nil {
		return "000000"
	}
	return fmt.Sprintf("%06v", dice)
}

func SendMail() {
	//smtp.SendMail()
}
