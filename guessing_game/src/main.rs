use rand::Rng;  // Rng 是一个trait，定义了随机数生成器应该实现的方法
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    //rand::thread_rng 函数提供实际使用的随机数生成器:它位于当前线程的本地环境，从操作系统提取seed
    //调用随机数生成器 gen_range 方法，获取一个范围表达式(range expression)作为参数，生成在此范围的一个随机数
    //范围表达式采用 start..=end , 范围包含上下端点
    let secret_number = rand::thread_rng().gen_range(1..=100);  

//    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
