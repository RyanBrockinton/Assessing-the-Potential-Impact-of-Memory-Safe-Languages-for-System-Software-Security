// To Compile: g++ cwe476.cpp

/* This example looks to run correctly even with a null pointer dereference due to optimizations in the compiler but it
    is undefined behavior.
*/

#include <iostream>
#include <cstring>

class SampleClass {
    int x;

    public:
        void setX() {
            x = 10;
        }
        void printMessage() {
            std::cout << "Hello World!" << std::endl;
        }
};

SampleClass* createSampleClass(char *input) {
    if (strcmp(input, "valid") == 0) {
        std::cout << "valid pointer test\n" << std::endl;
        return new SampleClass();
    } else {
        // Simulating a condition that leads to returning a NULL pointer
        std::cout << "null pointer dereference test\n" << std::endl;
        return nullptr;
    }
}

int main(int argc, char *argv[]) {
    if(argc > 1) {
        char* isValid = argv[1];

        // Simulate a condition where we accidentally create a NULL pointer
        SampleClass* myObject = createSampleClass(isValid);

        std::cout << "---------------------------------------------------------------------" << std::endl;
        std::cout << "Attempting to run null pointer class member function that does not reference member variables or virtual functions:" << std::endl;

        // Attempt to use the object without checking if it's NULL
        // This will lead to a NULL pointer dereference
        myObject->printMessage();

        std::cout << "Print function ran without a crash" << std::endl;
        std::cout << "---------------------------------------------------------------------\n" << std::endl;
        std::cout << "Attempting to run null pointer class member function that references a member variable:" << std::endl;

        // Attempt to use the object without checking if it's NULL
        // This will lead to a NULL pointer dereference
        myObject->setX();

        std::cout << "Member variable reference function ran without a crash" << std::endl;
        std::cout << "---------------------------------------------------------------------" << std::endl;

        delete myObject;
    } else {
        std::cout << "Invalid number of arguments";
    }

    return 0;
}
