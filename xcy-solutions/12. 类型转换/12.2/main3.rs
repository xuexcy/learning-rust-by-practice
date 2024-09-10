// use std::fs;
// use std::io;
// use std::num;
//
// enum CliError {
//     IoError(io::Error),
//     ParseError(num::ParseIntError),
// }
//
// impl From<io::Error> for CliError {
//     // 实现 from 方法
// }
//
// impl From<num::ParseIntError> for CliError {
//     // 实现 from 方法
// }
//
// fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
//     // ? 自动将 io::Error 转换成 CliError
//     let contents = fs::read_to_string(&file_name)?;
//     // num::ParseIntError -> CliError
//     let num: i32 = contents.trim().parse()?;
//     Ok(num)
// }
//
// fn main() {
//     println!("Success!")
// }

use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    // 实现 from 方法
    fn from(io_err: io::Error) -> CliError {
        CliError::IoError(io_err)
    }
}

impl From<num::ParseIntError> for CliError {
    // 实现 from 方法
    fn from(err: num::ParseIntError) -> CliError {
        CliError::ParseError(err)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? 自动将 io::Error 转换成 CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() {
    println!("Success!")
}
