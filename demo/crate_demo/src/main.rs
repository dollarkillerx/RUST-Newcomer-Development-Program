use crate_demo::*;
extern crate num_cpus;

fn main() {
    println!("Hello, world!");
    let conf = conf::PoemConfig::read_config();
    println!("{:#?}",conf);
}
