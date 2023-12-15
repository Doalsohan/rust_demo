
use std::fs::File;
use std::{fs, io};
use std::io::{ErrorKind, Read};
/// 失败时 panic 的简写：unwrap 和 expect
#[test]
fn test_file() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
}


fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut user_name_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match user_name_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


fn read_username_from_file_v1() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

///哪里可以使用 ? 运算符
///
/// ? 运算符只能被用于返回值与 ? 作用的值相兼容的函数。因为 ? 运算符被定义为从函数中提早返回一个值
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}


fn read_username_from_file_v3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

