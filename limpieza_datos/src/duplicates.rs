use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write}, // Import the Write trait
};

use crate::review::Review;
use chrono::NaiveDateTime;

pub fn get_uuids_set(file_path: &str) -> HashMap<String, NaiveDateTime> {
    let file = File::open(file_path).expect("Error: CouldNaiveDatefile.");
    let reader = BufReader::new(file);
    let mut latest_dates = std::collections::HashMap::new();
    let mut repeated_count = 0;

    for line in reader.lines() {
        let line = line.expect("Error: Couldn't read the line.");
        let line = line.replace('\"', "");

        match Review::new(&line, ",") {
            Ok(review) => {
                if let Some(date) = latest_dates.get(&review.id) {
                    repeated_count += 1;
                    if review.date > *date {
                        latest_dates.insert(review.id.clone(), review.date);
                    }
                } else {
                    latest_dates.insert(review.id.clone(), review.date);
                }
            }
            Err(_) => {
            }
        }
    }

    // if repeated_count > 0 {
    //     writeln!(
    //         program_data,
    //         "Había {} repeated UUIDs in the file.",
    //         repeated_count
    //     ).expect("Error: Couldn't write to the file."); 
    // }

    latest_dates
}
