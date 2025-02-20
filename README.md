# TODO App

This is a simple yet powerful TODO application built in Rust. The app helps users track tasks with features like task creation, updating, deletion, and list display. The backend is developed using **Tokio**, **Axum**, and **Serde** for asynchronous and HTTP request handling, alongside other utilities for formatting and time management.

---

## Features

- Create new TODO tasks with titles and optional descriptions.
- List all existing tasks with their statuses and creation timestamps.
- Update task details, including marking tasks as completed or modifying descriptions.
- Delete tasks you no longer need.
- Nice and interactive terminal output using **Crossterm** and **Comfy-Table**.

---

## Installation

To run this project locally, ensure you have **Rust (1.84.1)** or a compatible version installed. Clone the repository and build the project:

```bash
# Clone the repository
git clone <repo-url>
cd <repo-name>

# Run the project
cargo run
```

---

## Technologies Used

- **[Tokio](https://tokio.rs/):** Foundation for managing asynchronous tasks and HTTP requests.
- **[Axum](https://docs.rs/axum):** Lightweight HTTP framework used for routing and endpoint handling.
- **[Serde](https://serde.rs/):** For serialization and deserialization of tasks and other data.
- **[Chrono](https://docs.rs/chrono):** For time tracking (e.g., task creation timestamps).
- **[Crossterm](https://docs.rs/crossterm):** For interactive and colored terminal output.
- **[Comfy-Table](https://docs.rs/comfy-table):** For displaying task lists in structured and styled tables.

---

## HTTP API Endpoints

The app provides a RESTful API interface to manage tasks.

### **GET** `/tasks`
Retrieve a list of all tasks.

**Response example:**
```json
[
  {
    "id": 1,
    "title": "Learn Rust",
    "description": "Complete Rust book chapters 5-8",
    "completed": false,
    "created_at": "2023-10-01T12:00:00Z"
  },
  {
    "id": 2,
    "title": "Build TODO App",
    "description": "Use Axum framework to build a TODO app",
    "completed": true,
    "created_at": "2023-10-02T15:25:00Z"
  }
]
```

---

### **POST** `/tasks`
Create a new task.

**Request body (JSON):**
```json
{
  "title": "New Task",
  "description": "Optional description here"
}
```

**Response example:**
```json
{
  "id": 3,
  "title": "New Task",
  "description": "Optional description here",
  "completed": false,
  "created_at": "2023-10-10T08:45:00Z"
}
```

---

### **PUT** `/tasks/{id}`
Update an existing task's details.

**Request body (JSON):**
```json
{
  "title": "Updated Task Title",
  "description": "Updated optional description",
  "completed": true
}
```

**Response example:**
```json
{
  "id": 3,
  "title": "Updated Task Title",
  "description": "Updated optional description",
  "completed": true,
  "created_at": "2023-10-10T08:45:00Z"
}
```

---

### **DELETE** `/tasks/{id}`
Delete a task by its ID.

**Response example:**
```json
{
  "message": "Task successfully deleted!"
}
```

---

## Running Tests

The app includes unit and integration tests. Run them with:

```bash
cargo test
```

---
