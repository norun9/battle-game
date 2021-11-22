extern crate rand;
use rand::Rng;

mod battle;
use battle::Action;
use battle::Common;

fn main() {
    let mut brave: battle::Brave = Common::new();
    let mut_brave: &mut battle::Brave = &mut brave;
    let mut monster: battle::Monster = Common::new();
    let mut_monster: &mut battle::Monster = &mut monster;

    let mut turn_condition: bool = true;
    loop {
        let turn: String;
        let mut is_attack: bool = false;
        let mut is_magic: bool = false;
        if turn_condition {
            // 勇者ターン
            turn = String::from("brave");
            let mut command = String::new();
            loop {
                println!("攻撃方法を選択してください。【 attack 】or【 magic 】\n");
                std::io::stdin().read_line(&mut command).ok();
                println!("");
                assert_eq!(command, "attack");
                if command != "" {
                    if command == "attack" {
                        is_attack = true;
                    }
                    if command == "magic" {
                        is_magic = true;
                    }
                    break;
                }
            }
        } else {
            // 魔物ターン
            turn = String::from("monster");
            let commands = ["attack", "magic"];
            let number: usize = rand::thread_rng().gen_range(0, 2);
            if commands[number] == "attack" {
                is_attack = true;
            }
            if commands[number] == "magic" {
                is_magic = true;
            }
        }

        let action: battle::Action = battle::Action::new(turn, is_attack, is_magic);
        Action::attack(action, mut_brave, mut_monster);

        turn_condition = !turn_condition;

        if mut_brave.spec.hit_point <= 0 {
            println!("魔物【 {} 】に敗北しました。\n", mut_monster.name);
            break;
        }
        if mut_monster.spec.hit_point <= 0 {
            println!("勇者【 {} 】の勝利です！\n", mut_brave.name);
            break;
        }
    }
}
