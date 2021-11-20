extern crate rand;
use rand::Rng;

pub fn run() {
    let number: i32 = rand::thread_rng().gen_range(1, 6);
    match number {
        1 => println!("{}", "凶"),
        2 | 3 => println!("{}", "吉"),
        4 | 5 => println!("{}", "中吉"),
        _ => println!("{}", "大吉"),
    }
}
