# Web Server Project Template Documentation with Actix Web

## 1. Environment Variables Configuration

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
## 2. Docker Compose Configuration

```docker-compose.db.yml```
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

```docker-compose.yml```
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

## 3. Setup and Run the Project
### step 1: Prepare the Environment
Copy the ```.env.example``` file to ```.env``` and adjust the environment variables according to your needs.
Ensure that Docker and Docker Compose are installed on your system.

### step 2: Run Database Only
If you only need to run the database, use the ```docker-compose.db.yml``` file to start the database:

```bash
docker-compose -f docker-compose.db.yml up -d
```

### step 3: Run the Application and Database
If you need both the application and database running together, use the ```docker-compose.yml``` file to start the entire application:

```bash
docker-compose up -d
```

### step 4: Migration and Build the Project

SQLx CLI is used to manage database migrations and ensure the database structure matches the application's requirements. Below is a detailed guide on how to set up, perform migrations, and build your project.

#### 1. Install SQLx CLI
```bash
cargo install sqlx-cli --features postgres
```

#### 2. Add a Migration
Create a new migration using the SQLx CLI. Use the following command to generate a new migration file:

```bash
cargo migrate add -r <migration_name>
```

#### 3. Write SQL for the Migration
Edit the migration files to define the changes you want to make to your database.

```up``` migration
```sql
-- Add up migration script here
CREATE TABLE users (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    password VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

```down``` migration
```sql
-- Add down migration script here
DROP TABLE IF EXISTS users;
````

#### 4. Run the Migrations and Revert
Run the migrations to apply the changes to your database. Use the following command:

```bash
sqlx migrate run
sqlx migrate revert
```

#### 5. Build the Project
After setting up the migrations, build your project to ensure everything is correctly configured:

```bash
cargo build
```

### step 5: Run the Application
Once the project is built, you can run the application with the following command:

```bash
cargo run
```
> **Note**: The ```cargo run``` command will automatically build the project first. If you have already built the project, you can skip the build step by running
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
## 4. Suggestions and Feedback

This documentation is made to help you understand how to run this project. If anything is unclear or if there's a simpler way to explain something, I would really appreciate it if you could provide feedback.

Since I’m still a beginner, I’m totally open to suggestions or constructive criticism about this Actix Web project template or the documentation I’ve created. Is there anything that can be improved or added? I really want to learn more and make this project better.

Feel free to leave comments or suggestions. Thanks a lot!
