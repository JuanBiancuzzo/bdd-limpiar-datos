mod review;

use serde_json::Value;
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::process::exit;

const OUTPUT_FILE : &str = "datos/clean_reviews.csv";

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
    if arguments.len() != 3 {
        eprintln!("Error: Se necesitan ambas rutas a los archivos de datos y parámetros.");
        exit(1);
    }
}

fn get_path_from_arguments() -> (Option<String>, Option<String>) {
    let arguments: Vec<String> = env::args().collect();
    let path_datos = Some(arguments[1].clone());
    let path_parametros = Some(arguments[2].clone());
    (path_datos, path_parametros)
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

fn process_file(data_file : BufReader<File>, mut clean_reviews : File, header : usize, sep : String) {
    for (i, line) in data_file.lines().enumerate() {
        if i < header {
            writeln!(clean_reviews, "{}", line.unwrap()).expect("Error al escribir la cabecera en el archivo");
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
                writeln!(
                    clean_reviews,
                    "{};{};{};{};{};{};{}",
                    review.id,
                    review.user_name,
                    review.content,
                    review.score,
                    review.thumbs_up,
                    review.app_version,
                    review.date
                ).expect("Error al escribir la reseña en el archivo");
            },
            Err(_error) => {

            }
        }
    }
}

fn main() {
    validate_arguments();
    let (data_path, parameters_path) = get_path_from_arguments();
    let (data_file, parameters) = 
        match obtain_file_with_parameters(data_path, parameters_path) {
            Some((data_file, parameters)) => (BufReader::new(data_file), parameters),
            _ => {
                eprintln!("Error: Couldn't read the file.");
                return;
            }
        };

    println!("Parameters: \n{:?}\n", parameters);
    
    let (header, sep) = get_header_and_separator(parameters);

    //Crear el archivo. Si existe, simplemente re-escribirlo supongo.
    let clean_reviews = create_output_file();
    
    process_file(data_file, clean_reviews, header, sep);
}
