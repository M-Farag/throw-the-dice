use std::io;
use rand::{self, Rng};

struct Dice {
    color:String,
    value: u8
}

impl Dice {

    fn run(color:String) -> Self
    {
        Self { color, value: rand::thread_rng().gen_range(1..=6) }
    }
    
}

fn main() {

    

    'app_main_loop: loop {
        let dice_1 = Dice::run(String::from("White"));
        println!("throw_>");
        let mut user_input:String = String::new();
        io::stdin().read_line(&mut user_input).expect("Err reading your input");
        
        let exit_code: char = user_input.chars().take(1).last().unwrap();
        
        match exit_code as u8 {
            88 => break 'app_main_loop,
            120 => break 'app_main_loop,
            _ => ()
        };
        

        println!("Dice color: {}, & Value: {}",dice_1.color, dice_1.value);
    }
}
