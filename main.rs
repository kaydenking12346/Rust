use std::io;

mod calculator {
    use std::io;

    pub fn addition(a: i32, b: i32) {
        println!("Answer: {}", a + b);
    }
    pub fn subtraction(a: i32, b: i32) {
        println!("Answer: {}", a - b);
    }
    pub fn multiplication(a: i32, b: i32) {
        println!("Answer: {}", a * b);
    }
    pub fn division(a: i32, b: i32) {
        println!("Answer: {}", a / b);
    }
}
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn main() {
    // get numbers
    let mut input = String::new();
    println!("Enter number 1:");
    io::stdin().read_line(&mut input).unwrap();
    let num_one: i32 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter number 2:");
    io::stdin().read_line(&mut input).unwrap();
    let num_two: i32 = input.trim().parse().unwrap();

    // get operation choice
    input.clear();
    println!("Choose operation (1=Add, 2=Subtract, 3=Multiply, 4=Divide):");
    io::stdin().read_line(&mut input).unwrap();
    let choice = match input.trim() {
        "1" => Operation::Add,
        "2" => Operation::Subtract,
        "3" => Operation::Multiply,
        "4" => Operation::Divide,
        _ => {
            println!("Invalid choice");
            return;
        }
    };

    // call function from module
    match choice {
        Operation::Add => calculator::addition(num_one, num_two),
        Operation::Subtract => calculator::subtraction(num_one, num_two),
        Operation::Multiply => calculator::multiplication(num_one, num_two),
        Operation::Divide => calculator::division(num_one, num_two),
    }
}