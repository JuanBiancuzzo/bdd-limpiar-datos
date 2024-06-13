import csv
import sqlite3
from datetime import datetime
import re
import sys
import os
from sql_constants import GET_VERSION_ID_QUERY, INSERT_VERSION_QUERY, GET_USER_ID_QUERY, INSERT_USER_QUERY, INSERT_REVIEW_QUERY

OUTPUT_PATH = "../logs/load_to_db_data.txt"

def get_or_create_app_version(cursor, app_version):
    match = re.match(r'(\d+\.\d+\.\d+) build (\d+) (\d+)', app_version)
    if not match:
        raise ValueError(f"Formato de versión no válido: {app_version}")
    
    version = match.group(1)
    build_number = match.group(2)
    build_code = match.group(3)

    # Verificar si la versión ya existe en la base de datos
    cursor.execute(GET_VERSION_ID_QUERY, (version, build_number, build_code))
    row = cursor.fetchone()
    if row:
        return row[0]
    else:
        cursor.execute(INSERT_VERSION_QUERY, (version, build_number, build_code))
        return cursor.lastrowid
    
def get_or_create_user(cursor, user_name):
    cursor.execute(GET_USER_ID_QUERY, (user_name,))
    row = cursor.fetchone()
    if row:
        return row[0]
    else:
        cursor.execute(INSERT_USER_QUERY, (user_name,))
        return cursor.lastrowid
    

def process_reviews(reviews_path, db_path, output_path):
    start_time = datetime.now()

    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    success = True
    total_rows = 0
    error_rows = 0

    with open(reviews_path, 'r', encoding='utf-8') as csvfile:
        csvreader = csv.DictReader(csvfile)
        for row in csvreader:
            try:
                review_id = row['reviewId']
                user_name = row['userName']
                content = row['content']
                score = row['score']
                thumbs_up_count = row['thumbsUpCount']
                created_at = datetime.strptime(row['date'], '%Y-%m-%d %H:%M:%S')
                app_version = row['appVersion']
                version_id = get_or_create_app_version(cursor, app_version)
                user_id = get_or_create_user(cursor, user_name)
                # Insertar la review incluyendo a los ids de usuario y versión
                cursor.execute(INSERT_REVIEW_QUERY,
                               (review_id, user_id, content, score, thumbs_up_count, created_at, version_id))
                total_rows += 1
            except Exception as e:
                print(f"Error al procesar la fila {row}")
                print(f"Error: {e}")
                error_rows += 1
                success = False

    # Guardar cambios y cerrar la conexión
    conn.commit()
    conn.close()

    end_time = datetime.now()
    with open(output_path, "a", encoding="utf-8") as output_file:
        output_file.write(f"Inicio del proceso: {start_time}\n")
        output_file.write(f"Fin del proceso: {end_time}\n")
        output_file.write(f"Tiempo total: {end_time - start_time}\n")
        output_file.write(f"Total de filas procesadas: {total_rows}\n")
        output_file.write(f"Filas con errores: {error_rows}\n")
        if success:
            output_file.write("El proceso se completó exitosamente\n")
        else:
            output_file.write("Ocurrieron errores durante el proceso\n")
        output_file.write(f"--------------------------------------------------------------------------------------\n")



def main():
    if len(sys.argv) != 3 and len(sys.argv) != 4:
        print("Uso: python load_reviews.py <archivo> <base de datos> [opcional: <salida del programa>]")
    elif len(sys.argv) == 3:
        reviews_path = sys.argv[1] 
        db_path = sys.argv[2]
        output = OUTPUT_PATH  # Nombre del archivo de salida por defecto
        if not os.path.isfile(output):  # Verifica si el archivo de salida no existe
            open(output, 'w').close()  # Crea el archivo si no existe
        output_mode = "a" if os.path.isfile(output) else "w"  # Modo "append" si el archivo existe, de lo contrario crea uno nuevo
        process_reviews(reviews_path, db_path, output)
    else:
        reviews_path = sys.argv[1] 
        db_path = sys.argv[2]
        output = sys.argv[3]
        if not os.path.isfile(output):  # Verifica si el archivo de salida no existe
            open(output, 'w').close()  # Crea el archivo si no existe
        output_mode = "a" if os.path.isfile(output) else "w"  # Modo "append" si el archivo existe, de lo contrario crea uno nuevo
        process_reviews(reviews_path, db_path, output)

if __name__ == "__main__":
    main()