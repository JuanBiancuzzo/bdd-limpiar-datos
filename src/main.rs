use chrono::NaiveDateTime;
use csv::Writer;
use serde::Serialize;
use serde_json::Value;
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter};

#[derive(Debug)]
struct Review {
    id: String,
    user_name: String,
    content: String,
    score: i32,
    thumbs_up: i32,
    app_version: String,
    date: String,
}

fn obtener_archivo_con_parametros(
    path_datos: Option<&String>,
    path_parametros: Option<&String>,
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
use regex::Regex;

fn is_valid_id(id: &str) -> bool {
    let re = Regex::new(
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
    )
    .unwrap();
    re.is_match(id)
}

fn is_valid_user_name(user_name: &str) -> bool {
    return user_name != "";
}

fn is_valid_comment(comment: &str) -> bool {
    return comment != "";
}

//Agregar validaciones extra? Por ejemplo si esperamos tener reviews de cierto año en adelante y demás
fn is_valid_date(date: &str) -> bool {
    return NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").is_ok();
}

//Cuando se lee, si no hay nada, por default se pone -1, así que habría que descartar estas líneas
fn is_valid_score(score: i32) -> bool {
    return score != -1;
}

//Cuando se lee, si no hay nada, por default se pone -1, así que habría que descartar estas líneas
fn is_valid_thumbs_up(thumbs_up: i32) -> bool {
    return thumbs_up != -1;
}

fn is_valid_app_version(app_version: &str) -> bool {
    let re = Regex::new(r"^\d+\.\d+\.\d+ build \d+ \d+$").unwrap();
    re.is_match(app_version)
}

fn is_valid_record(review: &Review) -> bool {
    return is_valid_date(&review.date)
        && is_valid_score(review.score)
        && is_valid_thumbs_up(review.thumbs_up)
        && is_valid_id(&review.id)
        && is_valid_user_name(&review.user_name)
        && is_valid_comment(&review.content)
        && is_valid_app_version(&review.app_version);
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let path_datos = arguments.get(1);
    let path_parametros = arguments.get(2);

    let (archivo_datos, parametros) =
        match obtener_archivo_con_parametros(path_datos, path_parametros) {
            Some((archivo_datos, parametros)) => (BufReader::new(archivo_datos), parametros),
            _ => {
                eprintln!("Error: Se necesitan ambas rutas a los archivos de datos y parámetros.");
                return;
            }
        };

    println!("Tenemos los parametros: \n{:?}", parametros);

    //Crear el archivo. Si existe, simplemente re-escribirlo supongo.

    for (i, line) in archivo_datos.lines().enumerate() {
        if i == 0 {
            //Escribir el encabezado en el archivo
            continue;
        }
        if i >= 5 {
            break;
        }

        let line = match line {
            Ok(line) => line,
            Err(_) => {
                eprintln!("Error al leer la línea del archivo de datos");
                return;
            }
        };

        // Ignorar la última columna del CSV
        let mut fields: Vec<&str> = line.split(',').collect();
        fields.pop();

        // Convertir a Review
        let review = Review {
            id: fields[0].to_string(),
            user_name: fields[1].to_string(),
            content: fields[2].to_string(),
            score: fields[3].parse().unwrap_or(-1),
            thumbs_up: fields[4].parse().unwrap_or(-1),
            app_version: fields[5].to_string(),
            date: fields[6].to_string(),
        };

        println!("{:?}", review);
        if is_valid_record(&review) {
            println!("Valid review!\n");
            //Escribir en el archivo
        }
        else {
            println!("Invalid review!\n");
        }
    }
}
