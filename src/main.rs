use std::io;
use crate::table::Table;

mod table;

fn main() {
    println!("  ______      __    __        ____            _                      ");
    println!(" /_  __/___ _/ /_  / /__     / __ \\___  _____(_)___ _____  ___  _____");
    println!("  / / / __ `/ __ \\/ / _ \\   / / / / _ \\/ ___/ / __ `/ __ \\/ _ \\/ ___/");
    println!(" / / / /_/ / /_/ / /  __/  / /_/ /  __(__  ) / /_/ / / / /  __/ /    ");
    println!("/_/  \\__,_/_.___/_/\\___/  /_____/\\___/____/_/\\__, /_/ /_/\\___/_/     ");
    println!("                                            /____/                   ");
    println!("CopyrightⒸ 2023 Cristiano Rocha.");
    println!("All rights reserved.");
    println!();

    let mut t = Table{
        style: 1,
        ..Default::default()
    };

    let mut input:String = String::new();
    println!("Introduce Headers for the table (ex: Name,Age,Email): ");
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    let mut l_words: Vec<&str> = input.split(',').collect();
    for w in l_words {
        t.add_header(w.trim().to_string());
    }

    loop {
        println!("╭───────────────────────────────────────────────╮");
        println!("│                 Table Designer                │");
        println!("│───────────────────────────────────────────────│");
        println!("│1. Insert Row                                  │");
        println!("│───────────────────────────────────────────────│");
        println!("│2. Print Without Centering                     │");
        println!("│───────────────────────────────────────────────│");
        println!("│3. Print and Center Table                      │");
        println!("│───────────────────────────────────────────────│");
        println!("│4. Change Style                                │");
        println!("│───────────────────────────────────────────────│");
        println!("│5. Convert to print statement                  │");
        println!("│───────────────────────────────────────────────│");
        println!("│Q. Quit                                        │");
        println!("╰───────────────────────────────────────────────╯");

        let mut user_input = String::new();
        println!("=> ");
        std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let user_input_char: char = user_input.trim().parse().expect("Please type a single character");

        match user_input_char {
            '1' => t.input_row(),
            '2' => {
                t.auto_format = false;
                println!("{}", t.design_table());
            },
            '3' => {
                t.auto_format = true;
                println!("{}", t.design_table());
            },
            '4' => t.change_style(),
            '5' => t.print_statements(),
            'Q' | 'q' => break,
            _ => println!("Cannot recognise character!")
        };
    }
}
