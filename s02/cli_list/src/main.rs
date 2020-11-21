#[macro_use]
extern crate lazy_static;

use std::cell::Cell;
use std::collections::HashMap;
use std::io;
use std::sync::Mutex;

lazy_static! {
    static ref DB: Mutex<HashMap<String, Account>> = {
        let mut db = HashMap::new();
        db.insert("dollarkiller@dollarkiller.com".to_string(), Account{
            email: String::from("dollarkiller@dollarkiller.com"),
            password: String::from("dollarkiller"),
            balance: Cell::new(12.0),
        });
        Mutex::new(db)
    };
}

#[derive(Debug)]
struct Account {
    email: String,
    password: String,
    balance: Cell<f32>,
}

fn main() {
    println!("欢迎来到实力至上主义银行");
    let mut input = String::new();
    'ma:
    loop {
        input.clear();
        top1_menu();
        if io::stdin().read_line(&mut input).is_err() {
            println!("you input error!!! ");
            continue;
        }
        match input.as_str().trim() { // .trim 去空白符
            "1" => {
                open_account();
            }
            "2" => {
                login();
            }
            "exit" => {
                println!("感谢你使用我们的系统");
                break 'ma;
            }
            _ => {
                println!("you input error!!! {:?}", input);
                continue;
            }
        };
    }
}

fn top1_menu() {
    println!(r#""================""#);  // r#"  "#
    println!("1. 开户");
    println!("2. 登录");
    println!("exit. 退出");
    println!("====================");
    println!("请输入: ");
}

fn open_account() {
    {
        println!("现有账户: {:?}", DB.lock());  // 使用锁的地方 请使用小的生命周期
    }


    println!("请输入Email: ");
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("you input error!!! {}", input);
        return;
    }
    let email = input.clone().trim().to_string();
    println!("请输入密码: ");
    input.clear();
    if io::stdin().read_line(&mut input).is_err() {
        println!("you input error!!! {}", input);
        return;
    }
    let passwd = input.clone().trim().to_string();
    let ac = Account {
        email: email.clone(),
        password: passwd,
        balance: Cell::new(0.0),
    };
    {
        println!("ac: {:?}", ac);
        DB.lock().unwrap().insert(email, ac);
    }
}

fn login() {
    {
        println!("现有账户: {:?}", DB.lock());  // 使用锁的地方 请使用小的生命周期
    }

    let mut input = String::new();
    let user;
    let password;
    println!("请输入你的账户: ");
    io::stdin().read_line(&mut input).expect("what are you fucking input?");
    user = input.clone().trim().to_string();
    println!("请输入你的密码: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("what are you fucking input?");
    password = input.clone().trim().to_string();

    {
        match DB.lock().unwrap().get(&user) {
            None => {
                println!("user: {:?} , passwd: {:?}", user, password);
                println!("None");
            }
            Some(data) => {
                if data.password == password {
                    hall(data);
                } else {
                    println!("密码或者用户名错误");
                    return;
                }
            }
        };
    }
}

fn hall(account: &Account) {
    println!("欢迎来到银行大厅");
    let mut input = String::new();
    'ma:
    loop {
        input.clear();
        hall_menu();
        io::stdin().read_line(&mut input).unwrap();
        match input.as_str().trim() {
            "1" => {
                println!("You Balance: {}", account.balance.get());
            }
            "2" => {
                println!("请输入存储金额: ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let p: f32 = match input.trim().parse() {
                    Ok(i) => i,
                    Err(_) => {
                        println!("you input error!!!");
                        continue;
                    }
                };
                account.balance.set(account.balance.get() + p);
                println!("存款成功");
            }
            "3" => {
                println!("请输入取款金额: ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let p: f32 = match input.trim().parse() {
                    Ok(i) => i,
                    Err(_) => {
                        println!("you input error!!!");
                        continue;
                    }
                };
                if p > account.balance.get() {
                    println!("余额不足");
                    continue;
                }
                account.balance.set(account.balance.get() - p);
                println!("取款成功");
            }
            "exit" => {
                println!("感谢你使用我们的系统");
                break 'ma;
            }
            _ => {
                println!("you input error!!! {}", input);
                continue;
            }
        };
    }
}

fn hall_menu() {
    println!("======Menu=====");
    println!("1. 查询余额");
    println!("2. 存款");
    println!("3. 取款");
    println!("exit: Exit");
    println!("===============");
}