// extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(1..101); // [1,101)
    // let n = n.to_string();
    println!("猜数!: {}", n);
    loop {
        println!("猜测一个数");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取");
        println!("猜测的数为: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("输入错误!!!");
                continue;
            },
        };
        // if guess == n {
        //     println!("猜测正确");
        //     break;
        // }
        // println!("猜测错误!!!");

        match guess.cmp(&n) {
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Equal => {
                println!("you wind!");
                break;
            }
            Ordering::Greater => {
                println!("too big!");
            }
        }
    }

    println!("Over");
}
