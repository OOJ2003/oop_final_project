#![allow(unused)]

use crossterm::ExecutableCommand;
use crossterm::terminal::{Clear, ClearType};
use serde::{Deserialize, Serialize};

use std::collections::{self, HashMap};
use std::io::{self, stdin, stdout, Write};
use std::process::Command;

#[macro_export]
macro_rules! 打印消息 {
    ($x:expr) => {
        match $x {
            Ok(y) => println!("{}", y),
            Err(y) => println!("错误：{}", y),
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
struct 账户 {
    姓名: String,
    卡号: String,
    密码: String,
    余额: f64,
    债务: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct 银行 {
    账户哈希表: HashMap<String, 账户>,
    管理员密码: String,
}

fn main() {
    hello();
    let mut line = String::new();
    let mut flag: bool;
    println!("欢迎使用储蓄账户管理系统，是否从已备份的文件加载储户信息？(y/n)");

    
    loop {
        line.clear();
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
        银行::有文件初始化()
    } else {
        银行::无文件初始化()
    };
    println!("初始化已完成。\n\n");

    event_loop(bank);
}

impl 账户 {
    fn 余额变更(&mut self, money: f64) -> Result<&mut Self, &str> {
        if (self.余额 + money) < 0.0 {
            Err("当前账户余额不足！")
        } else {
            self.余额 += money;
            Ok(self)
        }
    }

    fn 获取贷款(&mut self) -> Result<&str, &str> {
        match self.债务 {
            None => {}
            Some(x) => return Err("已存在贷款，无法在还清贷款前继续借贷。"),
        }
        let y = get_a_f64("请输入贷款金额。")?;
        if y <= 0.0 {
            Err("非法数据。")
        } else {
            self.债务 = Some(y);
            Ok("贷款成功")
        }
    }

    fn 偿还贷款(&mut self) -> Result<&str, &str> {
        let money = get_a_f64("请输入还款金额。")?;
        if money < 0.0 {
            Err("非法数据。")
        } else if self.余额 >= money {
            if money>= self.债务.unwrap() {
                self.余额 -= self.债务.unwrap();
            }
            else {
            self.余额 -= money;
            }
            if self.债务.unwrap() == 0.0 {
                self.债务 = None;
            }
            Ok("还款成功。")
        } else {
            Err("当前账户的余额不足以满足您输入的还款金额！")
        }
    }

    fn 输出账户信息(&self) {
        println!("户主姓名：{}\n卡号：{}", &(self.姓名), &(self.卡号));
        println!("账户余额：{}", self.余额);
        match self.债务 {
            Some(x) => println!("负债：￥ {}", x),
            None => println!("当前户主无负债。\n"),
        }
    }

    fn 新建账户() -> 账户 {
        let mut name = String::new();
        println!("请输入姓名。");
        io::stdin().read_line(&mut name).expect("异常输入。");

        let mut id = String::new();
        println!("请输入卡号。");
        io::stdin().read_line(&mut id).expect("异常输入。");

        let mut password = String::new();
        println!("请输入密码。");
        io::stdin().read_line(&mut password).expect("异常输入。");

        账户 {
            姓名: name.trim().to_string(),
            卡号: id.trim().to_string(),
            密码: password.trim().to_string(),
            余额: 0.0,
            债务: None,
        }
    }
}

impl 银行 {
    fn 输出所有账户信息(&self) -> Result<&str, &str> {
        println!("请输入银行管理员密码。");
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("异常输入，程序将终止运行。");

        if self.管理员密码.as_str() != password.trim() {
            return Err("管理员密码错误。");
        }

        println!("\n以下为所有银行储户账户的信息\n");
        if self.账户哈希表.len() <= 0 {
            return Err("当前系统内没有储户信息！");
        }
        for (m, n) in &self.账户哈希表 {
            n.输出账户信息();
        }

        Ok("成功。")
    }

    fn 获取账户可变引用(&mut self) -> Result<&mut 账户, &str> {
        let mut id = String::new();
        println!("请输入卡号。");
        io::stdin().read_line(&mut id).expect("异常输入。");

        let mut password = String::new();
        println!("请输入密码。");
        io::stdin().read_line(&mut password).expect("异常输入。");

        match self.账户哈希表.get_mut(id.trim()) {
            None => return Err("账户不存在。"),
            Some(x) => {
                if x.密码 == password.trim() {
                    return Ok(x);
                } else {
                    return Err("密码错误。");
                }
            }
        }
    }

    fn 无文件初始化() -> 银行 {
        let mut temp = HashMap::new();
        temp.insert(
            "mb12345".to_string(),
            账户 {
                姓名: "test".to_string(),
                卡号: "mb12345".to_string(),
                密码: "qwer1234".to_string(),
                余额: 999.99,
                债务: None,
            },
        );
        银行 {
            账户哈希表: temp,
            管理员密码: "admin".to_string(),
        }
    }

    fn 有文件初始化() -> 银行 {
        let mut file_path = String::new();
        let mut buf = String::new();
        println!("请输入存放用户数据的json文件的路径");
        io::stdin().read_line(&mut file_path).expect("非法输入。");
        match read_file(file_path.as_str(), &mut buf) {
            Ok(()) => {}
            _ => {
                panic!("无法打开文件！")
            }
        }

        serde_json::from_str(&buf).expect("json反序列化失败。")
    }

    fn 存取款业务(&mut self) -> Result<&str, &str> {
        let account = self.获取账户可变引用()?;
        let delta = get_a_f64("请输入存取款金额。")?;
        match account.余额变更(delta) {
            Ok(x) => Ok("存取款成功。"),
            Err(_) => todo!(),
        }
    }

    fn 输出单个账户信息(&mut self) -> Result<&str, &str> {
        let account = self.获取账户可变引用()?;
        account.输出账户信息();
        Ok("成功。")
    }

    fn 贷款业务(&mut self) -> Result<&str, &str> {
        let account = self.获取账户可变引用()?;
        account.获取贷款()
    }

    fn 还贷业务(&mut self) -> Result<&str, &str> {
        let account = self.获取账户可变引用()?;
        account.偿还贷款()
    }

    fn 销户业务(&mut self) -> Result<&str, &str> {
        let account = self.获取账户可变引用();
        let what = [
            "确定吗？(y/n)",
            "您真的确定办理销户业务吗？(y/n)",
            "该操作不可逆！确定仍然办理销户业务吗？(y/n)",
        ];
        let mut line = String::new();
        for i in what {
            println!("{}", i);
            io::stdin()
                .read_line(&mut line)
                .expect("异常输入，程序将终止运行。");
            match line.trim() {
                "y" => {}
                "n" => return Err("销户业务取消。"),
                _ => return Err("未匹配的输入，销户业务取消。"),
            }
            line.clear();
        }
        Ok("销户业务办理成功，您的储户信息已被移出本行管理系统。")
    }

    fn 修改账户密码(&mut self) -> Result<&str, &str> {
        let account = self.获取账户可变引用()?;
        println!("进入密码修改模式。");
        println!("请输入您的新密码。");
        let mut pass = String::new();
        io::stdin()
            .read_line(&mut pass)
            .expect("异常输入，程序将终止运行。");

        println!("请重新输入密码。");
        let mut pass_prove = String::new();
        io::stdin()
            .read_line(&mut pass_prove)
            .expect("异常输入，程序将终止运行。");

        if pass.as_str() == pass_prove.as_str() {
            account.密码 = pass;
            Ok("密码修改成功，请您牢记你的新密码。")
        } else {
            Err("两次输入的结果不匹配。")
        }
    }

    fn 保存(&self) -> Result<&str, &str> {
        let mut s = String::new();
        println!("请输入存放用户数据的json文件的路径。");
        io::stdin()
            .read_line(&mut s)
            .expect("异常输入，程序将终止运行。");

        let mut file = match write_file(s.as_str()) {
            Ok(x) => x,
            Err(_) => return Err("打开保存文件失败！程序将终止运行。"),
        };

        let serialized = match serde_json::to_string(self) {
            Ok(x) => x,
            Err(_) => return Err("序列化为json失败！程序将终止运行。"),
        };

        file.write(serialized.as_bytes())
            .expect("将数据写入文件中发生意外，程序将强制退出。");
        Ok("保存成功，程序将退出运行。")
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
    println!("(1) 向指定卡号的账户存取款。");
    println!("(2) 查询指定卡号的账户的信息。");
    println!("(3) 为指定卡号的账户办理贷款业务。");
    println!("(4) 为指定卡号的账户办理还贷业务。");
    println!("(5) 为指定卡号的账户办理销户业务。");
    println!("(6) 注册一个新的账户。");
    println!("(7) 为指定卡号的账户修改登陆密码。");
    println!("(8) 输出所有账户的信息。");
    println!("(9) 保存所有信息并退出。");
}

fn read_file(file_path: &str, buf: &mut String) -> io::Result<()> {
    let mut f = std::fs::File::open(file_path.trim())?;
    io::Read::read_to_string(&mut f, buf)?;
    Ok(())
}

fn write_file(file_path: &str) -> io::Result<std::fs::File> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path.trim())?;
    Ok(file)
}

fn get_a_f64(msg: &str) -> Result<f64, &str> {
    println!("{}", msg);
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("异常输入，程序将终止运行。");
    match s.trim().parse::<f64>() {
        Ok(x) => Ok(x),
        Err(_) => Err(msg),
    }
}

fn event_loop(mut bank: 银行) {
    let mut line = String::new();
    let mut stdout = stdout();
    loop {
        stdout.execute(Clear(crossterm::terminal::ClearType::All));
        help();
        println!("输入数字以继续！");
        line.clear();
        println!("输入数字进行对应操作。\n");
        io::stdin().read_line(&mut line).expect("异常输入。");
        match line.trim() {
            "1" => {
                //存取款业务
                打印消息!((&mut bank).存取款业务());
            }
            "2" => {
                //查询账户信息
                打印消息!((&mut bank).输出单个账户信息());
            }
            "3" => {
                //申请贷款业务
                打印消息!((&mut bank).贷款业务());
            }
            "4" => {
                //还贷业务
                打印消息!((&mut bank).还贷业务());
            }
            "5" => {
                //销户业务
                打印消息!((&mut bank).销户业务())
            }
            "6" => {
                //开户业务
                let mut account = 账户::新建账户();
                bank.账户哈希表.insert(account.姓名.clone(), account);
            }
            "7" => {
                //修改账户密码
                打印消息!((&mut bank).修改账户密码());
            }
            "8" => {
                //显示所有账户信息
                打印消息!(bank.输出所有账户信息());
            }
            "9" => {
                //保存并退出
                打印消息!(bank.保存());
                return;
            }
            _ => println!("未匹配的输入，请重新输入选项！"),
        }

        println!("按下 “Enter” 以继续。");
        io::stdin().read_line(&mut line).expect("异常输入。");
    }
}
