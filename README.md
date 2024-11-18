# Dokumentasi Proyek Web Server dengan Actix Web

## 1. Struktur Proyek

Berikut adalah struktur direktori proyek:

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
2. Konfigurasi Environment Variables
.env
File ini menyimpan konfigurasi lokal:

env
Salin kode
# Environment Configuration
APP_ENV=development
APP_PORT=8080
APP_HOST=127.0.0.1
APP_BASE_URL=http://localhost:8080

# JWT Configuration
JWT_SECRET_KEY=your_jwt_secret_key
JWT_EXPIRATION_TIME=3600

# PostgreSQL Database Configuration
POSTGRES_USER=postgres
POSTGRES_PASSWORD=password
POSTGRES_DB=my_database
DB_HOST=localhost
DB_PORT=5432

DATABASE_URL=postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${DB_HOST}:${DB_PORT}/${POSTGRES_DB}?sslmode=prefer

# SSL Configuration
CERT_FILE=certs/cert.pem
KEY_FILE=certs/private_key.pem
.env.example
Contoh file .env untuk dokumentasi:

env
Salin kode
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
3. Docker Compose
docker-compose.db.yml
Untuk menjalankan database PostgreSQL:
yaml
Salin kode
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

docker-compose.yml
Untuk menjalankan seluruh aplikasi dan database:

yaml
Salin kode
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


4. Setup dan Jalankan Proyek
Langkah 1: Persiapan Lingkungan
Salin .env.example ke .env dan sesuaikan variabel sesuai kebutuhan.
Pastikan Docker dan Docker Compose terinstal di sistem Anda.
Langkah 2: Menjalankan Database Saja
Gunakan file docker-compose.db.yml untuk menjalankan database:

bash
Salin kode
docker-compose -f docker-compose.db.yml up -d
Langkah 3: Menjalankan Aplikasi dan Database
Gunakan file docker-compose.yml untuk menjalankan seluruh aplikasi:

bash
Salin kode
docker-compose up -d
Langkah 4: Menjalankan Pengujian
Jalankan pengujian unit dengan perintah:

bash
Salin kode
cargo test

```
