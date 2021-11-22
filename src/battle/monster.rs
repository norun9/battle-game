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
        let spec: common::Spec = common::Spec::new(attack);

        let monster_names = [
            "バハムート",
            "ティアマト",
            "カリュブディス",
            "サボテンダー",
            "ベヒーモス",
        ];
        let number: usize = rand::thread_rng().gen_range(0, 5);
        let monster_name = monster_names[number];
        println!("魔物【 {} 】が現れた！\n", monster_name);
        Monster {
            name: monster_name.to_string(),
            spec,
            attribute: attribute.to_string(),
        }
    }
}
