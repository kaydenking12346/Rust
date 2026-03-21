use std::io;

fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    loop {
        println!("Enter numbers to sort [x to end]");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error");
        let trimmed = input.trim();
        if trimmed == "x" || trimmed == "X" {
            numbers.sort();
            println!("Sorted Numbers: {:?}", numbers);
            break;
        }
        let num: i32 = trimmed.parse().expect("Please enter a valid number or 'x'");
        numbers.push(num);
    }
}