use std::env;


struct CSV {
    old_filename: String,
    new_filename: String,
}


fn main() {
    // PARAM 1 CSV
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    let csv_path = &args[1];

    print!("{}", csv_path);

    // parse csv

    // construct Vec of csv's

    // loop throug vec
        // rename files
    
}
