use std::io::{self, Write};

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn calculate_average(scores: &[f64]) -> f64 {
    if scores.is_empty() {
        return 0.0;
    }
    let sum: f64 = scores.iter().sum();
    sum / scores.len() as f64
}

fn letter_grade(average: f64) -> &'static str {
    match average as u32 {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    }
}

fn pass_fail(average: f64) -> &'static str {
    if average >= 60.0 { "PASS" } else { "FAIL" }
}

fn main() {
    println!("=== Student Grade Calculator ===\n");
    let name = read_line("Student name: ");
    let count_str = read_line("Number of subjects: ");
    let count: usize = count_str.parse().unwrap_or(0);

    if count == 0 {
        println!("Invalid number of subjects.");
        return;
    }

    let mut scores: Vec<f64> = Vec::new();
    let mut subjects: Vec<String> = Vec::new();

    for i in 0..count {
        let subject = read_line(&format!("  Subject {} name: ", i + 1));
        let score_str = read_line(&format!("  Subject {} score (0-100): ", i + 1));
        let score: f64 = score_str.parse().unwrap_or(0.0);
        subjects.push(subject);
        scores.push(score);
    }

    let average = calculate_average(&scores);
    let grade = letter_grade(average);
    let result = pass_fail(average);

    println!("\n=== Report Card ===");
    println!("Student: {}", name);
    println!("-------------------");

    for (subject, score) in subjects.iter().zip(scores.iter()) {
        println!("  {:<20} {:>6.1}", subject, score);
    }
    println!("-------------------");
    println!("  {:<20} {:>6.1}", "Average", average);
    println!("  Grade: {}", grade);
    println!("  Result: {}", result);
}
