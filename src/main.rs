use std::collections::HashMap;
use std::io;
struct PasswordData {
    website: String,
    username: String,
    password: String,
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

    let password_data = PasswordData {
        website: website.trim().to_string(),
        username: username.trim().to_string(),
        password: password.trim().to_string(),
    };

    data.insert(website, password_data);

    for p_data in data.values() {
        println!(
            "{}, {}, {}",
            p_data.website, p_data.username, p_data.password
        );
    }
}
