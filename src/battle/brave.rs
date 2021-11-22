extern crate rand;
use super::common;

#[derive(Debug)]
pub struct Brave {
    pub name: String,
    pub spec: common::Spec,
    pub attribute: String,
}

impl common::Common for Brave {
    fn new() -> &'static mut Brave {
        let mut name = String::new();
        println!("Please decide the name of brave");
        std::io::stdin().read_line(&mut name).ok();
        println!("Brave {} Defeat the monster!!", name);

        // Magic constructor :
        let attribute = Self::get_attribute();
        let magic = common::Magic::new(attribute);

        // Attack constroctor :
        let attack: common::Attack = common::Attack::new(magic);

        // Spec constructor :
        let spec: common::Spec = common::Spec::new(attack);

        &mut Brave {
            name,
            spec,
            attribute: attribute.to_string(),
        }
    }
}
