#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::collections::{self, HashMap};
use std::io;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    name: String,
    id: String,
    password: String,
    money: f64,
    debt: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Bank {
    all_account: HashMap<String, Account>,
    count: i32,
}

fn main() {
    let mut clear: Command = if cfg!(target_os = "windows") {
        Command::new("cls")
    } else {
        Command::new("clear")
    };

    let mut stop: Command = if cfg!(target_os = "windows") {
        Command::new("pause")
    } else {
        Command::new("read")
    };

    let mut line = String::new();
    // hello();
    let mut flag: bool;
    println!("欢迎使用储蓄账户管理系统，是否从已备份的文件加载储户信息？(y/n)");

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("异常输入。");
        match line.as_str().trim() {
            "y" => {
                flag = true;
                break;
            }
            "n" => {
                flag = false;
                break;
            }
            _ => {
                println!("未匹配的输入，请重新尝试。");
                continue;
            }
        }
    }

    let mut bank = if flag {
    } else {
    };
}

impl Account {
    fn money_delta(&mut self, delta: f64) -> Result<String, String> {
        if self.money + delta < 0 as f64 {
            Err("当前账户余额不足！\n".to_string())
        } else {
            self.money += delta;
            Ok("已完成存读款操作。\n".to_string())
        }
    }

    fn debt_get(&mut self, debt: f64) -> Result<String, String> {
        match self.debt {
            Some(x) => Err(format!(
                "储户已有一笔{}的贷款，无法在已经存在贷款的前提下继续借贷。\n",
                self.debt.unwrap()
            )),
            None => {
                if debt <= 0 as f64 {
                    Err("非法数据。".to_string())
                } else {
                    self.debt = Some(debt);
                    Ok(format!("储户已经成功贷款 ￥{}。\n", debt))
                }
            }
        }
    }

    fn debt_pay(&mut self, delta: f64) -> Result<String, String> {
        if delta <= 0 as f64 {
            Err("非法输入。".to_string())
        } else if delta > self.money {
            Err("当前账户的余额不足以满足您输入的金额！".to_string())
        } else {
            if delta >= self.debt.unwrap() {
                self.money -= self.debt.unwrap();
                self.debt = None;
                Ok("已经偿还所有债务。".to_string())
            } else {
                self.money -= delta;
                self.debt = Some(self.debt.unwrap() - delta);
                Ok("已经偿还部分债务。".to_string())
            }
        }
    }

    fn account_display(&self) {
        println!("户主姓名：{}\n卡号：{}", &(self.name), &(self.id));
        println!("账户余额：{}", self.money);
        match self.debt {
            Some(x) => println!("负债：￥ {}", x),
            None => println!("当前户主无负债。"),
        }
    }
}

impl Bank {
    fn display_all(&self) {
        println!("\n以下为所有银行储户账户的信息\n");
        if self.all_account.len() <= 0 {
            println!("当前银行没有储户！");
            return;
        }
        for (m, n) in &self.all_account {
            n.account_display();
        }
    }

    fn get(&mut self, password: &str, id: &str) -> Result<&Account, String> {
        match self.all_account.get(id) {
            None => return Err("账户不存在。".to_string()),
            Some(x) => {
                if x.password == password {
                    return Ok(x);
                } else {
                    return Err("密码错误。".to_string());
                }
            }
        }
    }

    fn init_without_file() -> Bank{
        let mut temp = HashMap::new();
        temp.insert(
            "test".to_string(),
            Account {
                name: "test".to_string(),
                id: "mb12345".to_string(),
                password: "qwer1234".to_string(),
                money: 999.99,
                debt: None,
            },
        );
        Bank {all_account: temp, count: 1}
    }

    fn init_with_file() -> Bank{
        let mut file_path = String::new();
        let mut buf = String::new();
        println!("请输入存放用户数据的json文件的路径");
        io::stdin().read_line(&mut file_path).expect("非法输入。");
        match read_file(file_path.as_str(), &mut buf) {
            Ok(()) => {},
            _ => {panic!("无法打开文件！")},
        }

        serde_json::from_str(&buf).expect("json解析失败。")
    }
}

fn hello() {
    println!(
        "
    $$$$$      
    $:::$                  ____              _    
$$$$$:::$$$$$$            |  _ \\            | |   
$$::::::::::::::$         | |_) | __ _ _ __ | | __
$:::::$$$$$$$::::$        |  _ < / _` | '_ \\| |/ /
$::::$       $$$$$        | |_) | (_| | | | |   <
$::::$                    |____/ \\__,_|_| |_|_|\\ _\\
$::::$            
$:::::$$$$$$$$$   
$$::::::::::::$$          /\\/\\   __ _ _ __   __ _  __ _  ___
$$$$$$$$$:::::$          /    \\ / _` | '_ \\ / _` |/ _` |/ _ \\
         $::::$         / /\\/\\ \\ (_| | | | | (_| | (_| |  __/
         $::::$         \\/    \\/\\__,_|_| |_|\\__,_|\\__, |\\___|
$$$$$       $::::$                                |___/      
$::::$$$$$$$:::::$
$::::::::::::::$$ 
$$$$$$:::$$$$$   
   $:::$       
   $$$$$       
               
"
    );
}

fn help() {
    println!("(1) 向指定卡号的账户存取款");
    println!("(2) 查询指定卡号的账户的信息");
    println!("(3) 为指定卡号的账户办理贷款业务");
    println!("(4) 注销指定卡号的账户");
    println!("(5) 增加新的账户");
    println!("(6) 保存所有信息并退出");
}



fn read_file(file_path: &str, buf: &mut String) -> io::Result<()> {
    let mut f = std::fs::File::open(file_path)?;
    io::Read::read_to_string(&mut f, buf)?;
    Ok(())
}

