mod duplicates;
mod load_reviews;
mod review;

use chrono::{DateTime, Local, NaiveDateTime};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::exit;
use std::{env, io};

const OUTPUT_FILE: &str = "datos/clean_reviews.csv";
const PROGRAM_DATA: &str = "datos/program_data.txt";

fn get_program_data_file(file_path: String) -> File {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Error: Couldn't open the file.");
    file
}

fn obtain_file_with_parameters(
    path_datos: Option<String>,
    path_parametros: Option<String>,
) -> Option<(File, Value)> {
    let (path_datos, path_parametros) = match (path_datos, path_parametros) {
        (None, _) => {
            println!("Se tiene que ingresar un path a un archivo con datos");
            return None;
        }
        (_, None) => {
            println!("Se tiene que ingresar un path a un archivo con los parametros de los datos");
            return None;
        }
        (Some(path_datos), Some(path_parametros)) => (path_datos, path_parametros),
    };

    let archivo_datos = match File::open(path_datos) {
        Ok(datos) => datos,
        Err(_) => {
            println!("El archivo de datos no existe");
            return None;
        }
    };

    let parametros: Value = match fs::read_to_string(path_parametros) {
        Ok(parametros_string) => match serde_json::from_str(&parametros_string) {
            Ok(parametros) => parametros,
            Err(_) => {
                println!("Los parametros no se pueden abrir como un archivo json");
                return None;
            }
        },
        Err(_) => {
            println!("El archivo de parametros no existe");
            return None;
        }
    };

    Some((archivo_datos, parametros))
}

fn validate_arguments() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 3 && arguments.len() != 4 {
        eprintln!("Error: Se necesitan ambas rutas a los archivos de datos y parámetros.");
        exit(1);
    }
}

fn get_path_from_arguments() -> (Option<String>, Option<String>, Option<String>) {
    let arguments: Vec<String> = env::args().collect();

    let path_datos = arguments.get(1).cloned();
    let path_parametros = arguments.get(2).cloned();
    let path_program_data = arguments.get(3).cloned();

    (path_datos, path_parametros, path_program_data)
}

fn get_header_and_separator(parameters: Value) -> (usize, String) {
    let header: usize = match &parameters["header"] {
        Value::Number(header) if header.is_u64() => header.as_u64().unwrap().try_into().unwrap(),
        _ => exit(1),
    };

    let sep = match &parameters["separador"] {
        Value::String(sep) => sep.to_string(),
        _ => exit(1),
    };

    (header, sep)
}

fn get_bufreader_from_file(path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(path).expect("Error: Couldn't open the file.");
    Ok(BufReader::new(file))
}

fn get_bufwriter_from_file(path: &str) -> io::Result<BufWriter<File>> {
    let file = File::create(path).expect("Error: Couldn't create the file.");
    Ok(BufWriter::new(file))
}

fn log_start(program_data: &mut File) -> io::Result<DateTime<Local>> {
    let start_time = Local::now();
    let start_time_string = start_time.to_string();
    let message =
        "Inicio del procesamiento del archivo de datos: ".to_string() + &start_time_string;
    writeln!(program_data, "{}", message)?;
    Ok(start_time)
}

fn log_end(program_data: &mut File, start_time: DateTime<Local>) -> io::Result<()> {
    let end_time = Local::now();
    let end_time_str = end_time.to_string();
    writeln!(
        program_data,
        "Fin del procesamiento del archivo de datos: {}",
        end_time_str
    )?;
    let duration = end_time - start_time;
    let seconds = duration.num_seconds();
    let minutes = seconds as f64 / 60.0;
    let duration_str = format!(
        "Tiempo total de procesamiento: {} segundos ({:.2} minutos)",
        seconds, minutes
    );
    writeln!(program_data, "{}", duration_str)?;
    Ok(())
}

fn log_stats(program_data: &mut File, contador_ok: i32, contador_error: i32) -> io::Result<()> {
    writeln!(
        program_data,
        "Cantidad de líneas sin errores: {}",
        contador_ok
    )?;
    writeln!(
        program_data,
        "Cantidad de líneas con errores: {}",
        contador_error
    )?;
    writeln!(
        program_data,
        "--------------------------------------------------------------------------------------"
    )?;
    Ok(())
}

fn write_header(writer: &mut BufWriter<File>) -> io::Result<()> {
    writeln!(
        writer,
        "reviewId,userName,content,score,thumbsUpCount,date,appVersion"
    )?;
    Ok(())
}

fn write_review(
    writer: &mut BufWriter<File>,
    review: review::Review,
    seen_uuids: &mut HashSet<String>,
) -> io::Result<()> {
    let new_line = format!(
        "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"",
        review.id,
        review.user_name,
        review.content,
        review.score,
        review.thumbs_up,
        review.date,
        review.app_version
    );

    seen_uuids.insert(review.id.clone());
    writeln!(writer, "{}", new_line)?;
    Ok(())
}

fn process_line(
    line: String,
    output_writer: &mut BufWriter<File>,
    latest_reviews: &HashMap<String, NaiveDateTime>,
    seen_uuids: &mut HashSet<String>,
    sep: &str,
) -> io::Result<()> {
    let review = match review::Review::new(&line, sep) {
        Ok(review) => review,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Error: Couldn't process the line.",
            ))
        }
    };

    if let Some(date) = latest_reviews.get(&review.id) {
        if review.date == *date && !seen_uuids.contains(&review.id) {
            write_review(output_writer, review, seen_uuids)?;
        } else {
            return Ok(());
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Error: Couldn't find the UUID in the map.",
        ));
    }

    Ok(())
}

fn process_lines(
    data_file_reader: BufReader<File>,
    output_writer: &mut BufWriter<File>,
    header: usize,
    latest_reviews: HashMap<String, NaiveDateTime>,
    sep: String,
) -> io::Result<(i32, i32)> {
    let mut contador_ok = 0;
    let mut contador_error = 0;

    write_header(output_writer)?;
    let mut seen_uuids = HashSet::new();

    for line in data_file_reader.lines().skip(header) {
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                contador_error += 1;
                continue;
            }
        };

        match process_line(
            line,
            output_writer,
            &latest_reviews,
            &mut seen_uuids,
            &sep,
        ) {
            Ok(_) => contador_ok += 1,
            Err(_) => contador_error += 1,
        }
    }

    output_writer
        .flush()
        .expect("Error: Couldn't flush the buffer.");
    Ok((contador_ok, contador_error))
}

fn process_file(
    data_path: &str,
    output_path: &str,
    mut program_data: File,
    header: usize,
    sep: String,
) -> io::Result<()> {
    let data_file_reader = get_bufreader_from_file(data_path)?;
    let mut writer = get_bufwriter_from_file(output_path)?;
    let latests_reviews = duplicates::get_uuids_set(data_path);
    let start_time = log_start(&mut program_data)?;
    let (contador_ok, contador_error) =
        match process_lines(data_file_reader, &mut writer, header, latests_reviews, sep) {
            Ok((ok, error)) => (ok, error),
            Err(_) => {
                eprintln!("Error: Couldn't process the file.");
                return Ok(());
            }
        };
    log_end(&mut program_data, start_time)?;
    log_stats(&mut program_data, contador_ok, contador_error)?;
    Ok(())
}

fn main() {
    validate_arguments();
    let (data_path, parameters_path, program_data_path) = get_path_from_arguments();
    let (_, parameters) = match obtain_file_with_parameters(data_path.clone(), parameters_path) {
        Some((data_file, parameters)) => (BufReader::new(data_file), parameters),
        _ => {
            eprintln!("Error: Couldn't read the file.");
            return;
        }
    };

    let (header, sep) = get_header_and_separator(parameters);
    let program_data = get_program_data_file(program_data_path.unwrap_or(PROGRAM_DATA.to_string()));

    let result = process_file(
        &data_path.expect("Error: Couldn't read the file."),
        OUTPUT_FILE,
        program_data,
        header,
        sep,
    );
    match result {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Couldn't process the file.");
        }
    }
}
