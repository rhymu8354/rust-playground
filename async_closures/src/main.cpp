#include <future>
#include <stdlib.h>
#include <stdio.h>
#include <string>

int main(int argc, char* argv[]) {
    auto name = std::string("Fred");
    auto closure = [&]{
        name = "Bob";
    };
    printf("Name is %s\n", name.c_str());
    name = "Alice";
    printf("Name is %s\n", name.c_str());
    closure();
    printf("Name is %s\n", name.c_str());
    return EXIT_SUCCESS;
}
