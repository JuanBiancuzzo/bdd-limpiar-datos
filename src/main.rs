mod review;

use csv::Writer;
use serde::Serialize;
use serde_json::Value;
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter};


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

    println!("Tenemos los parametros: \n{:?}\n", parametros);

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

        match review::Review::new(&line) {
            Ok(review) => {
                println!("{}", review);
                println!("Valid review!\n");
            },
            Err(error) => {
                println!("Invalid review! \n\tPor {:?}\n", error);
            }
        }
    }
}
