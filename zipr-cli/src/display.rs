use comfy_table::Table;
use zipr::{
    data::borrowed::{AsSymbols, ZipEntry, ZipPath},
    std::{ToNaiveDate, ToNaiveTime},
};

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for ZipPath<'_> {
    fn to_string(&self) -> String {
        self.to_utf8().collect::<String>()
    }
}

pub fn display_entries(entries: Vec<ZipEntry<'_>>) -> Table {
    let mut table = Table::new();
    let mut total = 0;
    table.set_header(vec!["Length", "Date", "Time", "Name"]);

    for e in entries.iter() {
        let row = vec![
            format!("{}", e.compressed_data.uncompressed_size()),
            format!("{}", e.file_modification_date.to_date()),
            format!("{}", e.file_modification_time.to_time()),
            e.file_name.to_string(),
        ];
        total += e.compressed_data.uncompressed_size();
        table.add_row(row);
    }
    table.add_row(vec![
        format!("{}", total),
        String::new(),
        String::new(),
        format!("{}", entries.len()),
    ]);

    table
}
