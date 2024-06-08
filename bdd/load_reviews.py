import csv
import sqlite3
from datetime import datetime
import re
import sys

def get_or_create_app_version(cursor, app_version):
    match = re.match(r'(\d+\.\d+\.\d+) build (\d+) (\d+)', app_version)
    if not match:
        raise ValueError(f"Formato de versión no válido: {app_version}")
    
    version = match.group(1)
    build_number = match.group(2)
    build_code = match.group(3)

    # Verificar si la versión ya existe en la base de datos
    cursor.execute("SELECT versionId FROM NetflixAppVersion WHERE version = ? AND buildNumber = ? AND buildCode = ?", 
                   (version, build_number, build_code))
    row = cursor.fetchone()
    if row:
        return row[0]
    else:
        cursor.execute("INSERT INTO NetflixAppVersion (version, buildNumber, buildCode) VALUES (?, ?, ?)", 
                       (version, build_number, build_code))
        return cursor.lastrowid
    
def get_or_create_user(cursor, user_name):
    cursor.execute("SELECT userId FROM NetflixUser WHERE userName = ?", (user_name,))
    row = cursor.fetchone()
    if row:
        return row[0]
    else:
        cursor.execute("INSERT INTO NetflixUser (userName) VALUES (?)", (user_name,))
        return cursor.lastrowid
    
def process_reviews(reviews_path, db_path):
    print(f"Procesando reviews desde {reviews_path} hacia {db_path}")
    # Conexion a la base de datos
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

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
                cursor.execute("""
                INSERT INTO NetflixReview (reviewID, userID, content, score, thumbsUpCount, createdAt, versionId)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                """, (review_id, user_id, content, score, thumbs_up_count, created_at, version_id))
            except Exception as e:
                print(f"Error al procesar la fila {row}")
                print(f"Error: {e}")

    print("Proceso terminado")
    # Guardar cambios y cerrar la conexión
    conn.commit()
    conn.close()

def main():
    if len(sys.argv) != 3:
        print("Uso: python load_reviews.py <archivo> <base de datos>")
    else:
        reviews_path = sys.argv[1] 
        db_path = sys.argv[2]
        #print(f"Procesando reviews desde {reviews_path} hacia {db_path}")
        process_reviews(reviews_path, db_path)

if __name__ == "__main__":
    main()