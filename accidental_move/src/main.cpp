#include <stdlib.h>
#include <stdio.h>

struct Page {
    int number;

    Page(int number)
        : number(number)
    {
    }

    void advance() {
        ++number;
    }
};

enum BookState {
    Closed,
    Open,
};

union BookState_u {
    Page open;

    BookState_u() {}
};

struct Book {
    int pages;
    bool finished;
    BookState state;
    BookState_u state_u;

    Book(int pages)
        : pages(pages)
        , finished(false)
        , state(BookState::Closed)
        , state_u()
    {
    }

    void close() {
        state = BookState::Closed;
    }

    void open() {
        state = BookState::Open;
        state_u.open.number = 1;
    }

    void read_page() {
        if (state == BookState::Open) {
            printf(
                "Reading next page...  %d pages left.\n",
                pages - state_u.open.number
            );
        }
    }

    void turn_page() {
        if (state == BookState::Open) {
            if (state_u.open.number < pages) {
                state_u.open.advance();
            } else {
                finished = true;
            }
        }
    }
};

int main(int argc, char* argv[]) {
    Book book(10);
    book.open();
    while (!book.finished) {
        book.read_page();
        book.turn_page();
    }
    book.close();
    printf("Done reading!\n");
    return EXIT_SUCCESS;
}
