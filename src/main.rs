use std::collections::HashMap;
use std::io;
mod password;
use password::PasswordData;
fn search_by_website(password_list: &HashMap<String, PasswordData>) {
    let mut input = String::new();
    println!("What is the website the data is for?");

    io::stdin()
        .read_line(&mut input)
        .expect("Cannot read input");

    let Some(target) = password_list.get(&input.trim().to_string()) else {
        return;
    }; //the hashmap get returns an option, must use Some(val) in order to handle the None case

    println!("{}", target);
}
fn main() {
    let mut data: HashMap<String, PasswordData> = HashMap::new();
    let mut website = String::new();
    let mut username = String::new();
    let mut password = String::new();

    println!("Welcome to password manager");
    println!("Enter the name of the website this is for:");
    io::stdin()
        .read_line(&mut website)
        .expect("Error parsing input");

    println!("Enter the username:");
    io::stdin()
        .read_line(&mut username)
        .expect("Error parsing input");
    println!("Enter the password:");

    io::stdin()
        .read_line(&mut password)
        .expect("Error parsing input");

    let password_data = PasswordData::new(
        website.trim().to_string(),
        username.trim().to_string(),
        password.trim().to_string(),
    );

    data.insert(password_data.website.clone(), password_data);

    search_by_website(&data);
}
