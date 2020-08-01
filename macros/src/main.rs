#![recursion_limit = "9001"]

#[derive(Debug)]
struct Cat {
    favorite_word: String,
    relative_age: String,
}

macro_rules! cat {

    ( $x:expr ) => {
        Cat {
            favorite_word: String::from("Meow"),
            relative_age: String::from($x),
        }
    };

    ( $x:expr , $y:expr ) => {
        Cat {
            favorite_word: String::from($x),
            relative_age: String::from($y),
        }
    };

}

macro_rules! dead_rule {
    ($i:ident +) => { { $i += 1; $i } };
    ($e:expr) => { $e };
}

macro_rules! vec {
    ( $( $x:expr ),* ) => [
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    ]
}

fn main() {

    let _x = vec![1, 2, 3];

    let _x = {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    };

    let _casper = cat!["Purr", "old"];
    let _casper = cat!["old"];

    let mut x = 42;
    dead_rule!(x+);

    println!("Hello, world!");
}
