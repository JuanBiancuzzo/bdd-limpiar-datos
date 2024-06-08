#!/bin/bash

# Definir códigos de color ANSI
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # Sin color

echo -e "${GREEN}Creando tabla en la base de datos...${NC}"
cd bdd
sqlite3 reviews.db < create_reviews_table.sql
cd ..

echo -e "${YELLOW}Limpiando los datos. Be patient, this may take a while.${NC}"
cargo run datos/netflix_reviews.csv datos/parametros.json > /dev/null 2>&1

echo -e "${GREEN}Resultado:${NC}"
echo "--------------------------------------------------------------------------------------"
cd datos
FILE_NAME="program_data.txt"
print_last_output() {
    local last_output=""
    local start_found=0
    local line=""
    while IFS= read -r line; do
        if [[ "$line" == *"Inicio del procesamiento del archivo de datos:"* ]]; then
            start_found=1
            last_output=""
        fi
        if [ $start_found -eq 1 ]; then
            last_output+="$line\n"
        fi
    done < "$FILE_NAME"
    echo "$last_output" | sed 's/\\n/\n/g'
}
print_last_output
cd ..

echo -e "${YELLOW}Cargando los datos en la base de datos. Be patient, this may take a while.${NC}"
cd bdd
python3 load_reviews.py ../datos/clean_reviews.csv reviews.db

echo -e "${GREEN}Resultado de la carga a la base de datos:${NC}"
echo "--------------------------------------------------------------------------------------"
FILE_NAME="load_to_db_data.txt"

print_last_output_db() {
    local last_output=""
    local start_found=0
    local line=""
    while IFS= read -r line; do
        if [[ "$line" == *"Inicio del proceso:"* ]]; then
            start_found=1
            last_output=""
        fi
        if [ $start_found -eq 1 ]; then
            last_output+="$line\n"
        fi
    done < "$FILE_NAME"
    echo "$last_output" | sed 's/\\n/\n/g'
}
print_last_output_db

cd ..