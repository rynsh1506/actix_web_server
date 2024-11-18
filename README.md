# Web Server Project Template Documentation with Actix Web

## 1. Project Structure

Here’s the directory structure for the project:

```plaintext
web_server/
├── .github/
│   └── workflows/
│       └── simple-ci.yml
├── certs/
│   ├── cert.pem
│   ├── generate.txt
│   └── private_key.pem
├── migrations/
├── src/
│   ├── actor/
│   │   └── mod.rs
│   ├── config/
│   │   ├── app_config.rs
│   │   ├── builder.rs
│   │   ├── certs_config.rs
│   │   ├── config_loader.rs
│   │   ├── db_config.rs
│   │   └── mod.rs
│   ├── db/
│   │   ├── connection.rs
│   │   └── mod.rs
│   ├── handlers/
│   │   └── mod.rs
│   ├── middleware/
│   │   ├── logger.rs
│   │   └── mod.rs
│   ├── models/
│   │   └── mod.rs
│   ├── routes/
│   │   ├── health_check.rs
│   │   └── mod.rs
│   ├── utils/
│   │   ├── errors.rs
│   │   ├── init_logger.rs
│   │   └── mod.rs
│   ├── lib.rs
│   ├── main.rs
│   └── server.rs
├── target/
├── tests/
│   ├── health_check.rs
│   ├── main.rs
│   └── test_establish_connection.rs
├── .env
├── .env.example
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── docker-compose.db.yml
├── docker-compose.yml
├── Dockerfile
├── LICENSE
└── README.md
```
## 2. Environment Variables Configuration

This file contains configuration values for the project:

```env
# Environment Configuration
APP_ENV=
APP_PORT=
APP_HOST=
APP_BASE_URL=

# JWT Configuration
JWT_SECRET_KEY=
JWT_EXPIRATION_TIME=

# PostgreSQL Database Configuration
POSTGRES_USER=
POSTGRES_PASSWORD=
POSTGRES_DB=
DB_HOST=
DB_PORT=

DATABASE_URL=postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${DB_HOST}:${DB_PORT}/${POSTGRES_DB}?sslmode=prefer

# SSL Configuration
CERT_FILE=
KEY_FILE=
```
## 3. Docker Compose Configuration

docker-compose.db.yml
To run the PostgreSQL database:

```yml
version: "3.8"

services:
  db:
    image: postgres:13
    container_name: postgres
    environment:
      POSTGRES_USER: ${POSTGRES_USER}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: ${POSTGRES_DB}
    ports:
      - "${DB_PORT}:${DB_PORT}"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    networks:
      - mynetwork

networks:
  mynetwork:
    driver: bridge

volumes:
  postgres_data:
```

docker-compose.yml
To run the entire application and database:

``` yml
version: '3.8'

services:
  web:
    build: .
    image: web_server:latest
    ports:
      - "${APP_PORT}:${APP_PORT}"
    depends_on:
      - db
    environment:
      DATABASE_URL: ${DATABASE_URL}
      APP_ENV: ${APP_ENV}
      JWT_SECRET_KEY: ${JWT_SECRET_KEY}
      JWT_EXPIRATION_TIME: ${JWT_EXPIRATION_TIME}
    networks:
      - mynetwork

  db:
    image: postgres:latest
    container_name: postgres
    environment:
      POSTGRES_USER: ${POSTGRES_USER}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: ${POSTGRES_DB}
    ports:
      - "${DB_PORT}:${DB_PORT}"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    networks:
      - mynetwork

networks:
  mynetwork:
    driver: bridge

volumes:
  postgres_data:
```

## 4. Setup and Run the Project
### step 1: Prepare the Environment
Copy the .env.example file to .env and adjust the environment variables according to your needs.
Ensure that Docker and Docker Compose are installed on your system.

### step 2: Run Database Only
If you only need to run the database, use the docker-compose.db.yml file to start the database:

```bash
docker-compose -f docker-compose.db.yml up -d
```

### step 3: Run the Application and Database
If you need both the application and database running together, use the docker-compose.yml file to start the entire application:

```bash
docker-compose up -d
```


### step 4: Build the Project
Before running the application, make sure the project is built first. Use the following command to build the project:

```bash
cargo build
```

### step 5: Run the Application
Once the project is built, you can run the application with the following command:

```bash
cargo run
```
> **Note**: The ```bash cargo run``` command will automatically build the project first. If you have already built the project, you can skip the build step by running
> ```bash
> cargo run --release
> ```
> Or, to run without rebuilding the project (if the build already exists):
> ```bash
> cargo run --no-build
> ```


### step 6: Use Cargo Watch for Auto-Reload
If you want the application to automatically reload whenever there’s a change in the code, you can use ```cargo watch``` to monitor file changes and restart the application:

```bash
cargo watch -x run
```
> **Note**: Before using ```cargo watch```, make sure you have it installed globally. You can install it with the following command:


```bash
cargo install cargo-watch
```
Once installed, you can use cargo watch to automatically monitor changes.

### step 7: Run Unit Tests
You can also run unit tests with the following command:

```bash
cargo test
```
## 5. Suggestions and Feedback

This documentation is made to help you understand how to run this project. If anything is unclear or if there's a simpler way to explain something, I would really appreciate it if you could provide feedback.

Since I’m still a beginner, I’m totally open to suggestions or constructive criticism about this Actix Web project template or the documentation I’ve created. Is there anything that can be improved or added? I really want to learn more and make this project better.

Feel free to leave comments or suggestions. Thanks a lot!
