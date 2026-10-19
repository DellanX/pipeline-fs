# Run Tasks for Pipeline-FS

This project includes run tasks for both the UI and Rust service. These tasks can be used in VS Code or via command line.

## UI Tasks

### Building the UI
- `npm run build` - Build the UI for production
- `npm run dev` - Start development server (Vite)
- `npm run preview` - Preview the built project

## Rust Service Tasks

### Building the Service
- `cargo build` - Build the service in debug mode
- `cargo run` - Build and run the service

### Running the Service
- `run-rust-service-dev` - Run the service using cargo run (development mode)
- `run-rust-service` - Run the pre-built binary with default mount points

## Combined Tasks

### Running Both Services Together
- `run-all` - Run both UI and Rust service together (via script)

## How to Use in VS Code

1. Open the Command Palette (`Ctrl+Shift+P` or `Cmd+Shift+P`)
2. Type "Tasks: Run Task"
3. Select from the available tasks:
   - `build-ui`
   - `run-ui-dev`
   - `run-ui-preview`
   - `build-rust-service`
   - `run-rust-service-dev`
   - `run-rust-service`
   - `run-all`

## Prerequisites

Before running the services, make sure you have:
- Node.js and npm installed for UI
- Rust and Cargo installed for the service
- Required directories created (e.g., `/tmp/stage0`, `/tmp/stage1`)

## Directory Setup

Create the required directories before running:
```bash
mkdir -p /tmp/stage0 /tmp/stage1
```

The default mount point is `/tmp/test-mount`.