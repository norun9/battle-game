extern crate rand;
use rand::Rng;

#[derive(Debug)]
pub struct Brave {
    pub name: String,
}

#[derive(Debug)]
struct Spec {
    pub attack: i32,
    pub hit_point: i32,
}

impl Brave {
    pub fn new() -> Self {
        let mut name = String::new();
        println!("Please decide the name of brave");
        std::io::stdin().read_line(&mut name).ok();
        println!("Brave {}. Defeat the monster!!", name);
        Brave { name }
    }
}
