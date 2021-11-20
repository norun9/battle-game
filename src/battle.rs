extern crate rand;
use rand::Rng;

#[derive(Debug)]
pub struct Scene {
    monster: Monster,
    spec: Spec,
}

#[derive(Debug)]
struct Monster {
    pub name: String,
}

#[derive(Debug)]
struct Spec {
    pub attack: i32,
    pub hit_point: i32,
}

impl Scene {
    pub fn new() -> Self {
        let monster_names = ["Bahamut", "Tiamat", "Charybdis", "Weapon mouse", "Redicle"];
        let number: usize = rand::thread_rng().gen_range(0, 5);
        let attack = rand::thread_rng().gen_range(100, 50001);
        let hit_point = rand::thread_rng().gen_range(100, 100001);
        let spec: Spec = Spec { attack, hit_point };
        let monster: Monster = Monster {
            name: monster_names[number].to_string(),
        };
        println!("{} has appeared!!", monster_names[number].to_string());
        println!("Attack is {}", attack);
        println!("Hit Point is {}", hit_point);

        return Scene { monster, spec };
    }
}
