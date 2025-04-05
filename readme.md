# Rust File Manipulator CLI Tool

This Rust-based CLI tool allows users to manipulate files through various operations like reading, writing, appending, copying and deleting
files in a directory. The tool is designed to be user-friendly, with clear command-line instructions.

## Features

- **Read a file**: Displays the contents of a file.
- **Write to a file**: Writes a string to a file.
- **Append to a file**: Appends a string to a file.
- **Copy a file**: Copies a file.
- **Delete a file**: Deletes a file.

## Prerequisites

- **Rust & Cargo:**he tool is built with Rust, so Cargo is needed for compilation. If you haven't installed Rust and
  Cargo, follow the [official installation guide](https://www.rust-lang.org/tools/install).

## Installation

To compile the tool into an executable:

```bash
cargo build --release
```

The executable will be located in the `target/release` directory.

## Running the Tool

### Using the Executable Directly

You can run the executable directly from the command line. Below are examples of common commands:

1. Display Help:

    ```bash
    ./file_manipulator --help
    ```

2. Read a File:

    ```bash
    ./file_manipulator --read src/file.txt
    ```

3. Write to a File:

    ```bash
    ./file_manipulator --write src/file.txt "This is new content"
    ```

4. Append to a File:

    ```bash
    ./file_manipulator --ppend src/file.txt "This content is append"
    ```

5. Copy a File:

    ```bash
    ./file_manipulator --copy src/file.txt dst/file.txt
    ```

6. Delete a File:

    ```bash
    ./file_manipulator --delete src/file.txt
    ```
   
### Batch File for Easy Execution
To simplify running the tool, you can create a batch file (file_manipulator.bat) in the same directory as the executable:

```bat
@echo off
file_manipulator.exe %*
```
This allows you to run the tool with commands like:

```bash
./file_manipulator.bat -r "path_to_file"
```

### Adding to PATH
1. For easier access from any directory, add the executable's directory to your system's `PATH`:
2. Right-click `This PC` and select `Properties`.
3. Click `Advanced system settings`.
4. Click `Environment Variables`.
5. In the `System variables` section, find the `Path` variable and click `Edit`.
6. Click `New` and add the path to your executable (e.g., `C:\path\to\file_manipulator`).
7. Click `OK` to close all windows.

After this, you can run `file_manipulator.exe` from any command prompt window.

### Example Usage
Reading a File
```bash
file_manipulator.exe -r "C:\example\file.txt"
```

Writing to a File
```bash
file_manipulator.exe -w "C:\example\file.txt" "This is new content"
```

Appending to a File
```bash
file_manipulator.exe -a "C:\example\file.txt" "This content is append"
```

Copying a File
```bash
file_manipulator.exe -c "C:\example\file.txt" "C:\example\file_copy.txt"
```

Deleting a File
```bash
file_manipulator.exe -d "C:\example\file.txt"
```

## Packaging for Distribution
To distribute this tool without requiring Rust and Cargo on the user's machine:
1. Compile the tool with `cargo build --release`.
2. Package the `file_manipulator.exe` and any necessary documentation (like this `README.md`) into a ZIP file.
3. Distribute the ZIP file

## Automated Script
Create an automated script (`package.bat`) for building and packaging:

```bat
@echo off
cargo build --release
mkdir package
copy target\release\file_manipulator.exe package\
copy README.md package\
cd package
zip -r file_manipulator.zip .
cd ..
rmdir /s /q package
```

Running `package.bat` will generate a file_manipulator.zip ready for distribution.

## Contributing
Feel free to fork this project and submit pull requests. Contributions are welcome!

## License
This project is licensed under the MIT license.
