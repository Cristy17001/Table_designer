extern crate core;

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

    //loop {
    //    println!("╭───────────────────────────────────────────────╮");
    //    println!("│                 Table Designer                │");
    //    println!("│───────────────────────────────────────────────│");
    //    println!("│1. Insert new row                              │");
    //    println!("│───────────────────────────────────────────────│");
    //    println!("│2. Insert new row                              │");
    //    println!("│───────────────────────────────────────────────│");
    //    println!("│3. Insert row                                  │");
    //    println!("│───────────────────────────────────────────────│");
    //    println!("│4. Convert to print statement                  │");
    //    println!("╰───────────────────────────────────────────────╯");


    //}


    let mut t = Table {
        style: 5,
        auto_format: true,
        headers: vec!["name".to_owned(), "age".to_owned(), "email".to_owned(), "random".to_owned()],
        rows: Vec::new(),
        word_len: Vec::new(),
    };

    t.insert_row(vec!["Cristiano".to_owned(), "20".to_owned(), "cristianorocha170@gmail.com".to_owned(), "fffffff".to_owned()]);
    t.insert_row(vec!["Rocha".to_owned(), "20".to_owned(), "rocha170@gmail.com".to_owned(), "dasdasdadgdgdgd".to_owned()]);
    t.insert_row(vec!["Pedro".to_owned(), "20".to_owned(), "rocha170@gmail.com".to_owned(), "dkjanskdnaksd".to_owned()]);

    println!("{:?}", t.get_rows());
    t.print();
}
