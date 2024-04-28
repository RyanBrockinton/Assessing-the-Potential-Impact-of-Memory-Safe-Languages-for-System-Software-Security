/* To run native:
    go run stackBufferOverflow.go

   To run Web Assembly:
    GOOS=js GOARCH=wasm go build -o stackBufferOverflow.wasm stackBufferOverflow.go
        1. In the js file enter your arguments in the "this.argv" array on line 97
        2. Launch local host server for the html file
*/

package main

import (
	"fmt"
	"os"
)

const BUFSIZE = 16

func printStackData(buf1, buf2 []byte) {
	fmt.Printf("Stack Data 1: %s, Memory Address: %p\n", buf1, &buf1[0])
	fmt.Printf("Stack Data 2: %s, Memory Address: %p\n", buf2, &buf2[0])
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: program <input>")
		return
	}

	fmt.Printf("argv[1]: %s\n", os.Args[1])

	var buf1 [BUFSIZE]byte
	var buf2 [BUFSIZE]byte

	copy(buf1[:], "password")

	fmt.Printf("\nExecute:\nvar buf1 [%d]byte\nvar buf2 [%d]byte\ncopy(buf1[:], \"password\")\n\n", BUFSIZE, BUFSIZE)

	printStackData(buf1[:], buf2[:])

	fmt.Printf("\nExecute:\ncopy(buf2[:], os.Args[1])\n\n")

	copy(buf2[:], os.Args[1])

	printStackData(buf1[:], buf2[:])
}
