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
```
## 2. Konfigurasi Environment Variables
.env
File ini menyimpan konfigurasi:

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
## 3. Docker Compose
docker-compose.db.yml
Untuk menjalankan database PostgreSQL:

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
Untuk menjalankan seluruh aplikasi dan database:

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

## 4. Setup dan Jalankan Proyek
Langkah 1: Persiapan Lingkungan
Salin file .env.example menjadi .env dan sesuaikan variabel lingkungan sesuai kebutuhan Anda.
Pastikan Docker dan Docker Compose sudah terinstal di sistem Anda.

Langkah 2: Menjalankan Database Saja
Jika Anda hanya perlu menjalankan database, gunakan file docker-compose.db.yml untuk menjalankan database:

```bash
docker-compose -f docker-compose.db.yml up -d
```

Langkah 3: Menjalankan Aplikasi dan Database
Jika Anda membutuhkan aplikasi dan database berjalan bersamaan, gunakan file docker-compose.yml untuk menjalankan seluruh aplikasi:

```bash
docker-compose up -d
```
> **Catatan**: Jika Anda hanya memerlukan database, cukup jalankan langkah 2 saja.


Langkah 4: Membangun Proyek
Sebelum menjalankan aplikasi, pastikan proyek Anda sudah dibangun terlebih dahulu. Gunakan perintah berikut untuk membangun proyek:

```bash
cargo build
```

### Langkah 5: Menjalankan Aplikasi
Setelah proyek selesai dibangun, Anda dapat menjalankan aplikasi menggunakan perintah berikut:

```bash
cargo run
```
> **Catatan**: Perintah cargo run secara otomatis akan melakukan build terlebih dahulu. Jika Anda sudah melakukan build sebelumnya, proses build bisa dilewati dengan menjalankan ```bash cargo run --release``` (untuk mode produksi) atau ```basg cargo run --no-build``` (untuk menjalankan aplikasi tanpa build ulang, jika build sudah ada).


Langkah 6: Menggunakan cargo watch untuk Monitoring Perubahan
Jika Anda ingin aplikasi berjalan otomatis setiap kali ada perubahan pada kode, Anda bisa menggunakan cargo watch untuk memonitor perubahan dan menjalankan ulang aplikasi setiap kali kode diubah:

```bash
cargo watch -x run
```
> **Catatan**: Sebelum menggunakan cargo watch, pastikan Anda sudah menginstal cargo-watch secara global. Anda bisa menginstalnya dengan perintah berikut:


```bash
cargo install cargo-watch
```
Setelah itu, Anda bisa menggunakan cargo watch untuk memonitor perubahan secara otomatis.

Langkah 7: Menjalankan Pengujian
Anda juga dapat menjalankan pengujian unit dengan perintah:

```bash
cargo test
```
