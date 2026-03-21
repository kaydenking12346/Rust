use std::io;

fn choice() {
        let answer: i32;
        let mut choice_two: String = String::new();
        let mut x = String::new(); println!("Enter number:"); 
        io::stdin().read_line(&mut x).expect("Error");
        let mut y = String::new(); println!("Enter number:"); 
        io::stdin().read_line(&mut y).expect("Error");
        println!("
What calculation:
(1) Addition
(2) Subtraction
(3) Multiplication
(4) Division
        "); io::stdin()
            .read_line(&mut choice_two)
            .expect("Error");
            let x: i32 = x.trim().parse().expect("Error");
            let y: i32 = y.trim().parse().expect("Error");
            if choice_two.trim() == "1" {
                answer = x + y; println!("{} + {} = {}", x, y, answer);
            } else if choice_two.trim() == "2" {
                answer = x - y; println!("{} - {} = {}", x, y, answer);
            } else if choice_two.trim() == "3" {
                answer = x * y; println!("{} x {} = {}", x, y, answer);
            } else if choice_two.trim() == "4" {
                answer = x / y; println!("{} ÷ {} = {}", x, y, answer);
            }
}


fn main() {
    choice();
}
