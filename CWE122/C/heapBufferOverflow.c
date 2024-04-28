/* To run native: 
    gcc heapBufferOverflow.c

   To run Web Assembly: 
    emcc -g heapBufferOverflow.c -o heapBufferOverflow.html
        1. In the js file enter your arguments in the "arguments_" array
        2. Launch local host server for the html file
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
# define BUFSIZE 8

void printHeapData(char *ptr1, char *ptr2) {
    printf("Heap Data 1: %s, Memory Address: %p\n", ptr1, ptr1);
    printf("Heap Data 2: %s, Memory Address: %p\n", ptr2, ptr2);
}

int main(int argc, char *argv[]) {
    
    printf("argv[1]: %s\n", argv[1]);
    printf("argv[2]: %s\n", argv[2]);

    char *ptr1, *ptr2;
    ptr1 = malloc(BUFSIZE);
    ptr2 = malloc(BUFSIZE);

    printf("\nExecute:\nchar *ptr1, *ptr2;\nptr1 = malloc(%i);\nptr2 = malloc(%i);\n\n", BUFSIZE, BUFSIZE);

    printHeapData(ptr1, ptr2);

    strcpy(ptr1, argv[1]);

    printf("\nExecute:\nstrcpy(ptr1, argv[1]);\n\n");

    printHeapData(ptr1, ptr2);

    strcpy(ptr2, argv[2]);

    printf("\nExecute:\nstrcpy(ptr2, argv[2]);\n\n");
    
    printHeapData(ptr1, ptr2);

    return 0;

}