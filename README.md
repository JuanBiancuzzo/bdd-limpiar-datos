# Cómo correr el proyecto
* El proyecto corre en Linux o, en su defecto, en WSL en Windows.
* Es necesario tener instalado Rust y Python3. Puede instalarse Rust corriendo el siguiente comando:
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
* Clonar el proyecto

# Sobre el proyecto
## Sobre el process.sh
* El workflow consiste de dos pasos: limpieza de datos y carga en la base de datos. El workflow completo puede correrse corriendo el ./process.sh. Antes de correrlo, ejecutar chmod +x process.sh
* El archivo process.sh crea la base de datos en la carpeta db con el nombre reviews.db (o la usa si ya está creada), corre los dos pasos e imprime el resultado de la corrida actual.
* Los logs de los dos pasos del workflow quedan en la carpeta logs con los nombres program_data.txt (salida de la limpieza de datos) y load_to_db_data.txt (salida del upload a la base de datos).

## Sobre la limpieza de datos
* La limpieza de datos toma el archivo netflix_reviews.csv de la carpeta datos y un archivo de parámetros que indica cual es el delimitador y deja un archivo nuevo llamado clean_reviews.csv con un nuevo formato coherente con el siguiente paso del workflow.
* El proceso puede correrse yendo a la carpeta cargo run path/al/archivo/que/se/quiere/limpiar path/de/parametros. Esto deja la salida en logs/program_data.txt. Si se desea, se puede especificar un archivo de logs haciendo cargo run path/al/archivo/que/se/quiere/limpiar path/de/parametros path/de/logs

 ## Sobre la carga de datos
 * La carga de datos toma el archivo clean_reviews.csv de la carpeta datos y un archivo .db para cargar lo que lee del csv al archivo .db.
 * El proceso se puede correr entrando a la carpeta carga datos y corriendo el comando python3 load_reviews.py path/al/archivo/de/clean/reviews path/al/archivo/db.
 * El proceso deja los logs en logs/load_to_db_data.txt. Se le puede pasar un archivo distinto haciendo python3 load_reviews.py path/al/archivo/de/clean/reviews path/al/archivo/db path/del/archivo/de/logs.txt.

## Sobre la base de datos
* El archivo create_reviews_table.sql permite crear la base de datos con las tablas especificadas en el informe. Se puede correr haciendo sqlite3 reviews.db < create_reviews_table.sql. Se puede usar otro nombre que no sea reviews.db también.

  # El informe con el análisis exploratorio de datos, el gráfico de la base de datos, el link al colab y otra información acerca del proyecto se encuentra en la carpeta informe.
