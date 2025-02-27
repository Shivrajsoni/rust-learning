use std::fs;

enum Result {
    Ok(String),
    Err(String),
}
fn main() {
    let contents = fs.read_to_string("a.txt");

    match contents {
        Ok(contents) => println!("File is readed"),
        Err(e) => print!("Error Come"),
    }
}
