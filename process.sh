#!/bin/bash

# Definir códigos de color ANSI
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # Sin color

# Función para imprimir el último resultado basado en un patrón de inicio
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

# Función para imprimir el número de paso y el horario de inicio
print_step_info() {
    local step_number=$1
    local total_steps=$2
    local description=$3
    local color=$4
    echo -e "${color}Paso $step_number/$total_steps: $description - $(date +"%Y-%m-%d %H:%M:%S")${NC}"
}

total_steps=5
step=1

# Paso 1: Verificar si el archivo de base de datos existe
archivo="bdd/reviews.db"

if [ -e "$archivo" ]; then
    print_step_info $step $total_steps "Base de datos detectada. Usando base de datos existente." "$GREEN"

else
    print_step_info $step $total_steps "Base de datos no detectada. Creando la base de datos." "$GREEN"

    cd bdd
    sqlite3 reviews.db < create_reviews_table.sql
    cd ..
fi
((step++))

# Paso 2: Limpiar los datos
print_step_info $step $total_steps "Limpiando los datos. Be patient, this may take a while." "$YELLOW"
cd limpieza_datos
#echo -e "${YELLOW}Limpiando los datos. Be patient, this may take a while.${NC}"
cargo run ../datos/netflix_reviews.csv ../datos/parametros.json > /dev/null 2>&1

echo -e "${GREEN}Resultado de la limpieza de datos:${NC}"
echo "--------------------------------------------------------------------------------------"
cd ../logs
FILE_NAME="program_data.txt"

print_last_output "$FILE_NAME" "Inicio del procesamiento del archivo de datos:"
cd ..
((step++))

# Paso 3: Cargar los datos en la base de datos
print_step_info $step $total_steps "Cargando los datos en la base de datos. Be patient, this may take a while." "$YELLOW"
#echo -e "${YELLOW}Cargando los datos en la base de datos. Be patient, this may take a while.${NC}"
cd carga_datos
python3 load_reviews.py ../datos/clean_reviews.csv ../bdd/reviews.db

cd ../logs
echo -e "${GREEN}Resultado de la carga a la base de datos:${NC}"
echo "--------------------------------------------------------------------------------------"
FILE_NAME="load_to_db_data.txt"

print_last_output "$FILE_NAME" "Inicio del proceso:"
cd ..
((step++))

echo -e "${GREEN}Proceso completado. Puede encontrar los logs del proceso en la carpeta logs.${NC}"