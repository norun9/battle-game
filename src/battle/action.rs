extern crate rand;
use super::brave;
use super::monster;

pub struct Action {
    pub turn: String,
    pub is_attack: bool,
    pub is_magic: bool,
}

impl Action {
    pub fn new(turn: String, is_attack: bool, is_magic: bool) -> Self {
        Action {
            turn,
            is_attack,
            is_magic,
        }
    }

    pub fn attack(self, brave: &mut brave::Brave, monster: &mut monster::Monster) {
        if self.turn == "monster" {
            if self.is_attack {
                println!("魔物【 {} 】は 通常攻撃 を選んだ。\n", monster.name);
                // 勇者のhit pointにモンスターのパワーをマイナス(攻撃)
                brave.spec.hit_point -= monster.spec.attack.power; // attack is power
            } else {
                println!(
                    "魔物【 {} 】は 攻撃魔法【 {} 】を選んだ。\n",
                    monster.name, monster.spec.attack.magic.name
                );
                // mpより消費mpの方が大きければエラー
                if monster.spec.magic_point < monster.spec.attack.magic.consume_magic_point_amount {
                    // TODO: エラー
                }
                // 勇者のhit pointにモンスターの魔法パワーをマイナス(魔法攻撃)
                brave.spec.hit_point -= monster.spec.attack.magic.power;
                // mpを消費する
                monster.spec.magic_point -= monster.spec.attack.magic.consume_magic_point_amount;
            }

            println!(
                "勇者【 {} 】は {} のダメージを受けた。\n",
                brave.name, monster.spec.attack.power
            );
            println!(
                "勇者【 {} 】の残りの体力は {} です。\n",
                brave.name, brave.spec.hit_point
            );
        } else {
            if self.is_attack {
                println!("勇者【 {} 】は 通常攻撃 を選んだ。\n", brave.name);
                // モンスターのhit pointに勇者のパワーをマイナス(攻撃)
                monster.spec.hit_point -= brave.spec.attack.power; // attack is power
                if monster.spec.hit_point <= 0 {
                    // モンスター負け
                }
            } else {
                println!(
                    "勇者【 {} 】は 攻撃魔法【 {} 】を選んだ。\n",
                    brave.name, brave.spec.attack.magic.name
                );
                // mpより消費mpの方が大きければエラー
                if brave.spec.magic_point < brave.spec.attack.magic.consume_magic_point_amount {
                    // TODO: エラー
                }
                // モンスターのhit pointにモンスターの魔法パワーをマイナス(魔法攻撃)
                monster.spec.hit_point -= brave.spec.attack.magic.power;
                // mpを消費する
                brave.spec.magic_point -= brave.spec.attack.magic.consume_magic_point_amount;
            }
            println!(
                "魔物【 {} 】に {} のダメージを与えた。\n",
                monster.name, brave.spec.attack.power
            );
            println!(
                "魔物【 {} 】の残りの体力は {} です。\n",
                monster.name, monster.spec.hit_point
            );
        }
    }
}
