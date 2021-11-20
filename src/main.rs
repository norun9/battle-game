use std::env;

mod battle;

fn main() {
    let args: Vec<String> = env::args().collect();
    let select: &str;
    if args.len() > 1 {
        select = &args[1]
    } else {
        select = "nothing"
    }
    match select {
        "battle" => {
            let monster: battle::Monster = battle::Monster::new();
            println!("{:?}", monster);
        }
        _ => println!("Something else!"),
    }
}
