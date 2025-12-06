use std::{fs::File, io::{self,Read}};


fn main() {

    match read_content_from_file() {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error Reading file : {}", e),
    }
}

fn read_content_from_file() -> Result<String, io::Error> {

    let mut file_to_read_from = File::open("Hello.txt")?;
    let mut username = String::new(); 
    
    file_to_read_from.read_to_string(&mut username)?;

    Ok(username)
}
