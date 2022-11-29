use std::{env, fs};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct CSV {
    // 9123917,45756807,de,DIN EN ISO 4759-1,200104,PDF
    old_filename: String,
    value1: String,
    lang: String,
    new_filename: String,
    value2: String,
    extension: String,
}


fn main() {
    // PARAM 1 CSV file
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    
    let csv_path = &args[1];
    // dbg!(csv_path);

    // figure out root dir
    let root_dir = Path::new(&csv_path).parent().unwrap();
    //dbg!(&root_dir);


    // parse csv
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(&csv_path).unwrap();
    
    // loop through CSV file lines
    for result in rdr.deserialize() {
        // rename files
        let line: CSV = result.unwrap();
        let old_filename = root_dir.display().to_string() + "\\" + &line.old_filename + ".pdf";
        // dbg!(old_filename);
        let new_filename = root_dir.display().to_string() + "\\" + &line.new_filename + ".pdf";
        // dbg!(new_filename);
        fs::rename(old_filename, new_filename).unwrap();
    }

}
