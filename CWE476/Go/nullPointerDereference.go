/* To run native:
    go run nullPointerDereference.go

   To run Web Assembly:
    GOOS=js GOARCH=wasm go build -o nullPointerDereference.wasm nullPointerDereference.go
        1. In the js file enter your arguments in the "this.argv" array on line 97
        2. Launch local host server for the html file
*/

package main

import (
	"fmt"
	"os"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: program <input>")
		return
	}

	var ptr *byte

	if os.Args[1] == "valid" {
		// Allocate memory if user enters "valid"
		ptr = new(byte)
	}

	// Attempt to use the pointer without checking if it's nil
	// This can lead to a nil pointer dereference if memory wasn't allocated
	*ptr = 10
	fmt.Printf("Value: %d\n", *ptr)
}
