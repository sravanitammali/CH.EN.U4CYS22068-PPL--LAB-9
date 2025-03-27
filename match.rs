use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter menu item (Burger/Pizza/Pasta):");
    io::stdin().read_line(&mut input).unwrap();
    let item = input.trim();

    println!("Enter quantity:");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let qty: u32 = input.trim().parse().unwrap();

    let price = match item {
        "Burger" => 100,
        "Pizza" => 250,
        "Pasta" => 150,
        _ => {
            println!("Invalid item");
            return;
        }
    };

    let discount = if qty > 3 { 0.1 } else { 0.0 };
    let total_price = (price * qty) as f32 * (1.0 - discount);

    println!("Total bill: ₹{:.2}", total_price);
}
