use std::cmp::Ordering;
use std::io;

use rand::Rng;
fn main() {
    println!("猜猜数字!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("秘密数字是{secret_number},不许告诉别人哦~");

    println!("你的幸运数字？");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");

        let cmd = guess.trim();

        match cmd {
            "quit" => {
                println!("收到，结束游戏，欢迎下次再来哟~");
                break;
            }
            _ => {}
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入有效数字！");
                continue;
            }
        };

        println!("你猜的数字是: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了哟"),
            Ordering::Greater => println!("大了哟"),
            Ordering::Equal => {
                println!("呀，被你猜到了，你赢了！");
                break;
            }
        }
    }
}
