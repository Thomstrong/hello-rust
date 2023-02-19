use crate::garden::vegetables::Asparagus;
pub mod garden;

fn main() {
    // 不使用use，等价于 let plant = garden::vegetables::Asparagus {};
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
