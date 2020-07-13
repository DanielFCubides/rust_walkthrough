use serde_derive::*;
use serde_json;

fn main() {
    let file = get_file(".gitignore").expect("a");
    println!("{}", file)
}

#[derive(Deserialize, Serialize, Debug)]
struct User {
    name: String,
    email: String,
}

enum MyError {
    LoadFile(String),
    MapFile(String),
}

impl From<std::io::Error> for MyError{
    fn from(e: std::io::Error) -> Self {
        MyError::LoadFile(e.to_string())
    }
}

impl From<serde_json::Error> for MyError{
    fn from(e: serde_json::Error) -> Self {
        MyError::LoadFile(e.to_string())
    }
}

fn get_file(fname: &str) -> Result<Vec<User>, MyError>{
    let file = std::fs::read_to_string(fname)?;
    let users = serde_json::from_str(&file)?;
    Ok(users)
}