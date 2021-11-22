extern crate rand;
use rand::Rng;

pub trait Common {
    fn get_attribute<'a>() -> &'a str {
        let attributes = ["fire", "frozon", "thunder", "wind"];
        let number: usize = rand::thread_rng().gen_range(0, 4);
        attributes[number]
    }

    fn new() -> Self;
}

#[derive(Debug)]
pub struct Spec {
    pub hit_point: i32,
    pub magic_point: i32,
    pub attack: Attack,
    pub evasion: Vec<bool>, // TODO: 回避可能確率配列
}

impl Spec {
    pub fn new(attack: Attack) -> Self {
        let magic_point: i32 = rand::thread_rng().gen_range(60, 151);
        let hit_point: i32 = rand::thread_rng().gen_range(100, 100001);
        let evasion: Vec<bool> = vec![true, false, false, true, false, false, true, false, false];
        Spec {
            hit_point,
            magic_point,
            attack,
            evasion,
        }
    }
}

#[derive(Debug)]
pub struct Attack {
    pub power: i32,   // 通常攻撃
    pub magic: Magic, // 魔法能力(MPが使用する魔法の消費MPよりも低ければ使用不可)
}

impl Attack {
    pub fn new(magic: Magic) -> Self {
        let power: i32 = rand::thread_rng().gen_range(100, 500);
        Attack { power, magic }
    }
}

// 今は一種類
#[derive(Debug)]
pub struct Magic {
    pub name: String,
    pub consume_magic_point_amount: i32, // 消費MP量
    pub power: i32,                      // 魔法攻撃力
}

#[derive(Debug)]
enum MagicType {
    Fire(String, i32, i32),
    Frozen(String, i32, i32),
    Thunder(String, i32, i32),
    Wind(String, i32, i32),
}

impl Magic {
    pub fn new(attribute: &str) -> Self {
        let magic_type: MagicType = match attribute {
            "fire" => MagicType::Fire(String::from("Fire"), 1500, 30), // TODO: 属性相性攻撃力UP
            "water" => MagicType::Frozen(String::from("Blizzard"), 1500, 30),
            "thunder" => MagicType::Thunder(String::from("Thunder"), 1500, 30),
            "wind" => MagicType::Wind(String::from("Aero"), 1500, 30),
            _ => MagicType::Fire(String::from("Fire"), 1500, 30),
        };

        let (name, consume_magic_point_amount, magic_power) = match magic_type {
            MagicType::Fire(name, consume_magic_point_amount, magic_power) => {
                (name, consume_magic_point_amount, magic_power)
            }
            MagicType::Frozen(name, consume_magic_point_amount, magic_power) => {
                (name, consume_magic_point_amount, magic_power)
            }
            MagicType::Thunder(name, consume_magic_point_amount, magic_power) => {
                (name, consume_magic_point_amount, magic_power)
            }
            MagicType::Wind(name, consume_magic_point_amount, magic_power) => {
                (name, consume_magic_point_amount, magic_power)
            }
        };

        Magic {
            name,
            consume_magic_point_amount,
            power: magic_power,
        }
    }
}
