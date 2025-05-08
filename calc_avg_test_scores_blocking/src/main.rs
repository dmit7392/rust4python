use std::fs::File;
use std::io::{self, BufRead};

fn process_file() -> io::Result<()> {
    let path = "../../data/biology_test1_scores.csv";

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut total_score = 0;
    let mut count = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        if index == 0 {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();

        if let Ok(score) = parts[1].parse::<i32>() {
            total_score += score;
            count += 1;
        }
    }

    if count > 0 {
        let average = total_score as f64 / count as f64;
        println!("Average Biology Test Score: {:.2}", average);
    } else {
        println!("No scores found in the file.");
    }

    Ok(())
}

fn main() -> io::Result<()> {
}
