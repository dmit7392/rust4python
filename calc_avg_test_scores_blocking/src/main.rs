use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use anyhow::Result;

fn process_file(path: &PathBuf) -> Result<f64> {
    //let path = "../../data/biology_test1_scores.csv";

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

    Ok(
        if count > 0 {
            let average = total_score as f64 / count as f64;
            average
        }
        else {
            std::f64::NAN
        }
    )
}

fn main() -> Result<()> {

    let map: HashMap<PathBuf, f64> = HashMap::new();
    let data = Arc::new(Mutex::new(map));

    let file_paths = std::fs::read_dir("../data")?;
    let mut handles = vec![];


    for (i, entry) in file_paths.enumerate() {
        let path = entry?.path();
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let val_or_err = process_file(&path);


            match val_or_err {
                Ok(val) => {
                    println!("{path:?} => {val}");
                },
                Err(e) => {
                    println!("{path:?} => no way");                    
                }
            }

        });

        handles.push(handle);
    }

    for handle in handles {
        match handle.join() {
            Ok(_) => {},
            Err(_) => {},
        }
    }

    Ok(())
}
