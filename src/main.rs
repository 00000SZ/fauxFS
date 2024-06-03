use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use rand::Rng;
use std::time::Instant;
use std::process::Command;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

const EICAR_SIGNATURE: &str = "X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*";

fn create_random_file(path: &Path, max_size: usize, use_eicar: bool) -> std::io::Result<usize> {
    let mut file = File::create(path)?;

    if use_eicar {
        let padding_size = max_size.saturating_sub(EICAR_SIGNATURE.len());
        let mut data = EICAR_SIGNATURE.as_bytes().to_vec();
        data.extend((0..padding_size).map(|_| rand::random::<u8>()));
        file.write_all(&data)?;
        Ok(data.len())
    } else {
        let size = rand::thread_rng().gen_range(1..=max_size);
        let data: Vec<u8> = (0..size).map(|_| rand::random::<u8>()).collect();
        file.write_all(&data)?;
        Ok(size)
    }
}

fn create_random_files_and_dirs(base_path: &Path, count: usize, max_size: usize, use_eicar: bool, pb: &ProgressBar) -> std::io::Result<(usize, u64)> {
    let mut total_size = 0;
    let start = Instant::now();

    for i in 0..count {
        let subdir_path = base_path.join(format!("dir_{}", i));
        fs::create_dir_all(&subdir_path)?;

        let file_path = subdir_path.join(format!("file_{}.bin", i));
        let file_size = create_random_file(&file_path, max_size, use_eicar)?;
        total_size += file_size;

        pb.inc(1); // Increment the progress bar
    }

    let duration = start.elapsed();
    let inodes = count * 2; // One inode for each file and one for each directory

    pb.finish_with_message("File generation completed");
    println!("{}", format!("Created {} files and directories in {:?}", format_number(count), duration).yellow());
    println!("{}", format!("Total size: {}", human_readable_size(total_size)).yellow());
    println!("{}", format!("Average file size: {}", human_readable_size(total_size / count)).yellow());
    println!("{}", format!("Total inodes used: {}", format_number(inodes)).yellow());
    println!("{}", "------------------------------------------".blue());

    Ok((total_size, inodes as u64))
}

fn human_readable_size(bytes: usize) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    format!("{:.2} {}", size, UNITS[unit])
}

fn format_number(num: usize) -> String {
    let num_str = num.to_string();
    let chars: Vec<char> = num_str.chars().rev().collect();
    let mut formatted = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            formatted.push(',');
        }
        formatted.push(*c);
    }
    formatted.chars().rev().collect()
}

fn get_inode_usage(path: &Path) -> std::io::Result<u64> {
    let output = Command::new("df")
        .arg("-i")
        .arg(path)
        .output()?;
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.split('\n').collect();
    let inode_usage: Vec<&str> = lines[1].split_whitespace().collect();

    inode_usage[2].parse().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

fn confirm_prompt(count: usize, max_size: usize, use_eicar: bool) -> bool {
    let eicar_msg = if use_eicar { " (with EICAR signature)" } else { "" };
    println!("{}", format!("You are about to create {} files with a maximum size of {} each{}.", format_number(count), human_readable_size(max_size), eicar_msg).red().bold());
    println!("{}", "Do you want to proceed? (yes/no): ".red().bold());

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().eq_ignore_ascii_case("yes")
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 || args.len() > 5 {
        eprintln!("Usage: {} <base_path> <count> <max_size> [--eicar]", args[0]);
        std::process::exit(1);
    }

    let base_path = Path::new(&args[1]);
    let count: usize = args[2].parse().expect("Count should be a number");
    let max_size: usize = args[3].parse().expect("Max size should be a number");
    let use_eicar = args.get(4).map_or(false, |arg| arg == "--eicar");

    println!("{}", "------------------------------------------".blue());
    println!("{}", "fauxFS".bold().green());
    println!("{}", "N Collins - ncollins@fortinet.com".bold().green());
    println!("{}", "------------------------------------------".blue());

    if !confirm_prompt(count, max_size, use_eicar) {
        println!("{}", "Operation cancelled by the user.".red().bold());
        std::process::exit(1);
    }

    // Create a progress bar
    let pb = ProgressBar::new(count as u64);
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg} {wide_bar:.cyan/blue} {pos}/{len} ({eta})"));

    pb.set_message("Generating files and directories");

    match create_random_files_and_dirs(base_path, count, max_size, use_eicar, &pb) {
        Ok((total_size, inodes)) => {
            let inode_usage = get_inode_usage(base_path)?;
            println!("{}", format!("Filesystem inode usage: {}", format_number(inode_usage as usize)).cyan());
 //           println!("{}", format!("Total size of created files: {}", human_readable_size(total_size)).cyan());
 //           println!("{}", format!("Total inodes used by created files: {}", format_number(inodes as usize)).cyan());
        },
        Err(e) => eprintln!("{}", format!("Error creating files: {}", e).red()),
    }

    Ok(())
}
