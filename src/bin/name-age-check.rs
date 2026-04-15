mod data {
    use std::io;
    pub fn name_formatter() {
        let mut name: String = String::new();
        println!("What is your first name?");

        io::stdin()
            .read_line(&mut name)
            .expect("Error");

        let first_letter: String = name[0..1].to_uppercase();
        let other: String = name[1..].to_lowercase();

        let formatted_name: String = first_letter + &other;
        println!("\nName: {}", formatted_name);
    }

    pub fn age_check() {
        let mut age: String = String::new();
        println!("What is your age?");
        io::stdin()
            .read_line(&mut age)
            .expect("Error");

        println!("Age: {}", age);
    }
}

fn main() {
    data::name_formatter();
    data::age_check();
}
