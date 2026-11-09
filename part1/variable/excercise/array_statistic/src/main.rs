fn main() {
    let scores: [i32; 6] = [85, 92, 78, 95, 60, 88];

    let min = scores.iter().min().unwrap();
    let max = scores.iter().max().unwrap();

    let sum: i32 = scores.iter().sum();
    let avg: f64 = sum as f64 / scores.len() as f64;

    println!("Min: {}", min);
    println!("Max: {}", max);
    println!("Sum: {}", sum);
    println!("Avg: {:.2}", avg);
}
