/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;

use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Bolatan Ibrahim <ehbraheem@gmail.com>",
    about = "Number of fruits to include in the salad"
)]

struct Opts {
    #[clap(short, long)]
    fruits: Vec<String>,
}

fn main() {
    let opts = Opts::parse();
    let mut fruit = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    // Add the fruits provided by the user to the fruit salad
    let user_fruits: Vec<String> = opts.fruits;

    if !user_fruits.is_empty() {
        user_fruits.iter().for_each(|f| {
            fruit.push(f);
        });
    }

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
