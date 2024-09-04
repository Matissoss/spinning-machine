use rand::Rng;
use std::io;
use std::fs;

fn main() {
    loop{
        println!("Spin? y/N | s for money");
        
        let mut input : String = "".to_string();
        io::stdin().read_line(&mut input).expect("Error reading line");
        if input.trim() != "y" && input.trim() != "s"{
            break;
        }
        else if input.trim() == "s"
        {
            let val = fs::read_to_string("score.txt");
            match val 
            {
                Ok(v) =>{
                    println!("You have: {}", v);
                }
                Err(_) =>{
                    println!("You have: 0")
                }
            }
        }
        std::process::Command::new("clear").status().unwrap();
        let nums : Vec<u8> = vec![rand::thread_rng().gen_range(1..=9), rand::thread_rng().gen_range(0..=9), rand::thread_rng().gen_range(0..=9)];
        println!("{}", machine(&nums[0], &nums[1], &nums[2]));
        println!("{}", check_result(&nums[0], &nums[0], &nums[2]));
    }
}

fn machine(number1 : &u8, number2 : &u8, number3 : &u8) -> String{
    format!(
        "
         ___________________
        /  |                \\
        |  |________________|
        |  |                |
        |  |   {}   {}   {}    |
        |  |________________|
        |  |                |
        |  |                |
        |  |    Coin []     |
        \\  |                |
         \\ |                |
          \\|________________| 
        "
    , number1, number2, number3)
} 

fn check_result<'a >(number1 : &'a u8, number2 : &'a u8, number3 : &'a u8) -> &'a str{
    if number1 == number2 && number2 == number3 && number1 == number3 && number1 == number3{
        write(*number1 as u128);
        return "Jackpot!";
    }
    else{
        return "Numbers don't match!";
    }
    
}
        
fn write(num : u128){
    let content  = fs::read_to_string("score.txt");
        let mut con : String = String::from("");
        match content{
            Ok(value) => {
                con = value;
            }
            Err(_) => {
                fs::write("score.txt", "0").expect("Error writing file");
            }
        }
        
        let final_content = con.trim().parse();
        let final_con: u128;
        match final_content 
        {
            Ok(val) => {
                final_con = val;
            }
            Err(_) => {
                final_con = 0;
            }
        }
        let fcon: u128 = final_con + num;
        fs::write("score.txt", &fcon.to_string()).expect("Error with writing");
}

        
