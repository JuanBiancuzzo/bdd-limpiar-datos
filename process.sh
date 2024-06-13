#!/bin/bash

# Definir códigos de color ANSI
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # Sin color

archivo="bdd/reviews.db"

# Verificar si el archivo existe
if [ -e "$archivo" ]; then
    echo -e "${GREEN}Usando base de datos existente.${NC}"
else
    echo -e "${GREEN}Creando la base de datos...${NC}"
    cd bdd
    sqlite3 reviews.db < create_reviews_table.sql
    cd ..
fi

cd limpieza_datos
echo -e "${YELLOW}Limpiando los datos. Be patient, this may take a while.${NC}"
cargo run ../datos/netflix_reviews.csv ../datos/parametros.json > /dev/null 2>&1

echo -e "${GREEN}Resultado de la limpieza de datos:${NC}"
echo "--------------------------------------------------------------------------------------"
cd ../logs
FILE_NAME="program_data.txt"

print_last_output() {
    local file_name=$1
    local start_pattern=$2
    local last_output=""
    local start_found=0
    local line=""

    while IFS= read -r line; do
        if [[ "$line" == *"$start_pattern"* ]]; then
            start_found=1
            last_output=""
        fi
        if [ $start_found -eq 1 ]; then
            last_output+="$line\n"
        fi
    done < "$file_name"

    echo -e "$last_output"
}

print_last_output "$FILE_NAME" "Inicio del procesamiento del archivo de datos:"

cd ..

echo -e "${YELLOW}Cargando los datos en la base de datos. Be patient, this may take a while.${NC}"
cd carga_datos
python3 load_reviews.py ../datos/clean_reviews.csv ../bdd/reviews.db

cd ../logs
echo -e "${GREEN}Resultado de la carga a la base de datos:${NC}"
echo "--------------------------------------------------------------------------------------"
FILE_NAME="load_to_db_data.txt"

print_last_output "$FILE_NAME" "Inicio del proceso:"

cd ..