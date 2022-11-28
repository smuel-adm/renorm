use std::env;
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
    // PARAM 1 CSV
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    let csv_path = &args[1];

    let root_dir = Path::new(&csv_path).parent().unwrap();
    //dbg!(&root_dir);

    print!("{}", csv_path);

    // parse csv
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(&csv_path).unwrap();
    for result in rdr.deserialize() {
        // loop throug vec
            // rename files
        let line: CSV = result.unwrap();
        println!("{:?}", line);
    }

}
