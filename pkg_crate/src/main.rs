pub mod garden;
use crate::garden::vegetables::Asparagus;

use std::fmt::Result as fmtResult;
use std::io::Result as ioResult; // 同名包可以用as来重命名

use std::{self, cmp::Ordering, collections::HashMap}; // 可以使用花括号简写use前缀相同的包, self 表示引入前缀包本身

fn main() {
    // 不使用use，等价于 let plant = garden::vegetables::Asparagus {};
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
    test_fn();
}

fn test_fn() {}