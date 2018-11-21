extern crate translate;

use std::env;
use translate::translate;
use translate::pirate::Pirate;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text: String = args.into_iter().skip(1).collect::<Vec<String>>().join(" ");
    println!("{}", translate::<Pirate>(&text));
}
