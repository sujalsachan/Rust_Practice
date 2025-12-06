use std::{fs::File, io::{self, Read}};

fn main() {

    match read_content_from_file() {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error Reading file : {}", e),
    }

}

fn read_content_from_file() -> Result<String, io::Error> {

    let mut file_to_read_from = match  File::open("Hello.txt") {
          Ok(file) => file,
          Err(e) => return Err(e),
    };

    let mut content = String::new();

    match file_to_read_from.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }

}
