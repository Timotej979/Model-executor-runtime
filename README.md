# Model-executor-runtime

## Basic information

Model executor runtime (MER) written in Rust. The project consists of two parts:

- **Migrations**
    - Program that migrates the MER-DB. The DB includes available servers running the different models to tell the driver whether the servers are remot (SSH-connection) or local, model access logs when certain models were used and specific model weight vectors that are used for available models.

- **Driver**
    - Program that primarily runs the MER-repl with which we interact (either manually or via external command scheduler). The repl is connected to different abstraction layers like the Data-Abstraction-Layer (DAL), Model-Executor-Abstraction-Layer (MEAL) and so on.

Available model configurations are written to the DB on migration, they can be changed on runtime via the REPL if the correct command flag is set on starting the driver.


## Project structure

THese are the relevant folders within the project:

- **migrations (Rust project)**
    - `sql` - Folder with all SQL migration scripts. The format in which the scripts should be named is: ```XX-NameOfTheScript.sql```, where:
        - `XX` -  The consecutive number of the migration, for example 00, 01, 02, 03, ...
        - `NameOfTheScript` - The name of the migration script, usually something meaningfull. The file should always end with the `.sql` file type if not the script won't execute it.
    - `src/main.rs` - The rust migration script of the project
    - `target` - Automaticaly generated folder using cargo build for rust binaries
    - `Cargo.lock` - Cargo file that keeps the list of locked packages and their remote sources
    - `Cargo.toml` - Cargo file that specifies which versions of packages that should be used
    - `Dockerfile` - Multistage build docker-file that runs the migrations in a custom small size container (Not really relevant for migrations, but maybe more so for the driver)

- **driver (Rust project)**
    - `src/main.rs` - The rust driver script which initializes and uses all the submodules like: repl, dal, meal and so on...
    - `target` - Automaticaly generated folder using cargo build for rust binaries
    - `Cargo.lock` - Cargo file that keeps the list of locked packages and their remote sources
    - `Cargo.toml` - Cargo file that specifies which versions of packages that should be used
    - `Dockerfile` - Multistage build docker-file that runs the driver in a custom small size container

- **startStopScripts (Automation bash scripts)**
    - `startMER.sh` - Script that starts the MER DB, runs the migrations binary and runs the driver binary
    - `stopMER.sh` - Script that stops the MER DB and driver
    - `testDriver.sh` - Script that only runs the driver binary

- **.env**
    - Example environment variables configuration file for this project

- **docker-compose.yaml**
    - Docker-compose file for the SurrealDB
