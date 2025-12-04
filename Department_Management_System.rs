use std::{collections::HashMap, io::{self, stdin}};

fn main() {
    
    
    println!("Employee Management System!");

    let choice  = String::new();
    let mut data:HashMap<String, u128> = HashMap::new();
    
    while choice != "4" {

        let mut choice  = String::new();
        
        println!("Enter your Choice");
        println!("1. Show all the departments and Employee Number.");
        println!("2. Add an Employee");
        println!("3. Add a new Department.");
        println!("4. Quit");

        io::stdin().read_line(&mut choice).expect("Invalid Input");

        match choice.trim() {
            "1" => {

                if data.is_empty() {
                    println!("No departments.");
                    continue;
                }

                for (key, value) in &data {
                    println!("{key} ->  {value}")
                }

            },

            "2" => {
                println!("Enter name of department to add the employee to.");
                
                let mut dept_name = String::new();

                io::stdin().read_line(&mut dept_name).expect("Input not registered");

                let dept_name = dept_name.trim().to_string();

                if let Some(value) = data.get_mut(&dept_name) {
                    *value += 1;
                    println!("Added an employee to {dept_name}");
                } else {
                    println!("Department Not Present in list.")
                }
            },
            "3" => {
                let mut new_department = String::new();
                
                println!("Enter department Name.");

                io::stdin().read_line(&mut new_department).expect("Invalid Input");

                let new_department = new_department.trim().to_string();

                data.entry(new_department.clone()).or_insert(0);

                println!("{} added to List of Departments.", new_department);
            }
            _ => break,
        }
    }

    println!("Thanks for using the EMS");
}
