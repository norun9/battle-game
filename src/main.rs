mod battle;
use battle::Common;

fn main() {
    let mut brave: battle::Brave = Common::new();
    println!("{:?}", brave);
    let mut monster: battle::Monster = Common::new();
    println!("{:?}", monster);
}
