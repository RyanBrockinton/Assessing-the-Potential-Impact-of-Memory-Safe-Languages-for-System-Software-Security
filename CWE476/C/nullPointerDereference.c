/* To run native: 
    gcc nullPointerDereference.c

   To run Web Assembly: 
    emcc -g nullPointerDereference.c -o nullPointerDereference.html
        1. In the js file enter your arguments in the "arguments_" array
        2. Launch local host server for the html file
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[]) {
    if(argc > 1) {
        char* ptr = NULL;

        if (strcmp(argv[1], "valid") == 0) {
            // Allocate memory if user enters 1
            ptr = (char*)malloc(strlen(argv[0])*sizeof(char));
        }

        // Attempt to use the pointer without checking if it's NULL
        // This can lead to a NULL pointer dereference if memory wasn't allocated
        *ptr = 10;

        printf("Value: %d\n", *ptr);
        free(ptr);
    } else {
        printf("Invalid number of arguments");
    }

    return 0;
}
