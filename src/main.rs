#![allow(unused)]

fn main() {
    let mut  line = String::new();

}

struct Account {
    name: String,
    id: String,
    money: f64,
    debt: Option<f64>,
}



impl Account {
    fn money_delta(&mut self, delta: f64) -> Result<&str, &str> { 
        if self.money + delta < 0 as f64 {
            Err("当前账户余额不足！\n")
        }
        else {
            self.money += delta;
            Ok("已完成存读取钱款操作。\n")
        }
    }

    fn debt_pay(&mut self, delta: f64) -> Result<&str, &str> {
        if delta <= 0 as f64 {
            Err("非法输入。")    
        }
        else if delta > self.money {
            Err("当前账户的余额不足以满足您输入的金额！")
        }
        else {
            if delta >= self.debt.unwrap() {
                self.money -= self.debt.unwrap();
                self.debt = None;
                Ok("已经偿还所有债务。")
            }
            else {
                self.money -= delta;
                self.debt = Some(self.debt.unwrap() - delta);
                Ok("已经偿还部分债务。")
            }
            
        }
    }

}

impl Account {
    fn display_account(&self) {
        println!("户主姓名：{}\n卡号：{}", &(self.name), &(self.id));
        println!("账户余额：{}", self.money);
        match self.debt {
            Some(x) => println!("负债：￥ {}", x),
            None => println!("当前户主无负债。"),
        }
    }
    
}