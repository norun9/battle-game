mod battle;
use battle::Common;
extern crate rand;
use rand::Rng;

fn main() {
    let mut brave: &mut battle::Brave = Common::new();
    println!("{:?}", brave);
    let mut monster: &mut battle::Monster = Common::new();
    println!("{:?}", monster);

    let turn_condition: bool = true;
    loop {
        let mut action: battle::Action;
        if turn_condition {
            // 勇者ターン
            action.turn = String::from("brave");
            let mut command = String::new();
            loop {
                println!("攻撃方法を選択してください。【 attack 】【 magic 】");
                std::io::stdin().read_line(&mut command).ok();
                println!(">");
                if command != "" {
                    if command == "attack" {
                        action.attack = Some(brave.spec.attack.power);
                    }
                    if command == "magic" {
                        action.magic = Some(brave.spec.attack.magic);
                    }
                    break;
                }
            }
            println!("勇者が選んだのは【 {} 】です", command);
        } else {
            // モンスターターン
            action.turn = String::from("monster");
            let commands = ["attack", "magic"];
            let number: usize = rand::thread_rng().gen_range(0, 2);
            if commands[number] == "attack" {
                action.attack = Some(brave.spec.attack.power);
            }
            if commands[number] == "magic" {
                action.magic = Some(brave.spec.attack.magic);
            }
            println!("モンスターが選んだのは【 {} 】です", commands[number]);
        }

        Common::action(action, brave, monster);

        if brave.spec.hit_point <= 0 {
            println!("【 モンスター 】の勝利！");
            break;
        } else {
            println!("【 勇者 】の勝利！");
            break;
        }
    }
}
