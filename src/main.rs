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

    fn set_color(&mut self, color:String)
    {
        self.color = String::from(color);
    }

    fn change_value(&mut self)
    {
        self.value = rand::thread_rng().gen_range(1..=6);
    }
    
}

fn main() {

    let mut dice_1 = Dice::run(String::from("White"));

    'app_main_loop: loop {
        println!("throw_>");
        let mut user_input:String = String::new();
        io::stdin().read_line(&mut user_input).expect("Err reading your input");
        dice_1.change_value();
        println!("Dice color: {}, & Value: {}",dice_1.color, dice_1.value);
    }
}
