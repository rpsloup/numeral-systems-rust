#[allow(unused)]

use std::io;
use std::io::Write;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    loop {
        let random_num: i32 = rng.gen_range(1..=255);
        let mut bin_input: String = String::new();
        let mut hex_input: String = String::new();

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

        let decoded_bin_input: i32 = i32::from_str_radix(&bin_input.trim(), 2).expect("Not a binary number!");
        let decoded_hex_input = hex::decode(&hex_input.trim());

        println!();
        if decoded_bin_input == random_num {
            println!("BIN: Correct!");
        } else {
            println!("BIN: Incorrect! ({})", format!("{:08b}", random_num));
        }

        if decoded_hex_input.as_ref().unwrap()[0] == random_num as u8 {
            println!("HEX: Correct!");
        } else {
            println!("HEX: Incorrect! ({})", format!("{:X}", random_num));
        }

        println!();
    }
}
