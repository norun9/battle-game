extern crate rand;
use super::common;
use rand::Rng;

#[derive(Debug)]
pub struct Monster {
    pub name: String,
    pub spec: common::Spec,
    pub attribute: String,
}

impl common::Common for Monster {
    fn new() -> Monster {
        // Magic constructor :
        let attribute = Self::get_attribute();
        let magic = common::Magic::new(attribute);

        // Attack constroctor :
        let attack: common::Attack = common::Attack::new(magic);

        // Spec constructor :
        let spec: common::Spec = common::Spec::new(attack, false);

        let monster_names = [
            "バハムート",
            "ティアマト",
            "カリュブディス",
            "サボテンダー",
            "ベヒーモス",
        ];
        let number: usize = rand::thread_rng().gen_range(0, 5);
        let monster_name = monster_names[number];
        println!("魔物【{}】が現れた！", monster_name);

        println!("魔物【{}】の属性は{}", monster_name, attribute);
        println!("魔物【{}】の体力は{}", monster_name, spec.hit_point);
        println!(
            "魔物【{}】の通常攻撃力は{}",
            monster_name, spec.attack.power
        );
        println!(
            "魔物【{}】の魔法攻撃力は{}\n",
            monster_name, spec.attack.magic.power
        );

        Monster {
            name: monster_name.to_string(),
            spec,
            attribute: attribute.to_string(),
        }
    }
}
