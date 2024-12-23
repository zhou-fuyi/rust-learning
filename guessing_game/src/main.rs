use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess you number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret number is {}", secret_number);

    loop {

        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // 遮蔽, 即支持使用相同的变量名, 但是指向的是不同的变量, 英文概念 -> shadowing
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
