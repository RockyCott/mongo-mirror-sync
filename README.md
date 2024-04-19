
# Mongo Mirror Sync

Este es un proyecto de una herramienta que simplifica el uso de mongodump y mongorestore para la sincronización de bases de datos MongoDB. Es especialmente útil en casos donde se gestionan múltiples bases de datos con configuraciones similares, como el caso de mantener actualizadas las bases de datos demás bases con respecto a una principal.

Mongo Mirror Sync es una herramienta cli desarrollada en [Rust](https://www.rust-lang.org/) + [Ratatui](https://ratatui.rs/) = ❤️ que simplifica el uso de mongodump y mongorestore para el proceso de sincronización de bases de datos MongoDB. Permite tener la gestión de los valores para las flags de dichos comandos más fácil.Esto con el fin de poder seleccionar fácilmente las bases de datos de origen y destino para la sincronización, lo que facilita la gestión de múltiples entornos de desarrollo para tener la misma data en cualquier momento.


## Dependencias

- Tener Rust instalado en el sistema.
- Tener instalado **MongoDB Database Tools**, ya que la herramienta utiliza las utilidades **mongodump** y **mongorestore** para realizar la sincronización de bases de datos. Puedes instalar MongoDB Database Tools siguiendo las instrucciones en la [documentación oficial de MongoDB](https://www.mongodb.com/docs/database-tools/installation/installation/).
- Tener acceso a las bases de datos MongoDB con los suficientes privilegios para realizar copias y sobreescribir la data a sincronizar.

## Instalación

### Instalación a través de cargo install
También puedes instalar MongoDB Mirror Sync directamente desde crates.io utilizando cargo:

```shell
cargo install mongo-mirror-sync
```
Esto instalará la herramienta globalmente en tu sistema y podrás ejecutarla desde cualquier lugar en tu terminal.

### Clonando el repositorio
- Clona este repositorio en tu máquina local.

    ```shell
    git clone https://github.com/RockyCott/mongo-mirror-sync.git
    ```
- Navega hasta el directorio del repositorio.
  
    ```shell
    cd mongo-mirror-sync
    ```
- Ejecuta `cargo build --release` para compilar el proyecto.
  
    ```shell
    cargo build --release
    ```

## Ejecución
- Una vez compilado, ejecuta el binario generado en el directorio target/release.
- Sigue las instrucciones en pantalla para seleccionar las bases de datos de origen y destino, así como cualquier otra configuración necesaria.

## Seguridad y Privacidad
Mongo Mirror Sync respeta la privacidad y seguridad de tus datos. Toda la información, incluidos los nombres de las bases de datos, entornos y credenciales, se almacena localmente en tu equipo. En ningún momento se envía esta información fuera de tu sistema. Puedes estar seguro de que tus datos están protegidos y no serán compartidos con terceros a través de esta herramienta.

## Contribuciones
Las contribuciones son bienvenidas. Si tienes alguna idea para mejorar esta herramienta, siéntete libre de abrir un issue o enviar un pull request.

