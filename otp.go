package main

import (
	"fmt"
	"os"

	otp "github.com/hgfischer/go-otp"
)

var (
	secret string
)

func main() {
	if secret == "" {
		fmt.Fprintf(os.Stderr, "Not compiled with a secret. Please see the readme.")
		os.Exit(1)
	}
	totp := &otp.TOTP{Secret: secret, IsBase32Secret: true}
	token := totp.Get()
	fmt.Println(token)
}
