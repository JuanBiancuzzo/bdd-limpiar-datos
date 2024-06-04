use std::io::{BufReader, BufRead};
use std::fs::{self, File};
use serde_json::Value;

fn obtener_archivo_con_parametros(path_datos: Option<&String>, path_parametros: Option<&String>) -> Option<(File, Value)> {

    let (path_datos, path_parametros) = match (path_datos, path_parametros) {
        (None, _) => {
            println!("Se tiene que ingresar un path a un archivo con datos");
            return None;
        }
        (_, None) => {
            println!("Se tiene que ingresar un path a un archivo con los parametros de los datos");
            return None;
        }
        (Some(path_datos), Some(path_parametros)) => (path_datos, path_parametros)
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
    let arguments: Vec<String> = std::env::args().collect();

    let path_datos = arguments.get(1);
    let path_parametros = arguments.get(2);

    let (archivo_datos, parametros) = match obtener_archivo_con_parametros(path_datos, path_parametros) {
        Some((archivo_datos, parametros)) => (BufReader::new(archivo_datos), parametros),
        _ => return
    };

    println!("Tenemos los parametros: \n{:?}", parametros);

    let mut contador = 0;
    for linea in archivo_datos.lines() {
        if let Ok(linea) = linea {
            println!("{}", linea);
        }

        if contador > 5 {
            break;
        }
        contador += 1;
    }

}
