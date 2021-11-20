extern crate rand;
use rand::Rng;

#[derive(Debug)]
pub struct Monster {
    pub name: String,
    pub spec: Spec,
}

#[derive(Debug)]
struct Spec {
    pub hit_point: i32,
    pub magic_point: i32,
    pub attack: Attack, // 攻撃バリエーション
}

#[derive(Debug)]
struct Attack {
    pub power: i32,   // 単純攻撃(mpが使用する魔法の消費mpよりも低ければ使用するしかない)
    pub magic: Magic, // 魔法能力
}

// 今は一種類
#[derive(Debug)]
struct Magic {
    pub consume_magic_point_amount: i32, // 消費するMP量
    pub magic_power: i32,                // 魔法攻撃力
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
            spec: spec,
        };
        println!("{} has appeared!!", monster_names[number].to_string());
        println!("Attack is {}", attack);
        println!("Hit Point is {}", hit_point);

        return Scene { monster, spec };
    }
}
