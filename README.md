# Todo List App

A simple command-line todo list application built with Rust.

## Features

- [x] Add new tasks
- [x] List all tasks
- [x] Mark tasks as complete
- [x] Delete tasks
- [x] Persistent storage (saves to JSON file)
- [x] Comprehensive unit tests
- [x] Flowchart documentation

## Prerequisites

- Rust (latest stable version recommended)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd todo-list-app
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Add a new task
```bash
cargo run -- add "Your task description here"
```

### List all tasks
```bash
cargo run -- list
```

### Mark a task as complete
```bash
cargo run -- complete <task_id>
```

### Delete a task
```bash
cargo run -- delete <task_id>
```

## Running Tests

To run the test suite:

```bash
cargo test
```

This will execute all unit tests with test coverage for core functionality.

## Project Structure

- `src/main.rs` - Main application code
- `tests/` - Unit tests (in main.rs with `#[cfg(test)]`)
- `docs/flowchart.puml` - PlantUML flowchart of application logic
- `Cargo.toml` - Project configuration and dependencies

## Data Storage

Tasks are automatically saved to a JSON file in your system's application data directory:
- Windows: `%APPDATA%\todo-list-app\tasks.json`
- macOS: `~/Library/Application Support/todo-list-app/tasks.json`
- Linux: `~/.local/share/todo-list-app/tasks.json`

## Development

### Adding New Features

1. Create a new branch for your feature
2. Add tests for the new functionality
3. Implement the feature
4. Run tests: `cargo test`
5. Update documentation as needed
6. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
