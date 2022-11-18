#[allow(unused)]

use std::io;
use rand::Rng;

fn main() {
    loop {
        let mut user_input: String = String::new();
        let mut rng = rand::thread_rng();
        let random_num = rng.gen_range(0..=10);

        println!("{}", random_num);
        io::stdin()
            .read_line(&mut user_input)
            .expect("Invalid input.");
    }
}
 