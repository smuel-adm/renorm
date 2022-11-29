use std::{env, fs, error::Error, process};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::path::Path;


#[derive(Debug, Deserialize)]
struct CSV {
    // 9123917,45756807,de,DIN EN ISO 4759-1,200104,PDF
    old_filename: String,
    _value1: String,
    _lang: String,
    new_filename: String,
    _value2: String,
    _extension: String,
}


fn rename() -> Result<(), Box<dyn Error>> {
    // The program needs one argument. The readme from the Beuth zip file
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("Error:\nThe program needs one argument. The readme from the Beuth zip file\n\n");
            process::exit(1);
        },
        _ => {}
    }
    
    let csv_path = &args[1];

    // Figure out root dir from given readme file. Should never fail.
    let root_dir = match Path::new(&csv_path).parent() {
        None => {
            println!("Error:\nCould not figure out root directory from given file\n\n");
            process::exit(2);
        },
        Some(path) => path
    };

    // parse readme
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(&csv_path)?;
    
    // loop through CSV file lines
    for result in rdr.deserialize() {
        // rename files
        let line: CSV = result?;
        let old_filename = root_dir.display().to_string() + "\\" + &line.old_filename + ".pdf";

        let new_filename = root_dir.display().to_string() + "\\" + &line.new_filename + ".pdf";

        fs::rename(old_filename, new_filename)?;
    }

    Ok(())

}


fn main() {
    if let Err(err) = rename() {
        println!("Error:\n{}", err);
        process::exit(1);
    }
}