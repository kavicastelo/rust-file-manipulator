use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};

fn print_usage() {
    println!("Usage:");
    println!("  file_manipulator read <file_path>");
    println!("  file_manipulator write <file_path> <content>");
    println!("  file_manipulator append <file_path> <content>");
    println!("  file_manipulator copy <source_file> <destination_file>");
    println!("  file_manipulator delete <file_path>");
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_file(file_path: &str, content: &str, append: bool) -> io::Result<()> {
    let mut options = OpenOptions::new();
    options.write(true).create(true);
    if append {
        options.append(true);
    } else {
        options.truncate(true);
    }

    let mut file = options.open(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn copy_file(src: &str, dst: &str) -> io::Result<u64> {
    fs::copy(src, dst)
}

fn delete_file(file_path: &str) -> io::Result<()> {
    fs::remove_file(file_path)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "read" if args.len() == 3 => {
            let file_path = &args[2];
            match read_file(file_path) {
                Ok(contents) => println!("File contents:\n{}", contents),
                Err(e) => eprintln!("Error reading file: {}", e),
            }
        }
        "write" if args.len() == 4 => {
            let file_path = &args[2];
            let content = &args[3];
            match write_file(file_path, content, false) {
                Ok(()) => println!("Successfully wrote to file."),
                Err(e) => eprintln!("Error writing to file: {}", e),
            }
        }
        "append" if args.len() == 4 => {
            let file_path = &args[2];
            let content = &args[3];
            match write_file(file_path, content, true) {
                Ok(()) => println!("Successfully appended to file."),
                Err(e) => eprintln!("Error appending to file: {}", e),
            }
        }
        "copy" if args.len() == 4 => {
            let src = &args[2];
            let dst = &args[3];
            match copy_file(src, dst) {
                Ok(_) => println!("Successfully copied the file."),
                Err(e) => eprintln!("Error copying the file: {}", e),
            }
        }
        "delete" if args.len() == 3 => {
            let file_path = &args[2];
            match delete_file(file_path) {
                Ok(()) => println!("Successfully deleted the file."),
                Err(e) => eprintln!("Error deleting the file: {}", e),
            }
        }
        _ => {
            print_usage();
        }
    }
}
