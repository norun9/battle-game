extern crate rand;
use super::common;

#[derive(Debug)]
pub struct Brave {
    pub name: String,
    pub spec: common::Spec,
    pub attribute: String,
}

impl common::Common for Brave {
    fn new() -> Brave {
        let mut name = String::new();
        println!("【 勇者 】の名前を決めてください。\n");
        std::io::stdin().read_line(&mut name).ok();
        println!("");

        // Magic constructor :
        let attribute = Self::get_attribute();
        let magic = common::Magic::new(attribute);

        // Attack constroctor :
        let attack: common::Attack = common::Attack::new(magic);

        // Spec constructor :
        let spec: common::Spec = common::Spec::new(attack);

        Brave {
            name,
            spec,
            attribute: attribute.to_string(),
        }
    }
}
