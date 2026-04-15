use std::fs::File;
use std::io::{self,Read};
fn main() {
    let mut v = vec![100, 37, 52];
    for i in &mut v {
        println!("{i}");
    }
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    let file_result = File::open("hello.txt");
    let file = match file_result {
        Ok(file) => panic!("File opened successfully: {file:?}"),
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
fn read_username_from_file() -> Result<String, std::io::Error> {
    let file_result = File::open("hello.txt");
    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}
