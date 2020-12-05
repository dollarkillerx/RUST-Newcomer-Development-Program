use std::io::stdin;

use crate::core::Core;

use super::*;

pub struct Back<'a> {
    core: &'a mut Core,
}

impl<'a> Back<'a> {
    pub fn new(core: &'a mut Core) -> Back {
        Back {
            core
        }
    }
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!("欢迎来到大雄银行");
        let mut input = String::new();
        'p:
        loop {
            input.clear();
            self.welcome();
            stdin().read_line(&mut input)?;
            match input.trim() {
                "1" => {
                    self.create_account()?;
                }
                "2" => {
                    self.login()?;
                }
                "exit" => {
                    println!("感谢你使用我们的系统");
                    break 'p;
                }
                _ => {
                    println!("you input error!!! {:?}", input);
                    continue;
                }
            }
        }

        Ok(())
    }

    fn welcome(&self) {
        println!("1. CreatAccount");
        println!("2. Login");
        println!("exit. Exit");
    }

    fn create_account(&self) -> Result<(), Box<dyn Error>> {
        let mut input = String::new();
        println!("请输入Email: ");
        stdin().read_line(&mut input)?;
        println!("请输入Password: ");
        let account = input.clone().trim().to_string();
        input.clear();
        stdin().read_line(&mut input)?;
        let password = input.trim().to_string();

        self.core.create_account(account, password)
    }

    fn login(&mut self) -> Result<(), Box<dyn Error>> {
        let mut input = String::new();
        println!("请输入Email: ");
        stdin().read_line(&mut input)?;
        let account = input.clone().trim().to_string();
        input.clear();
        stdin().read_line(&mut input)?;
        let password = input.trim().to_string();

        &self.core.login(account, password)?;
        self.hall()
    }

    fn hall(&mut self) -> Result<(), Box<dyn Error>> {
        println!("欢迎来到银行大厅");
        let mut input = String::new();
        'p:
        loop {
            input.clear();
            self.hall_menu();
            stdin().read_line(&mut input)?;
            match input.trim() {
                "1" => {
                    let b = &self.core.balance_inquiry()?;
                    println!("You Balance: {}", b);
                }
                "2" => {
                    input.clear();
                    println!("请输入存储金额: ");
                    stdin().read_line(&mut input)?;
                    println!("{:?}",input.trim());
                    let money: f32 = input.trim().parse()?;
                    &self.core.deposits(money)?;
                    println!("存款成功");
                }
                "3" => {
                    input.clear();
                    println!("请输入取款金额: ");
                    stdin().read_line(&mut input)?;
                    let money: f32 = input.trim().parse()?;
                    match &self.core.withdrawal(money) {
                        Ok(()) => {
                            println!("存款成功");
                        }
                        Err(e) => {
                            println!("{}", e.to_string());
                        }
                    }
                }
                "exit" => {
                    break 'p;
                }
                _ => {
                    println!("you input error!!! {}", input);
                    continue;
                }
            }
        }
        Ok(())
    }

    fn hall_menu(&self) {
        println!("======Menu=====");
        println!("1. 查询余额");
        println!("2. 存款");
        println!("3. 取款");
        println!("exit: Exit");
        println!("===============");
    }
}