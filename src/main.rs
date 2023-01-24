extern crate core;

use crate::table::Table;

mod table;

fn main() {
    let mut t = Table {
        style: 3,
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
