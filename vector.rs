fn main() {
    let temps = vec![30, 35, 32, 40, 38, 29, 31];

    let avg_temp: f64 = temps.iter().sum::<i32>() as f64 / temps.len() as f64;
    let min_temp = temps.iter().min().unwrap();
    let max_temp = temps.iter().max().unwrap();

    println!("Average: {:.2}, Min: {}, Max: {}", avg_temp, min_temp, max_temp);
}
