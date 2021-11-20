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

impl Spec {
    fn new(attack: Attack) -> Self {
        let magic_point: i32 = rand::thread_rng().gen_range(60, 151);
        let hit_point: i32 = rand::thread_rng().gen_range(100, 100001);
        Spec {
            hit_point,
            magic_point,
            attack,
        }
    }
}

#[derive(Debug)]
struct Attack {
    pub power: i32,   // 単純攻撃(mpが使用する魔法の消費mpよりも低ければ使用するしかない)
    pub magic: Magic, // 魔法能力
}

impl Attack {
    fn new(magic: Magic) -> Self {
        let power: i32 = rand::thread_rng().gen_range(100, 500);
        Attack { power, magic }
    }
}

// 今は一種類
#[derive(Debug)]
struct Magic {
    pub name: String,
    pub consume_magic_point_amount: i32, // 消費するMP量
    pub magic_power: i32,                // 魔法攻撃力
}

#[derive(Debug)]
enum MagicType {
    Fire(String, i32, i32),
    Frozen(String, i32, i32),
    Thunder(String, i32, i32),
    Wind(String, i32, i32),
}

impl Magic {
    fn new() -> Self {
        let types = ["fire", "frozon", "thunder", "wind"];
        let number: usize = rand::thread_rng().gen_range(0, 5);
        let magic_type: MagicType = match types[number] {
            "fire" => MagicType::Fire(String::from("Fire"), 1500, 30), // TODO: 対属性攻撃力上乗せ
            "water" => MagicType::Frozen(String::from("Blizzard"), 1500, 30),
            "thunder" => MagicType::Thunder(String::from("Thunder"), 1500, 30),
            "wind" => MagicType::Wind(String::from("Aero"), 1500, 30),
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
            magic_power,
        }
    }
}

impl Monster {
    pub fn new() -> Self {
        // Magic constructor :
        let magic: Magic = Magic::new();

        // Attack constroctor :
        let attack: Attack = Attack::new(magic);

        // Spec constructor :
        let spec: Spec = Spec::new(attack);

        let monster_names = ["Bahamut", "Tiamat", "Charybdis", "Weapon mouse", "Redicle"];
        let number: usize = rand::thread_rng().gen_range(0, 5);
        let monster: Monster = Monster {
            name: monster_names[number].to_string(),
            spec: spec,
        };
        println!("{} has appeared!!", monster.name);
        monster
    }
}
