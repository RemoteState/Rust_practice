
use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    // IMPLEMENT from method
    fn from(x: io::Error) -> Self {
        CliError::IoError(x)
    }
}

impl From<num::ParseIntError> for CliError {
    // IMPLEMENT from method
    fn from(x: num::ParseIntError) -> Self {
        CliError::ParseError(x)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? automatically converts io::Error to CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() {
    println!("Success!");
}
