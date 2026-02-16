# 🦀 TaskX

A console-based Task Manager built in Rust using SeaORM and MySQL.

TaskX allows you to manage tasks directly from the terminal with
persistent storage powered by MySQL. It demonstrates clean architecture
principles, async Rust, and ORM-based database interaction.

------------------------------------------------------------------------

## 🚀 Features

-   Add tasks with title and optional description\
-   List tasks (all / open / done)\
-   Mark tasks as completed\
-   Delete tasks\
-   Display status summary (total, open, done)\
-   MySQL-backed persistence\
-   Async runtime using Tokio\
-   ORM layer powered by SeaORM\
-   Environment-based configuration

------------------------------------------------------------------------

## 🧱 Tech Stack

-   Rust (stable)
-   Tokio (async runtime)
-   MySQL (database)
-   SeaORM (ORM layer)
-   dotenvy (environment configuration)

------------------------------------------------------------------------

## 📦 Requirements

-   Rust (latest stable)
-   MySQL 8+
-   Cargo

------------------------------------------------------------------------

## ⚙️ Installation

Clone the repository:

``` bash
git clone https://github.com/yourusername/taskx.git
cd taskx
```

Install dependencies:

``` bash
cargo build
```

------------------------------------------------------------------------

## 🗄️ Database Setup

Create a MySQL database:

``` sql
CREATE DATABASE taskx;
```

Create a `.env` file in the project root:

    DATABASE_URL=mysql://root:password@localhost/taskx

Replace `root` and `password` with your credentials.

------------------------------------------------------------------------

## ▶️ Running the Application

``` bash
cargo build
```

``` bash
cargo run -- <command>
```

Example:

``` bash
cargo run -- add "Finish Rust project | Use SeaORM and MySQL"
```

------------------------------------------------------------------------

## 🖥️ Available Commands

### ➕ Add Task

    add "<title> | <description>"

Example:

    add "Build CLI App | Using SeaORM"

------------------------------------------------------------------------

### 📋 List Tasks

    list [all|open|done]

Examples:

    list all
    list open
    list done

------------------------------------------------------------------------

### ✅ Complete Task

    complete <id>

Example:

    complete 3

------------------------------------------------------------------------

### 🗑️ Delete Task

    delete <id>

Example:

    delete 2

------------------------------------------------------------------------

### ❓ Help

    help

Displays all available commands.

------------------------------------------------------------------------

## 🏗️ Project Structure

    src/
     ├── main.rs
     ├── entity/
     │     ├── mod.rs
     │     └── task.rs
     ├── services/
     │     └── task_service.rs
     └── cli/
           └── parser.rs

### Architecture Layers

-   Entity Layer → SeaORM models\
-   Service Layer → Business logic\
-   CLI Layer → Command parsing\
-   Main → Application bootstrap + DB connection

------------------------------------------------------------------------

## 🧪 Testing

Run tests with:

``` bash
cargo test
```

------------------------------------------------------------------------

## 🔮 Future Improvements

-   SeaORM migrations
-   UUID-based IDs
-   Colored terminal output
-   Interactive TUI mode
-   Docker support
-   REST API version

------------------------------------------------------------------------

## 🎯 Purpose

TaskX is designed as a learning-focused yet production-structured CLI
application to practice:

-   Async Rust
-   Database integration
-   ORM patterns
-   Clean architecture
-   MySQL integration
-   CLI design

------------------------------------------------------------------------

## 📄 License

MIT License
