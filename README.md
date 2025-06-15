# Full-Stack Todo List App

A full-stack todo list application built with:
- **Backend**: Rust + Axum + SQLx
- **Frontend**: WebAssembly + Yew (Coming Soon)
- **Database**: MySQL (via XAMPP)

## Features

### Backend (Rust)
- [x] RESTful API with Axum
- [x] MySQL database integration with SQLx
- [x] Database migrations
- [x] CRUD operations for tasks
- [x] Error handling and validation

### Frontend (Coming Soon)
- [ ] WebAssembly frontend with Yew
- [ ] Task listing and management
- [ ] Real-time updates

### CLI (Existing)
- [x] Add new tasks
- [x] List all tasks
- [x] Mark tasks as complete
- [x] Delete tasks
- [x] Persistent storage (JSON)

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- XAMPP (for MySQL)
- Node.js & npm (for frontend, coming soon)

## Getting Started

### Backend Setup

1. Start XAMPP and ensure MySQL is running
2. Create a database named `todo_app` in phpMyAdmin
3. Clone the repository:
   ```bash
   git clone <repository-url>
   cd todo-list-app