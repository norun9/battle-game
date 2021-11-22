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
    fn new() -> &'static mut Monster {
        // Magic constructor :
        let attribute = Self::get_attribute();
        let magic = common::Magic::new(attribute);

        // Attack constroctor :
        let attack: common::Attack = common::Attack::new(magic);

        // Spec constructor :
        let spec: common::Spec = common::Spec::new(attack);

        let monster_names = ["Bahamut", "Tiamat", "Charybdis", "Weapon mouse", "Redicle"];
        let number: usize = rand::thread_rng().gen_range(0, 5);
        let monster: Monster = Monster {
            name: monster_names[number].to_string(),
            spec,
            attribute: attribute.to_string(),
        };
        println!("{} has appeared!!", monster.name);
        &mut monster
    }
}

impl Monster {
    pub fn action() {}
}
