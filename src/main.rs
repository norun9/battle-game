use std::env;

mod battle;
mod omikuji;

fn main() {
    let args: Vec<String> = env::args().collect();
    let select: &str;
    if args.len() > 1 {
        select = &args[1]
    } else {
        select = "nothing"
    }
    match select {
        "omikuji" => omikuji::run(),
        "battle" => {
            let scene: battle::Scene = battle::Scene::new();
            println!("{:?}", scene);
        }
        _ => println!("Something else!"),
    }
}
