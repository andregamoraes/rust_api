# Rust API with Actix-Web, Diesel, and Docker Compose

## 📌 Project Overview
This project is a **Rust-based web application** using **Actix-Web** for the backend, **Diesel** as the ORM, and **PostgreSQL** as the database. The entire stack is containerized with **Docker Compose** to ensure consistency across environments.

## 🛠️ Setup Instructions

### Prerequisites
Ensure you have the following installed:
- [Docker & Docker Compose](https://docs.docker.com/get-docker/)

### Clone the Repository
```sh
 git clone https://github.com/andregamoraes/rust_api.git
 cd rust_api
```

## Start the Database
First, start the PostgreSQL database container:
```sh
docker-compose up -d database
```

## Run Database Migrations
Once the database is running, apply the Diesel migrations:
```sh
docker-compose up -d migrate
```

## Run the Application
Finally, start the Rust application:
```sh
docker-compose up --build app
```

## Access the API
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
│── Cargo.toml         # Rust dependencies and metadata
```

## 🚀 API Endpoints
| Method | Endpoint       | Description          |
|--------|--------------|----------------------|
| GET    | /users       | Get all users       |
| GET    | /users?name=John | Get users by name |
| POST   | /users       | Create a new user   |

### 🧾 Request Body for `POST /users`

To create a user, send a JSON payload like this:

```json
{
  "name": "user",
  "email": "user@email.com",
  "password": "mypassword"
}
```

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