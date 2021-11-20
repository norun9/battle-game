extern crate rand;
use rand::Rng;

#[derive(Debug)]
pub struct Scene {
    brave: Brave,
    spec: Spec,
}

#[derive(Debug)]
struct Brave {
    pub name: String,
}

#[derive(Debug)]
struct Spec {
    pub attack: i32,
    pub hit_point: i32,
}

impl Scene {
    pub fn new() -> Self {
        let mut name = String::new();
        println!("Please decide the name of brave");
        std::io::stdin().read_line(&mut name).ok();
        let brave: Brave = Brave { name };
        println!("{} has appeared!!", monster_names[number].to_string());
        println!("Attack is {}", attack);
        println!("Hit Point is {}", hit_point);

        return Scene { monster, spec };
    }
}
