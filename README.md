# Command line todo list tool
Write in Rust Programming Language

## Features
- Tasks list
    - add, remove, mark as done, mark undone
    - clear done tasks
- Todo state save in files

## How to Build from source
1. Ensure you have Rust installed
2. Clone the repository
   ```shell
    git clone https://github.com/OmegaOoh/todo-cmd.git
   ```
4. Use cargo to build the project
   ```shell
    cargo build --release
   ```
5. Run the application `/target/release/todo-cmd`

### Optional
Make the app executable to your system's console/terminal
```shell
cargo install --path
```
