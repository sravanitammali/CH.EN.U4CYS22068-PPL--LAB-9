use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter your age:");
    io::stdin().read_line(&mut input).unwrap();
    let age: u32 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter your income:");
    io::stdin().read_line(&mut input).unwrap();
    let income: u32 = input.trim().parse().unwrap();

    if age < 21 {
        println!("You are ineligible for a loan.");
    } else if age <= 60 {
        if income > 50000 {
            println!("You are eligible for a loan.");
        } else {
            println!("Income too low for a loan.");
        }
    } else {
        println!("You need a guarantor for a loan.");
    }
}
