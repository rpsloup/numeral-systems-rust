#[allow(unused)]

use std::io;
use std::io::{Write};
use rand::Rng;

fn main() {
    let mut bin_input: String = String::new();
    let mut hex_input: String = String::new();
    let mut rng = rand::thread_rng();

    loop {
        let random_num = rng.gen_range(1..=255);

        println!("DEC: {}", random_num);
        print!("BIN: ");
        io::stdout().flush().expect("Flush failed!");
        io::stdin()
            .read_line(&mut bin_input)
            .expect("Invalid input!");
        
        print!("HEX: ");
        io::stdout().flush().expect("Flush failed!");
        io::stdin()
            .read_line(&mut hex_input)
            .expect("Invalid input!");

        println!();
    }
}
