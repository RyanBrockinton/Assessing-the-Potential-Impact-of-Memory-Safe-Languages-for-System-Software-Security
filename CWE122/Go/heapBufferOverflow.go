/* To run native:
    go run heapBufferOverflow.go

   To run Web Assembly:
    GOOS=js GOARCH=wasm go build -o heapBufferOverflow.wasm heapBufferOverflow.go
        1. In the js file enter your arguments in the "this.argv" array on line 97
        2. Launch local host server for the html file
*/

package main

import (
	"fmt"
	"os"
)

const BUFSIZE = 8

func printHeapData(ptr1, ptr2 []byte) {
	fmt.Printf("Heap Data 1: %s, Memory Address: %p\n", ptr1, &ptr1[0])
	fmt.Printf("Heap Data 2: %s, Memory Address: %p\n", ptr2, &ptr2[0])
}

func main() {
	if len(os.Args) < 3 {
		fmt.Println("Please provide two command line arguments.")
		return
	}

	fmt.Printf("argv[1]: %s\n", os.Args[1])
	fmt.Printf("argv[2]: %s\n", os.Args[2])

	ptr1 := make([]byte, BUFSIZE)
	ptr2 := make([]byte, BUFSIZE)

	fmt.Printf("\nExecute:\nvar ptr1, ptr2 []byte\nptr1 = make([]byte, %d)\nptr2 = make([]byte, %d)\n\n", BUFSIZE, BUFSIZE)

	printHeapData(ptr1, ptr2)

	copy(ptr1, os.Args[1])

	fmt.Printf("\nExecute:\ncopy(ptr1, os.Args[1])\n")

	printHeapData(ptr1, ptr2)

	copy(ptr2, os.Args[2])

	fmt.Printf("\nExecute:\ncopy(ptr2, os.Args[2])\n")

	printHeapData(ptr1, ptr2)
}
