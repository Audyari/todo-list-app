# Todo List App in Rust

A simple command-line todo list application built with Rust.

## Current Status: CLI Version Complete 

## WebAssembly Version (Coming Soon) 

### Phase 1: Backend API Setup

1. **Project Structure**
   - [x] **Create new workspace for full-stack app**
     - [x] Create new workspace directory structure
     - [x] Move existing CLI app to `cli` subdirectory
     - [x] Create `Cargo.toml` for workspace root
     - [x] Create `api` crate for backend
     - [x] Create `shared` crate for common models
     - [x] Create `frontend` crate for WebAssembly
     - [x] Update workspace members in root `Cargo.toml`
     - [x] Verify all crates build successfully

2. **Database** ✅
   - [x] Set up MySQL with XAMPP
   - [x] Configure SQLx for MySQL
   - [x] Create database migrations
   - [x] Set up connection pooling

3. **API Endpoints**
   - [ ] `GET /api/tasks` - List all tasks
   - [ ] `POST /api/tasks` - Create new task
   - [ ] `PATCH /api/tasks/:id` - Update task
   - [ ] `DELETE /api/tasks/:id` - Delete task
   - [ ] Add request/response validation

### Phase 2: WebAssembly Frontend

1. **Setup**
   - [ ] Initialize Yew project
   - [ ] Set up routing
   - [ ] Configure build pipeline

2. **Components**
   - [ ] TaskList component
   - [ ] TaskItem component
   - [ ] AddTaskForm component
   - [ ] Header & Navigation

3. **State Management**
   - [ ] Implement global state with Yewdux
   - [ ] Add API service layer
   - [ ] Handle loading/error states

### Phase 3: Styling & UX

1. **UI Components**
   - [ ] Design system setup
   - [ ] Responsive layout
   - [ ] Loading states
   - [ ] Error handling UI

2. **Animations**
   - [ ] Task transitions
   - [ ] Loading indicators
   - [ ] Feedback animations

### Phase 4: Testing & Deployment

1. **Testing**
   - [ ] Unit tests for components
   - [ ] Integration tests for API
   - [ ] E2E tests with WebDriver

2. **CI/CD**
   - [ ] GitHub Actions workflow
   - [ ] Automated builds
   - [ ] Test automation

3. **Deployment**
   - [ ] Docker setup
   - [ ] Cloud deployment (Vercel/Netlify for frontend)
   - [ ] Database hosting

## Current CLI Version Details

### Completed Tasks

- [x] Initialize new Rust project
- [x] Create basic CLI interface
- [x] Implement task addition functionality
- [x] Implement task listing functionality
- [x] Implement task completion
- [x] Implement task deletion
- [x] Add data persistence
- [x] Add error handling
- [x] Add user documentation
- [x] Create PlantUML flowchart
- [x] Add comprehensive unit tests
- [x] Update documentation

### In Progress Tasks
- [ ] Fix remaining warnings
- [ ] Add input validation

### Implementation Plan

1. ~~Set up a new Rust project using Cargo~~
2. ~~Create a basic command-line interface using `clap`~~
3. ~~Implement core functionality (add, list, complete, delete tasks)~~
4. ~~Add file-based persistence~~
5. ~~Add error handling and user feedback~~
6. ~~Write documentation~~
7. ~~Add unit tests~~
8. Improve code quality and error handling

### Relevant Files
- `src/main.rs` - Main application code with tests
- `Cargo.toml` - Project dependencies and metadata
- `README.md` - Project documentation and usage instructions
- `TASKS.md` - This file, tracking development progress
- `docs/flowchart.puml` - Application workflow diagram

### Test Coverage

- **Task Management**
  - [x] Add new tasks
  - [x] List tasks in order
  - [x] Mark tasks as complete
  - [x] Delete tasks
  - [x] Handle non-existent task IDs
  
- **Data Persistence**
  - [x] Save tasks to file
  - [x] Load tasks from file
  - [x] Handle empty/missing data file

### Next Steps

1. **Code Quality**
   - [ ] Fix all compiler warnings
   - [ ] Add input validation
   - [ ] Improve error messages

2. **Features**
   - [ ] Add due dates to tasks
   - [ ] Add task categories/tags
   - [ ] Implement task search/filter
   - [ ] Add task priorities

3. **Documentation**
   - [ ] Add more examples to README
   - [ ] Document error handling
   - [ ] Add contribution guidelines

## Changelog

### 2025-06-15
- Initial project setup
- Added basic CLI interface
- Implemented core task management features
- Added JSON file-based persistence
- Added comprehensive test suite
- Created project documentation
- Added PlantUML flowchart

## Getting Started with Development

### Prerequisites
- Rust (latest stable)
- Node.js (for WebAssembly frontend)
- SQLite
- Docker (optional)

### Setup

1. Clone the repository
2. For CLI version:
   ```bash
   cargo build
   cargo run -- --help
   ```

3. For Web version (coming soon):
   ```bash
   cd frontend
   trunk serve
   ```

## License

MIT
