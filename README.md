# 📝 rust-notes

**rust-notes** is a note-taking application built in **Rust**, following the principles of **Clean Architecture**.

This project is designed as a learning and experimentation ground for building well-structured, modular applications using Rust. It separates concerns across clear architectural layers: domain, application, and infrastructure.

<p align="center">
  <img src="./docs/clean_architecture.png" alt="Clean Architecture Diagram" width="600"/>
</p>

---

## 🎯 Project Goals

- Learn and apply **Clean Architecture** in a real Rust project.
- Explore separation of concerns using traits, modules, and layered design.
- Implement basic CRUD functionality for notes.
- Make it easy to swap infrastructure (e.g. in-memory vs PostgreSQL database).

---

## 🏗️ Architecture Overview

The project follows **Clean Architecture**, splitting the application into clear layers:

```

.
├── domain          # Core business logic and entities (e.g., Note)
│   ├── entities
│   └── repositories (traits for data access)
├── application     # Use cases (e.g., CreateNote)
├── infrastructure  # Database adapters (in-memory, Postgres, etc.)
└── main.rs         # Entry point

````

Each layer depends **only on the layer below**, and **never on infrastructure**.

---

## 📦 Features (WIP)

- [x] Create notes
- [ ] List notes
- [ ] Edit notes
- [ ] Delete notes
- [ ] Switch between in-memory and PostgreSQL backends

---

## 🚀 Getting Started

### Prerequisites

- [Rust (latest stable)](https://rust-lang.org/)
- (Optional) PostgreSQL, if using the `infrastructure::db::postgres` module

### Build and Run

```bash
cargo build
cargo run
````

---

## 🧪 Tests

```bash
cargo test
```

---

## 📘 License

Apache-2.0 License. See [LICENSE](./LICENSE) for details.
