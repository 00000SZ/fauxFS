# fauxFS

fauxFS is a utility to rapidly generate a large number of random files and directories. It includes options to specify the number of files, maximum file size, and an option to generate files with the EICAR Anti-Malware test file signature.

## Features

- Create a specified number of random files and directories.
- Specify the maximum size for each file.
- Option to generate files with the EICAR Anti-Malware test file signature.
- Displays progress with a spinner and progress bar.
- Provides statistics on the total size, average file size, and inode usage.

Let's see it in action

![Demo](https://github.com/00000sz/fauxFS/blob/master/.img/demo.gif)

## Build Instructions

### Prerequisites

- Rust and Cargo (install from [rust-lang.org](https://www.rust-lang.org/tools/install))

### Steps

1. Clone the repository:

   ```sh
   git clone https://github.com/yourusername/fauxFS.git
   cd fauxFS
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

3. The binary will be available at `target/release/fauxFS`.

## Usage

```sh
./target/release/fauxFS <base_path> <count> <max_size> [--eicar]
```

### Parameters

- `base_path`: The base directory where files and directories will be created.
- `count`: The number of files to create.
- `max_size`: The maximum size of each file in bytes.
- `--eicar`: Optional flag to generate files with the EICAR Anti-Malware test file signature.

### Example

```sh
./target/release/fauxFS /path/to/base_directory 10000 1024
```

This will create 10,000 random files with a maximum size of 1 KB each in the specified base directory.

```sh
./target/release/fauxFS /path/to/base_directory 10000 1024 --eicar
```

This will create 10,000 files with the EICAR signature and random padding up to 1 KB each.

## Confirmation Prompt

Before starting the file creation, the script will prompt for confirmation if the specified parameters might lead to an excessive number of files or large file sizes.

```sh
You are about to create 10,000 files with a maximum size of 1.00 KB each.
Do you want to proceed? (yes/no):
```

## Output

- Progress bar indicating the file creation progress.
- Summary of the creation process including the total size, average file size, and inode usage.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
