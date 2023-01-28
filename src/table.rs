use std::fmt::format;
use std::io::repeat;

#[derive(Default, Debug, PartialEq)]
pub struct Table {
    pub style: i32,
    pub auto_format: bool,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub word_len: Vec<usize>
}

impl Table {
    pub fn get_style(&self) -> i32 {self.style}
    pub fn get_format(&self) -> bool {self.auto_format}
    pub fn get_headers(&self) -> Vec<String> {self.headers.clone()}
    pub fn get_rows(&self) -> Vec<Vec<String>> {self.rows.clone()}
    pub fn add_header(&mut self, header: String) {self.headers.push(header);}
    pub fn change_style(&mut self) {
        let mut user_input = String::new();
        println!("Style 1:\n+---------+---------+\n|name     |age      |\n+---------+---------+");
        println!("Style 2:\n┏━━━━━━━━━┳━━━━━━━━━┓\n┃name     ┃age      ┃\n┗━━━━━━━━━┻━━━━━━━━━┛");
        println!("Style 3:\n╔═════════╦═════════╗\n║name     ║age      ║\n╚═════════╩═════════╝");
        println!("Style 4:\n┌─────────┬─────────┐\n│name     │age      │\n└─────────┴─────────┘");
        println!("Style 5:\n╭─────────┬─────────╮\n│name     │age      │\n╰─────────┴─────────╯");

        println!("Please select a style (Example: 1 or 2 or 3...): ");
        std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let user_input_char: char = user_input.trim().parse().expect("Please type a single character");

        match user_input_char {
            '1' => self.style = 1,
            '2' => self.style = 2,
            '3' => self.style = 3,
            '4' => self.style = 4,
            '5' => self.style = 5,
            _ => println!("Cannot recognise character!")
        };
    }

    pub fn insert_row(&mut self, row: Vec<String>) {
        if self.word_len.is_empty() {
            for e in &self.headers {
                self.word_len.push(e.len());
            }
        }

        for i in 0..row.len() {
            if row[i].len() > self.word_len[i] { self.word_len[i] = row[i].len();}
            if self.headers.len() > self.word_len[i] { self.word_len[i] = self.headers[i].len(); }
        }
        self.rows.push(row);
    }

    pub fn input_row(&mut self) {
        let mut user_input = String::new();

        println!("Please introduce a row (Example: Rname, Remail, Rage)");
        std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let mut row: Vec<&str> = user_input.split(',').collect();
        let row_string: Vec<String> = row.iter().map(|s| s.trim().to_string()).collect();
        self.insert_row(row_string);
    }

    pub fn design_table(&self) -> String {
        let mut top_left = "+";
        let mut top_right = "+";
        let mut bottom_left = "+";
        let mut bottom_right = "+";
        let mut top_intersect = "+";
        let mut bottom_intersect = "+";
        let mut right_separator = "+";
        let mut left_separator = "+";
        let mut separator = "+";
        let mut line = "-";
        let mut col = "|";

        match self.style {
            1 => {}
            2 => {
                top_left = "┏";
                top_right = "┓";
                bottom_left = "┗";
                bottom_right = "┛";

                top_intersect = "┳";
                bottom_intersect = "┻";
                right_separator = "┫";
                left_separator = "┣";
                separator = "╋";
                line = "━";
                col = "┃";
            }
            3 => {
                top_left = "╔";
                top_right = "╗";
                bottom_left = "╚";
                bottom_right = "╝";

                top_intersect = "╦";
                bottom_intersect = "╩";
                right_separator = "╣";
                left_separator = "╠";
                separator = "╬";
                line = "═";
                col = "║";
            }
            4 => {
                top_left = "┌";
                top_right = "┐";
                bottom_left = "└";
                bottom_right = "┘";

                top_intersect = "┬";
                bottom_intersect = "┴";
                right_separator = "┤";
                left_separator = "├";
                separator = "┼";
                line = "─";
                col = "│";}
            5 => {
                top_left = "╭";
                top_right = "╮";
                bottom_left = "╰";
                bottom_right = "╯";

                top_intersect = "┬";
                bottom_intersect = "┴";
                right_separator = "┤";
                left_separator = "├";
                separator = "┼";
                line = "─";
                col = "│";
            }
            _ => { panic!("Error: Style not accepted") }
        }


        let n_cols = self.headers.len();
        let n_rows = self.rows.len();
        let mut top_line: String = top_left.to_string();
        let mut end_line: String = bottom_left.to_string();
        let mut header: String = String::new();
        let mut normal_line: String = String::new();
        let mut res: String = String::new();

        if self.rows.len() == 0 {
            return "".to_string();
        }

        for i in 0..n_cols {
            if i == 0 {
                normal_line += format!("{}{}", left_separator, line.repeat(self.word_len[i] + 2)).as_str();
            } else {
                normal_line += format!("{}{}", separator, line.repeat(self.word_len[i] + 2)).as_str();
            }
            if i == n_cols - 1 {
                top_line += format!("{}{}", line.repeat(self.word_len[i] + 2), top_right).as_str();
                end_line += format!("{}{}", line.repeat(self.word_len[i] + 2), bottom_right).as_str();
            } else {
                top_line += format!("{}{}", line.repeat(self.word_len[i] + 2), top_intersect).as_str();
                end_line += format!("{}{}", line.repeat(self.word_len[i] + 2), bottom_intersect).as_str();
            }
            match self.auto_format {
                true => header += format!("{}{:^width$}", col, self.headers[i], width = self.word_len[i] + 2).as_str(),
                false => header += format!("{}{}{}", col, self.headers[i], " ".repeat(self.word_len[i] - self.headers[i].len() + 2)).as_str()
            }
        }
        header += col;
        normal_line += right_separator;
        res += format!("{}{}{}{}{}{}", top_line.as_str(), "\n", header.as_str(), "\n", normal_line.as_str(), "\n").as_str();

        for i in 0..n_rows {
            for j in 0..n_cols {
                match self.auto_format {
                    true => { res += format!("{}{:^width$}", col, self.rows[i][j], width = self.word_len[j] + 2).as_str();}
                    false => { res += format!("{}{}{}", col, self.rows[i][j], " ".repeat(self.word_len[j] - self.rows[i][j].len() + 2)).as_str();}
                }
            }
            res += format!("{col}").as_str();
            if i != n_rows - 1 {
                res += format!("\n{normal_line}\n").as_str();
            } else {
                res += format!("\n{end_line}\n").as_str();
            }
        }
        res
    }
}

