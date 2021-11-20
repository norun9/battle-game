use std::env;

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
        _ => println!("Something else!"),
    }
}
