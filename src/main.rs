mod review;
mod duplicates;

use chrono::Local;
use serde_json::Value;
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::process::exit;
use std::fs::OpenOptions;

const OUTPUT_FILE : &str = "datos/clean_reviews.csv";
const PROGRAM_DATA : &str = "datos/program_data.txt";


fn get_program_data_file(file_path : String) -> File {
    let file = OpenOptions::new()
    .append(true)  
    .create(true)
    .open(file_path).expect("Error: Couldn't open the file.");
    return file;
}

fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn create_output_file() -> File {
    if file_exists(OUTPUT_FILE) {
        match fs::remove_file(OUTPUT_FILE) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Error: Couldn't remove the file.");
                exit(1);
            }
        }
    }
    fs::File::create(OUTPUT_FILE).expect("Error: Couldn't create the file.")
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

fn get_header_and_separator(parameters : Value) -> (usize, String){
    let header: usize = match &parameters["header"] {
        Value::Number(header) if header.is_u64() => header.as_u64().unwrap().try_into().unwrap(),
        _ => exit(1),
    };

    let sep = match &parameters["separador"] {
        Value::String(sep) => sep.to_string(),
        _ => exit(1),
    };

    return (header, sep)
}

fn process_file(data_file : BufReader<File>, mut clean_reviews : File, mut program_data : File, header : usize, sep : String) {
    let start_time = Local::now();
    let start_time_str = start_time.to_string();
    let message = "Inicio del procesamiento del archivo de datos: ".to_string() + &start_time_str;
    writeln!(program_data, "{}", message).expect("Error al escribir en el archivo de datos del programa");

    let mut contador_ok = 0;
    let mut contador_error = 0;
    for (i, line) in data_file.lines().enumerate() {
        if i < header {
            writeln!(clean_reviews, "reviewId,userName,content,score,thumbsUpCount,date,appVersion").expect("Error al escribir la cabecera en el archivo");
            continue;
        }
    
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                eprintln!("Error al leer la línea del archivo de datos");
                return;
            }
        };
        match review::Review::new(&line, &sep) {
            Ok(review) => {
                contador_ok+=1;
                writeln!(
                    clean_reviews,
                    "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"",
                    review.id,
                    review.user_name,
                    review.content,
                    review.score,
                    review.thumbs_up,
                    review.date,
                    review.app_version
                ).expect("Error al escribir la reseña en el archivo");
            },
            Err(_error) => {
                contador_error+=1;
            }
        }
    }
    let end_time = Local::now();
    let end_time_str = end_time.to_string();
    writeln!(program_data, "Fin del procesamiento del archivo de datos: {}", end_time_str).expect("Error al escribir en el archivo de datos del programa");

    let duration = end_time - start_time;
    let seconds = duration.num_seconds();
    let minutes = seconds as f64 / 60.0;

    let duration_str = format!("Tiempo total de procesamiento: {} segundos ({:.2} minutos)", seconds, minutes);
    writeln!(program_data, "{}", duration_str).expect("Error al escribir en el archivo de datos del programa");
    writeln!(program_data, "Cantidad de líneas sin errores: {}", contador_ok).expect("Error al escribir en el archivo de datos del programa");
    writeln!(program_data, "Cantidad de líneas con errores: {}", contador_error).expect("Error al escribir en el archivo de datos del programa");
    writeln!(program_data, "--------------------------------------------------------------------------------------").expect("Error al escribir en el archivo de datos del programa");
}

fn main() {
    validate_arguments();
    let (data_path, parameters_path, program_data_path) = get_path_from_arguments();
    let (data_file, parameters) = 
        match obtain_file_with_parameters(data_path, parameters_path) {
            Some((data_file, parameters)) => (BufReader::new(data_file), parameters),
            _ => {
                eprintln!("Error: Couldn't read the file.");
                return;
            }
        };
    
    let (header, sep) = get_header_and_separator(parameters);

    let program_data = get_program_data_file(program_data_path.unwrap_or(PROGRAM_DATA.to_string()));

    let clean_reviews = create_output_file();
    
    process_file(data_file, clean_reviews, program_data, header, sep);

    duplicates::filter_duplicates();
}
