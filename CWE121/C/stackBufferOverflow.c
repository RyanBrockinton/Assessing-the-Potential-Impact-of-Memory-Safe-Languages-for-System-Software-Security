/* To run native: 
    gcc stackBufferOverflow.c

   To run Web Assembly: 
    emcc -g stackBufferOverflow.c -o stackBufferOverflow.html
        1. In the js file enter your arguments in the "arguments_" array
        2. Launch local host server for the html file
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BUFSIZE 16

void printStackData(char buf1[BUFSIZE], char buf2[BUFSIZE]) {
    printf("Stack Data 1: %s, Memory Address: %p\n", buf1, buf1);
    printf("Stack Data 2: %s, Memory Address: %p\n", buf2, buf2);
}

int main(int argc, char *argv[]) {
    
    printf("argv[1]: %s\n", argv[1]);

    char buf1[BUFSIZE], buf2[BUFSIZE];
    strcpy(buf1, "password");

    printf("\nExecute:\nchar buf1[%i], buf2[%i];\nstrcpy(buf1, \"password\");\n\n", BUFSIZE, BUFSIZE);

    printStackData(buf1, buf2);

    printf("\nExecute:\nstrcpy(buf2, argv[1]);\n\n");

    strcpy(buf2, argv[1]);
    
    printStackData(buf1, buf2);

    return 0;

}