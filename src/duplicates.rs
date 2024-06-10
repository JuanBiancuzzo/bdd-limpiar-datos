use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader, BufWriter, Write}};

#[derive(Debug)]
pub enum RecordError {
    InvalidFieldCount(usize, usize),
}

struct Record {
    uuid: String,
    date: String,
}

impl Record {
    fn from_line(line: &str, delimiter: &str) -> Result<Record, RecordError> {
        let fields: Vec<&str> = line.split(delimiter).collect();
        if fields.len() != 7 {
            return Err(RecordError::InvalidFieldCount(7, fields.len()));
        }

        Ok(Record {
            uuid: fields[0].to_string(),
            date: fields[6].to_string(),
        })
    }
}

// This function reads the csv and keeps a dictionary with the latest date of each uuid
// If a uuid is found again, it compares the date with the one in the dictionary
// If the date is newer, it replaces the date in the dictionary and writes the line to the output file
// If the date is older, it ignores the line
fn get_uuids_set(file_path: String) -> HashMap<String, String> {
    let file = File::open(file_path).expect("Error: Couldn't open the file.");
    let reader = BufReader::new(file);
    let mut latest_dates = std::collections::HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Error: Couldn't read the line.");
        let line = line.replace("\"", "");

        match Record::from_line(&line, ",") {
            Ok(review) => {
                if let Some(date) = latest_dates.get(&review.uuid) {
                    if review.date > *date {
                        latest_dates.insert(review.uuid.clone(), review.date.clone());
                    }
                } else {
                    latest_dates.insert(review.uuid.clone(), review.date.clone());
                }
            }
            Err(e) => {
                eprintln!("Error processing line: {:?} - {:?}", line, e);
            },
        }
    }


    latest_dates
}

pub fn filter_duplicates() {
    let file_path = "datos/clean_reviews.csv".to_string();
    let file = File::open("datos/clean_reviews.csv").expect("Error: Couldn't open the file.");
    let reader = BufReader::new(file);
    let output = File::create("datos/filtered_reviews.csv").expect("Error: Couldn't create the file.");
    let mut writer = BufWriter::new(output);

    let latest_dates = get_uuids_set(file_path);
    let mut seen_uuids = HashSet::new();

    for line in reader.lines() {
        let line = line.expect("Error: Couldn't read the line.");
        let line = line.replace("\"", "");

        match Record::from_line(&line, ",") {
            Ok(review) => {
                if let Some(date) = latest_dates.get(&review.uuid) {
                    if review.date == *date && !seen_uuids.contains(&review.uuid) {
                        writeln!(writer, "{}", line).expect("Error: Couldn't write to the file.");
                        seen_uuids.insert(review.uuid.clone());
                    }
                }
            }
            Err(e) => {
                eprintln!("Error processing line: {:?} - {:?}", line, e);
            },
        }
    }

    writer.flush().expect("Error: Couldn't flush the buffer.");
}