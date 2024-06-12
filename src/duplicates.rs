use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};


use chrono::NaiveDateTime;
use crate::review::Review;

pub fn get_uuids_set(file_path: &str) -> HashMap<String, NaiveDateTime> {
    let file = File::open(file_path).expect("Error: CouldNaiveDatefile.");
    let reader = BufReader::new(file);
    let mut latest_dates = std::collections::HashMap::new();
    let mut error_count = 0;
    let mut repeated_count = 0;

    for line in reader.lines() {
        let line = line.expect("Error: Couldn't read the line.");
        let line = line.replace("\"", "");

        match Review::new(&line, ",") {
            Ok(review) => {
                if let Some(date) = latest_dates.get(&review.id) {
                    repeated_count += 1;
                    if review.date > *date {
                        latest_dates.insert(review.id.clone(), review.date);
                    }
                } else {
                    latest_dates.insert(review.id.clone(), review.date.clone());
                }
            }
            Err(e) => {
                eprintln!("Error processing line: {:?} - {:?}", line, e);
                error_count += 1;
            }
        }
    }

    if error_count > 0 {
        eprintln!("There were {} errors while processing the file.", error_count);
    }
    
    if repeated_count > 0 {
        eprintln!("There were {} repeated UUIDs in the file.", repeated_count);
    }

    latest_dates
}