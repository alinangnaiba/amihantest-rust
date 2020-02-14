use std::io;
use std::io::Write;
mod letters;

fn main() {
    let letter_choice = get_input_letter();

    let num_choice = get_input_number();

    letters::draw_letter(&letter_choice, num_choice);
}

fn is_valid_choice(choice: &String) -> bool {
    let user_choice = choice.trim().to_uppercase();

    user_choice == "O" || user_choice == "X" || user_choice == "Y" || user_choice == "Z"
}

fn is_valid_number(num: u32) -> bool { 
    num > 2 && num % 2 > 0
}

fn display_letter_prompt() {
    print!("Enter a choice [O, X, Y, Z]: ");
    io::stdout().flush().unwrap();
}

fn display_num_prompt() {
    print!("Enter a non negative odd integer: ");
    io::stdout().flush().unwrap();
}

fn get_input_letter() -> String {
    display_letter_prompt();
    let mut letter = String::new();
    io::stdin().read_line(&mut letter)
            .expect("Error getting input.");
    while !is_valid_choice(&letter){
        display_letter_prompt();
        letter = String::new();
        io::stdin().read_line(&mut letter)
            .expect("Error getting input.");
    }

    letter.to_uppercase()
}

fn get_input_number() -> u32 {
    let mut num_text = String::new();
    display_num_prompt();
    io::stdin().read_line(&mut num_text)
        .expect("Error getting input.");
    let num_choice = num_text.trim();
    let mut num: u32 = num_choice.parse()
    .expect("Please type a number!");   
    while !is_valid_number(num) {
        display_num_prompt();
        num_text = String::new();
        io::stdin().read_line(&mut num_text)
        .expect("Error getting input.");
        let num_choice = num_text.trim();
        num = num_choice.parse()
        .expect("Please type a number!");   
    }

    num
}

