use crate::table::Table;

mod table;

fn main() {
    let mut t = Table {
        style: 1,
        row_id: true,
        auto_format: true,
        headers: vec!["name".to_owned(), "age".to_owned()],
        rows: Vec::new(),
        word_len: Vec::new(),
    };

    t.insert_row(vec!["Cristiano".to_owned(), "20".to_owned()]);
    println!("{:?}", t.get_rows());
    t.normal_print();
}
