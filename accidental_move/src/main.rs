struct Page {
    number: i32,
}

impl Page {
    fn new(number: i32) -> Self {
        Self {
            number,
        }
    }

    fn advance(&mut self) {
        self.number += 1;
    }
}

enum BookState {
    Closed,
    Open(Page),
}

struct Book {
    pages: i32,
    finished: bool,
    state: BookState,
}

impl Book {
    fn new(pages: i32) -> Self {
        Self {
            pages,
            finished: false,
            state: BookState::Closed,
        }
    }

    fn close(&mut self) {
        self.state = BookState::Closed;
    }

    fn open(&mut self) {
        self.state = BookState::Open(Page::new(1));
    }

    fn read_page(&self) {
        if let BookState::Open(page) = self.state {
            println!(
                "Reading next page...  {} pages left.",
                self.pages - page.number
            );
        }
    }

    fn turn_page(&mut self) {
        if let BookState::Open(page) = self.state {
            if page.number < self.pages {
                page.advance();
            } else {
                self.finished = true;
            }
        }
    }
}

fn main() {
    let mut book = Book::new(10);
    book.open();
    while !book.finished {
        book.read_page();
        book.turn_page();
    }
    book.close();
    println!("Done reading!");
}
