# Rust Project with Actix-Web, Diesel, and Docker Compose

## 📌 Project Overview
This project is a **Rust-based web application** using **Actix-Web** for the backend, **Diesel** as the ORM, and **PostgreSQL** as the database. The entire stack is containerized with **Docker Compose** to ensure consistency across environments.

## 🛠️ Setup Instructions

### 1️⃣ Prerequisites
Ensure you have the following installed:
- [Docker & Docker Compose](https://docs.docker.com/get-docker/)

### 2️⃣ Clone the Repository
```sh
 git clone https://github.com/andregamoraes/rust_api.git
 cd rust_api
```

## 3️⃣ Start the Database
First, start the PostgreSQL database container:
```sh
docker-compose up -d database
```

## 4️⃣ Run Database Migrations
Once the database is running, apply the Diesel migrations:
```sh
docker-compose up -d migrate
```
If you need to revert a migration:
```sh
diesel migration revert
```

## 5️⃣ Run the Application
Finally, start the Rust application:
```sh
docker-compose up --build app
```
```

### 6️⃣ Access the API
The backend runs on **http://localhost:8080** (or the port defined in your configuration).

## 📂 Project Structure
```
│── src/
│   ├── main.rs        # Entry point of the application
│   ├── routes.rs    # Route handlers
│   ├── models.rs      # Database models
│   ├── schema.rs      # Diesel schema (auto-generated)
│── migrations/        # Diesel migration files
│── Dockerfile         # Docker container configuration
│── docker-compose.yml # Docker Compose setup
│── .env               # Environment variables
│── Cargo.toml         # Rust dependencies and metadata
```

## 🚀 API Endpoints
| Method | Endpoint       | Description          |
|--------|--------------|----------------------|
| GET    | /users       | Get all users       |
| GET    | /users?name=John | Get users by name |
| POST   | /users       | Create a new user   |

## 🛑 Stopping and Cleaning Up
To stop the running containers:
```sh
docker-compose down
```

To remove all containers, networks, and volumes:
```sh
docker-compose down -v
```

## 📜 License
This project is licensed under the MIT License.

---
_Developed with Rust, Diesel, and Docker_ 🦀🔥