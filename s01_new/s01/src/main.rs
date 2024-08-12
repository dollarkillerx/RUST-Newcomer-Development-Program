use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct User {
    password: String,
    balance: f64,
}

struct Bank {
    users: HashMap<String, User>,
}

impl Bank {
    fn new() -> Bank {
        Bank {
            users: HashMap::new(),
        }
    }

    fn register(&mut self, username: String, password: String) {
        if self.users.contains_key(&username) {
            println!("用户名已存在！");
        } else {
            self.users.insert(
                username.clone(),
                User {
                    password,
                    balance: 0.0,
                },
            );
            println!("用户 {} 注册成功！", username);
        }
    }

    fn login(&self, username: &str, password: &str) -> bool {
        if let Some(user) = self.users.get(username) {
            if user.password == password {
                println!("登录成功！");
                true
            } else {
                println!("密码错误！");
                false
            }
        } else {
            println!("用户不存在！");
            false
        }
    }

    fn deposit(&mut self, username: &str, amount: f64) {
        if let Some(user) = self.users.get_mut(username) {
            user.balance += amount;
            println!("入金成功！当前余额：{}", user.balance);
        } else {
            println!("用户不存在！");
        }
    }

    fn transfer(&mut self, from: &str, to: &str, amount: f64) {
        if !self.users.contains_key(to) {
            println!("接收用户不存在！");
            return;
        }

        if let Some(from_user) = self.users.get_mut(from) {
            if from_user.balance < amount {
                println!("余额不足！");
            } else {
                from_user.balance -= amount;
                if let Some(to_user) = self.users.get_mut(to) {
                    to_user.balance += amount;
                }
                println!("转账成功！{} 向 {} 转账 {} 元", from, to, amount);
            }
        } else {
            println!("发送用户不存在！");
        }
    }

    fn show_all_users(&self) {
        for (username, user) in &self.users {
            println!("用户: {}, 余额: {}", username, user.balance);
        }
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut logged_in_user: Option<String> = None;

    loop {
        if logged_in_user.is_none() {
            println!("请选择操作: 1. 注册 2. 登录 3. 查询所有用户 4. 退出");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("读取失败");
            let choice: u32 = choice.trim().parse().expect("请输入数字");

            match choice {
                1 => {
                    let (username, password) = get_credentials();
                    bank.register(username, password);
                }
                2 => {
                    let (username, password) = get_credentials();
                    if bank.login(&username, &password) {
                        logged_in_user = Some(username);
                    }
                }
                3 => bank.show_all_users(),
                4 => break,
                _ => println!("无效操作，请重新选择"),
            }
        } else {
            println!("请选择操作: 1. 入金 2. 转账 3. 注销 4. 退出");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("读取失败");
            let choice: u32 = choice.trim().parse().expect("请输入数字");

            match choice {
                1 => {
                    let amount = get_input("请输入入金额: ")
                        .trim()
                        .parse::<f64>()
                        .expect("请输入有效金额");
                    if let Some(ref username) = logged_in_user {
                        bank.deposit(username, amount);
                    }
                }
                2 => {
                    let to = get_input("请输入接收用户名: ");
                    let amount = get_input("请输入转账金额: ")
                        .trim()
                        .parse::<f64>()
                        .expect("请输入有效金额");
                    if let Some(ref from) = logged_in_user {
                        bank.transfer(from, &to, amount);
                    }
                }
                3 => {
                    println!("已注销 {} 的登录。", logged_in_user.take().unwrap());
                }
                4 => break,
                _ => println!("无效操作，请重新选择"),
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取失败");
    input.trim().to_string()
}

fn get_credentials() -> (String, String) {
    let username = get_input("请输入用户名: ");
    let password = get_input("请输入密码: ");
    (username, password)
}
