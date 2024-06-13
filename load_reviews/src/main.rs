mod record;

use std::{env, fs::File, io::{self, BufRead, BufReader, BufWriter, Write}};
use lazy_static::lazy_static;
use regex::Regex;

use chrono::Local;
use rusqlite::{Connection, Transaction};

const OUTPUT_FILE: &str = "load_to_db_data.txt";

const GET_VERSION_ID_QUERY: &str = r#"
SELECT versionId 
FROM NetflixAppVersion 
WHERE version = ? AND buildNumber = ? AND buildCode = ?
"#;

const INSERT_VERSION_QUERY: &str = r#"
INSERT INTO NetflixAppVersion (version, buildNumber, buildCode) 
VALUES (?, ?, ?)
"#;

const GET_USER_ID_QUERY: &str = r#"
SELECT userId 
FROM NetflixUser 
WHERE userName = ?
"#;

const INSERT_USER_QUERY: &str = r#"
INSERT INTO NetflixUser (userName) 
VALUES (?)
"#;

const INSERT_REVIEW_QUERY: &str = r#"
INSERT INTO NetflixReview (reviewID, userID, content, score, thumbsUpCount, createdAt, versionId) 
VALUES (?, ?, ?, ?, ?, ?, ?)
"#;

lazy_static! {
    static ref VERSION_RE: Regex = Regex::new(
        r"(\d+\.\d+\.\d+) build (\d+) (\d+)"
    ).expect("Deberia ser un regex valido");

}

fn get_or_create_app_version(tx: &Transaction, app_version: &str) -> io::Result<i32> {
    let captures = match VERSION_RE.captures(app_version) {
        Some(captures) => captures,
        None => return Err(io::Error::new(io::ErrorKind::Other, "Error: Invalid app version format.")),
    };

    let version = captures.get(1).ok_or(io::Error::new(io::ErrorKind::Other, "Error: Couldn't get version."))?.as_str();
    let build_number = captures.get(2).ok_or(io::Error::new(io::ErrorKind::Other, "Error: Couldn't get build number."))?.as_str();
    let build_code = captures.get(3).ok_or(io::Error::new(io::ErrorKind::Other, "Error: Couldn't get build code."))?.as_str();

    let mut stmt = match tx.prepare(GET_VERSION_ID_QUERY) {
        Ok(stmt) => stmt,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
    };

    let version_id: i32 = match stmt.query_row(&[version, build_number, build_code], |row| row.get(0)) {
        Ok(version_id) => version_id,
        Err(_) => {
            match tx.execute(INSERT_VERSION_QUERY, &[version, build_number, build_code]) {
                Ok(_) => (),
                Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
            }
    
            match stmt.query_row(&[version, build_number, build_code], |row| row.get(0)) {
                Ok(version_id) => version_id,
                Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
            }
        }
    };
    Ok(version_id)
}

fn get_or_create_user(tx: &Transaction, user_name: &str) -> io::Result<i32> {
    let mut stmt = match tx.prepare(GET_USER_ID_QUERY) {
        Ok(stmt) => stmt,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
    };

    let user_id: i32 = match stmt.query_row(&[user_name], |row| row.get(0)) {
        Ok(user_id) => user_id,
        Err(_) => {
            match tx.execute(INSERT_USER_QUERY, &[user_name]) {
                Ok(_) => (),
                Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
            }
    
            match stmt.query_row(&[user_name], |row| row.get(0)) {
                Ok(user_id) => user_id,
                Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
            }
        }
    };
    Ok(user_id)
}


fn get_bufreader_from_file(path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(path).expect("Error: Couldn't open the file.");
    Ok(BufReader::new(file))
}

fn get_bufwriter_from_file(path: &str) -> io::Result<BufWriter<File>> {
    let file = File::create(path).expect("Error: Couldn't create the file.");
    Ok(BufWriter::new(file))
}

fn process_reviews(reviews_path: &str, database_path: &str, stats_output_path: &str) -> io::Result<()> {
    let start_time = Local::now();
    let mut conn = match Connection::open(database_path) {
        Ok(conn) => conn,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
    };
    let transaction = match conn.transaction() {
        Ok(transaction) => transaction,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
    };

    let reviews_reader = get_bufreader_from_file(reviews_path)?;
    let mut total_rows = 0;
    let mut error_rows = 0;

    for line in reviews_reader.lines().skip(1) {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                error_rows += 1;
                println!("Error: Couldn't read line. {}", e.to_string());
                continue;
            }
        };

        // Clean the "" from the line
        let line = line.replace("\"", "");

        let record = match record::Record::new(&line, ",") {
            Ok(record) => record,
            Err(e) => {
                error_rows += 1;
                println!("Error: Couldn't read line. {:?}", e);
                continue;
            }
        };

        let app_version_id = get_or_create_app_version(&transaction, &record.app_version)?;
        let user_id = get_or_create_user(&transaction, &record.user_name)?;

        match transaction.execute(INSERT_REVIEW_QUERY, &[&record.id, &user_id.to_string(), &record.content, &record.score.to_string(), &record.thumbs_up.to_string(), &record.date.to_string(), &app_version_id.to_string()]) {
            Ok(_) => total_rows += 1,
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
        }
    }
    
    transaction.commit().expect("Error: Couldn't commit the transaction.");
    let end_time = Local::now();
    let elapsed_time = end_time - start_time;

    let mut stats_writer = get_bufwriter_from_file(stats_output_path)?;
    writeln!(stats_writer, "Start time: {}", start_time)?;
    writeln!(stats_writer, "End time: {}", end_time)?;
    writeln!(stats_writer, "Total rows processed: {}", total_rows)?;
    writeln!(stats_writer, "Error rows: {}", error_rows)?;
    writeln!(stats_writer, "Elapsed time: {}", elapsed_time)?;
    writeln!(stats_writer, "--------------------------------------------------------------------------------------")?;
    Ok(())
}   

fn main() {
    let args = env::args().collect::<Vec<String>>();
    println!("{:?}", args);

    if args.len() != 3 && args.len() != 4 {
        println!("Error: Invalid number of arguments.");
        println!("Usage: cargo run <file> <database> [optional: <program output>]");
        return;
    }

    let reviews_path = args[1].clone();
    let database_path = args[2].clone();
    let program_output_path = if args.len() == 4 {
        args[3].clone()
    } else {
        OUTPUT_FILE.to_string()
    };

    println!("Reviews file: {}", reviews_path);
    println!("Database file: {}", database_path);
    println!("Program output: {}", program_output_path);

    // Load reviews from file
    match process_reviews(&reviews_path, &database_path, &program_output_path) {
        Ok(_) => println!("Reviews loaded successfully."),
        Err(err) => println!("Error: {}", err),
    }


}