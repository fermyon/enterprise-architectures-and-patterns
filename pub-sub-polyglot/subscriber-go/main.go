package main

import (
	"fmt"

	"github.com/fermyon/spin/sdk/go/v2/redis"
)

func init() {
	redis.Handle(func(payload []byte) error {
		fmt.Println("Received Message via Redis Channel")
		fmt.Println(string(payload))
		return nil
	})
}

func main() {}
