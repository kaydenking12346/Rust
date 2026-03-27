use std::io;
fn password_strength(password: &str) {
    let mut pass_score = 0;
    if password.len() >= 8 {
        pass_score += 1;
    }
    if password.chars().any(|c| c.is_uppercase()) {
        pass_score += 1;
    }
    if password.chars().any(|c| c.is_numeric()) {
        pass_score += 1;
    }
    if password.chars().any(|c| c.is_alphanumeric()) {
        pass_score += 1
    }
    match pass_score {
        0 | 1 => println!("Weak"),
        2 => println!("Medium"),
        3 | 4 => println!("Strong"),
        _ => println!("Error"),
    }
}
fn main() {
    let mut password = String::new();
    println!("Enter password");
    io::stdin()
        .read_line(&mut password)
        .expect("Error");
    let password = password.trim();
    password_strength(password);
}
