extern crate rand;
use rand::Rng;

use super::brave;
use super::monster;

pub struct Action {
    pub turn: String,
    pub attack: Option<i32>,
    pub magic: Option<Magic>,
}

pub trait Common {
    fn get_attribute<'a>() -> &'a str {
        let attributes = ["fire", "frozon", "thunder", "wind"];
        let number: usize = rand::thread_rng().gen_range(0, 4);
        attributes[number]
    }

    fn new() -> &'static mut Self;

    fn action<'a>(
        action: Action,
        brave: &'a mut brave::Brave,
        monster: &'a mut monster::Monster,
    ) -> (&'a mut brave::Brave, &'a mut monster::Monster) {
        if action.turn == "monster" {
            match action.attack {
                None => {
                    println!(
                        "{} is selected {} magic",
                        monster.name, monster.spec.attack.magic.name
                    )
                }
                Some(attack) => {
                    // 勇者のhit pointにモンスターのパワーをマイナス(攻撃)
                    brave.spec.hit_point -= attack; // attack is power
                    if brave.spec.hit_point <= 0 {
                        // 勇者負け
                    }
                }
            }
            match action.magic {
                None => {
                    println!("{} is selected attack", monster.name)
                }
                Some(magic) => {
                    // mpより消費mpの方が大きければエラー
                    if monster.spec.magic_point < magic.consume_magic_point_amount {
                        // TODO: エラー
                    }
                    // 勇者のhit pointにモンスターの魔法パワーをマイナス(魔法攻撃)
                    brave.spec.hit_point -= monster.spec.attack.magic.power;
                    // mpを消費する
                    monster.spec.magic_point -=
                        monster.spec.attack.magic.consume_magic_point_amount;
                }
            }
        } else {
            match action.attack {
                None => {
                    println!(
                        "{} is selected {} magic",
                        brave.name, brave.spec.attack.magic.name
                    )
                }
                Some(attack) => {
                    // モンスターのhit pointに勇者のパワーをマイナス(攻撃)
                    monster.spec.hit_point -= attack; // attack is power
                    if monster.spec.hit_point <= 0 {
                        // モンスター負け
                    }
                }
            }
            match action.magic {
                None => {
                    println!("{} is selected attack", brave.name)
                }
                Some(magic) => {
                    // mpより消費mpの方が大きければエラー
                    if brave.spec.magic_point < magic.consume_magic_point_amount {
                        // TODO: エラー
                    }
                    // モンスターのhit pointにモンスターの魔法パワーをマイナス(魔法攻撃)
                    monster.spec.hit_point -= magic.power;
                    // mpを消費する
                    brave.spec.magic_point -= magic.consume_magic_point_amount;
                }
            }
        }
        return (brave, monster);
    }
}

#[derive(Debug)]
pub struct Spec {
    pub hit_point: i32,
    pub magic_point: i32,
    pub attack: Attack,     // 攻撃バリエーション
    pub evasion: Vec<bool>, // 回避確率配列
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
    pub power: i32,   // 単純攻撃(mpが使用する魔法の消費mpよりも低ければ使用するしかない)
    pub magic: Magic, // 魔法能力
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
    pub consume_magic_point_amount: i32, // 消費するMP量
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
            "fire" => MagicType::Fire(String::from("Fire"), 1500, 30), // TODO: 対属性攻撃力上乗せ
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