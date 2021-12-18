use rand::Rng;
use std::io;
fn main() {
    'main: loop{
        let mut x = 0;
        let tup = ["ㄤ奈","SnowFey","我可能很廢","afaf","雨崎","虎山大三專主義帝國","LTurret","ペットリー","海獺"];
        println!("請輸入要抽幾次:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("讀取失敗");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your input is not a number!");
                continue
            },
        };
        loop{
            if x == input{
                break
            }else if input == 100{
                break 'main
            }
            let reporter = rand::thread_rng().gen_range(0..8);
            println!("今天的主講者是: {}",tup[reporter]);
            x += 1;
        }
    }
}
