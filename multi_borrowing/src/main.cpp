#include <stdlib.h>
#include <stdio.h>
#include <string>

struct Context {
    std::string name;

    Context(std::string&& name)
        : name(std::move(name))
    {
    }

    void add_prefix_by_ref(
        const std::string& prefix
    ) {
        name = prefix + name;
    }

    void add_prefix_by_value(
        std::string&& prefix
    ) {
        name = std::move(prefix) + name;
    }

    void report() {
        printf("The name is %s.\n", name.c_str());
    }
};

int main(int argc, char* argv[]) {
    Context context("Foo");
    context.report();
    context.add_prefix_by_ref(
        "Bar"
    );
    context.report();
    context.add_prefix_by_value(
        std::string(context.name)
    );
    context.report();
    context.add_prefix_by_ref(
        context.name
    );
    context.report();
    return EXIT_SUCCESS;
}
