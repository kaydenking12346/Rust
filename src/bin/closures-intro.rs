
fn main() {
    let num = 16;
    let is_even = |x| x % 2 == 0;
    let results = if is_even(num) { "yes" } else { "no" };
    println!("Is {} even? {}!", num, results);
}
