use std::fmt::format;
use std::io::repeat;

pub struct Table {
    pub style: i32,
    pub row_id: bool,
    pub auto_format: bool,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub word_len: Vec<usize>
}

impl Table {
    // Getters
    /// Desired style for the table format:
    /// 1 -> +---------+---------+---------+
    /// 2 -> special characters
    pub fn get_style(&self) -> i32 {self.style}
    pub fn get_row_id(&self) -> bool {self.row_id}
    pub fn get_format(&self) -> bool {self.auto_format}
    pub fn get_headers(&self) -> Vec<String> {self.headers.clone()}
    pub fn get_rows(&self) -> Vec<Vec<String>> {self.rows.clone()}


    pub fn insert_row(&mut self, row: Vec<String>) {
        if self.word_len.is_empty() {self.word_len = vec![0; self.headers.len()]}

        for i in 0..row.len() {
            if row[i].len() > self.word_len[i] {self.word_len[i] = row[i].len();}
            if self.headers.len() > self.word_len[i] {self.word_len[i] = self.headers[i].len();}
        }
        self.rows.push(row);
    }

    pub fn normal_print(&self) {
        let top_left = "┏";
        let top_right = "┓";
        let bottom_left = "┗";
        let bottom_right = "┛";

        let top_intersect = "┳";
        let bottom_intersect = "┻";
        let right_separator = "┫";
        let left_separator = "┣";
        let separator = "╋";
        let line = "━";
        let col = "┃";

        let n_cols = self.headers.len();
        let n_rows = self.rows.len();

        // +--------+--------+--------+
        // |name    |age     |email   |
        // +--------+--------+--------+

        let mut top_line: String = top_left.to_string();
        let mut header: String = String::new();
        let mut normal_line: String = String::new();
        for i in 0..n_cols {
            if i == 0 {
                normal_line += format!("{}{}", left_separator, line.repeat(self.word_len[i]+2)).as_str();
            }
            else {
                normal_line += format!("{}{}", separator, line.repeat(self.word_len[i]+2)).as_str();
            }
            if i == n_cols-1 {
                top_line += format!("{}{}", line.repeat(self.word_len[i]+2), top_right).as_str();
            }
            else {
                top_line += format!("{}{}", line.repeat(self.word_len[i]+2), top_intersect).as_str();
            }
            header += format!("{}{}{}", col, self.headers[i], " ".repeat(self.word_len[i]-self.headers[i].len()+2)).as_str();
        }
        header += col;
        normal_line += right_separator;


        println!("{}", top_line);
        println!("{}", header);
        println!("{}", normal_line);


        //top_line[5] = "a";


    }


    pub fn print(&self) {
        //match self.style {
            //1 => normal_print(),
            //_ => {}
        //}
    }




    pub fn preview(&self) {

    }

}