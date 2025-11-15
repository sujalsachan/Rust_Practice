use std::io;

fn main() {
    
    let mut s = String::new();
    let mut val = String::new();

    println!("Enter the Temperature :");
    
    io::stdin()
    .read_line(&mut val)
    .expect("Falied to take the input.");

    let val : i128 = val.trim().parse()
    .expect("Not a Number");

    println!("Enter the type to convert the temp in :");

    io::stdin()
    .read_line(&mut s)
    .expect("Falied to take the input.");

    if s == "Celcius" {
        let val = (val - 32) * (5/9);
        println!("Temperature in Fahrenheit : {val}");
    } else {
        let val =  ((9/5) * val ) + 32;
        println!("Temperature in Celcius : {val}");
    }
}