# Rusty Journal

A command-line todo application written in Rust that helps you manage your tasks efficiently.

## Features

- Add new tasks
- Mark tasks as completed
- List all incomplete tasks
- List completed tasks
- Edit existing tasks
- Persistent storage in JSON format
- Default storage location: `~/.rusty-journal.json`

## Prerequisites

- Rust programming language (1.56.0 or later)
- Cargo package manager
- Git (for cloning the repository)

You can install Rust and Cargo by following the instructions at [https://rustup.rs/](https://rustup.rs/)

Make sure you have Rust installed on your system. Then clone this repository and build the project:

## Installation

```bash
git clone https://github.com/yourusername/rusty-journal.git
cd rusty-journal
cargo build --release
```

## Usage Examples

### Adding a New Task

```bash
rusty-journal add "Buy groceries"
```

### List all incomplete tasks

```bash
rusty-journal list
```

### Compplete a task

```bash
rusty-journal done 1
```

### List completed tasks

```bash
rusty-journal list-completed
```

### Edit a task

```bash
rusty-journal edit 1 "Buy milk"
```

### Using a Custom Journal File

You can specify a custom file location using the `-j` or `--journal-file` option:

```bash
rusty-journal -j /path/to/my-journal.json add "Custom file task"
```

## Command Details

- `add <text>`: Add a new task with the specified text
- `done <position>`: Mark the task at the given position as complete (shows confirmation prompt)
- `list`: Display all incomplete tasks
- `list-completed`: Display all completed tasks
- `edit <position> <new-text>`: Modify the text of an existing incomplete task

## File Format

Tasks are stored in JSON format with the following information:

- Task text
- Creation timestamp
- Completion timestamp (if completed)

## License

This project is licensed under the MIT License - see the LICENSE file for details.
