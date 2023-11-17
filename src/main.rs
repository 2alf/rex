use rayon::prelude::*;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use walkdir::WalkDir;
use std::io::Write;
use open;

static SHOULD_STOP_ANIMATION: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static ANIMATION_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn main() -> io::Result<()> {
    println!(
        r"
    __________ ____ ___  ____________________    __    __  _______________  _____________.____    ________ _______________________________ 
    \______   \    |   \/   _____/\__    ___/   / /   / /  \_   _____/\   \/  /\______   \    |   \_____  \\______   \_   _____/\______   \
     |       _/    |   /\_____  \   |    |     / /   / /    |    __)_  \     /  |     ___/    |    /   |   \|       _/|    __)_  |       _/
     |    |   \    |  / /        \  |    |     \ \   \ \    |        \ /     \  |    |   |    |___/    |    \    |   \|        \ |    |   \
     |____|_  /______/ /_______  /  |____|      \_\   \_\  /_______  //___/\  \ |____|   |_______ \_______  /____|_  /_______  / |____|_  /
            \/                 \/                                  \/       \_/                  \/       \/       \/        \/         \/ 
    
            
    REX by 2alf                                                                                                                      0.1.2      
            "
    );

    loop {
        let selected_drive = select_drive()?;
        println!("💾 Selected drive: {}", selected_drive);

        let root_path = format!("{}:\\", selected_drive.trim_end_matches(':'));
        println!("📁 Root path: {:?}", &root_path);

        println!("Enter the file name you want to find (type 'exit' to quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let file_name = input.trim();

        if file_name.to_lowercase() == "exit" {
            break;
        }

        println!("Searching for '{}'", file_name);
        SHOULD_STOP_ANIMATION.store(false, std::sync::atomic::Ordering::Relaxed);
        let animation_thread = thread::spawn(move || {
            let animation = ".......##########<<<<<<<<>>>>>>>>>>/////////.......................";
            let mut counter = 0;
            while !SHOULD_STOP_ANIMATION.load(std::sync::atomic::Ordering::Relaxed) {
                print!("\rSearching 📄 : {}   ", &animation[counter..counter + 10]);
                io::stdout().flush().unwrap();
                counter = (counter + 1) % (animation.len() - 9);
                thread::sleep(Duration::from_millis(100));
            }
        });

        let search_result = search_file(file_name, &root_path.clone())?;
        SHOULD_STOP_ANIMATION.store(true, std::sync::atomic::Ordering::Relaxed);
        animation_thread.join().unwrap();

        match search_result {
            Some(path) => {
                println!("\nSearch completed successfully.");
                if ask_user_to_open_file_location(&path)? {
                    open_file_location(&path)?;
                }
            }
            None => println!("\nSearch completed. File not found."),
        }
        println!("Do you want to perform another search? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            break; 
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
//SEARCH

fn search_file(file_name: &str, start_path: &str) -> io::Result<Option<String>> {
    let start_time = Instant::now();
    let mut result_path = None;

    for entry in WalkDir::new(start_path).into_iter().filter_map(|e| e.ok()) {
        let entry_name = entry.file_name().to_string_lossy();

        if entry_name.to_lowercase().ends_with(&file_name.to_lowercase()) {
            result_path = Some(entry.path().to_string_lossy().to_string());
            break;
        }
    }

    let elapsed_time = start_time.elapsed();

    match result_path {
        Some(path) => {
            println!("File found at: {}", path);
            println!("Elapsed time: {:?}", elapsed_time);
            Ok(Some(path))
        }
        None => Ok(None),
    }
}

////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
//OPEN

fn open_file_location(file_path: &str) -> io::Result<()> {
    println!("Opening file location: {}", file_path);

    // Open the file location using the 'open' crate
    if let Err(err) = open::that(file_path) {
        println!("Error opening file: {}", err);
    } else {
        // Open the parent directory separately
        let dir = Path::new(file_path).parent().unwrap_or(Path::new("."));
        if let Err(err) = open::that(dir) {
            println!("Error opening file location: {}", err);
        }
    }

    Ok(())
}

fn ask_user_to_open_file_location(file_path: &str) -> io::Result<bool> {
    println!("Do you want to open the file location? (y/n)");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().eq_ignore_ascii_case("y"))
}

////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
//DRIVES


// windows

#[cfg(target_os = "windows")]
fn select_drive() -> io::Result<String> {
    let drives: Vec<String> = (b'A'..=b'Z')
        .map(|drive| format!("{}:", drive as char))
        .filter_map(|drive| {
            if fs::canonicalize(&drive).is_ok() {
                Some(drive)
            } else {
                None
            }
        })
        .collect();

    if drives.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No drives found"));
    }

    println!("Select a drive:");
    for (index, drive) in drives.iter().enumerate() {
        println!("{}. {}", index + 1, drive);
    }

    let drive_selector: usize = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if let Ok(index) = input.trim().parse::<usize>() {
            if index > 0 && index <= drives.len() {
                break index;
            } else {
                println!("Invalid drive number. Please enter a valid number.");
            }
        } else {
            println!("Invalid input. Please enter a valid number.");
        }
    };

    Ok(drives[drive_selector - 1].clone())
}

// wip

#[cfg(not(target_os = "windows"))]
fn select_drive() -> io::Result<String> {
    let drives: Vec<String> = fs::read_dir("/Volumes")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();

    if drives.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No drives found"));
    }

    println!("Select a drive:");
    for (index, drive) in drives.iter().enumerate() {
        println!("{}. {}", index + 1, drive);
    }

    let drive_selector: usize = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if let Ok(index) = input.trim().parse::<usize>() {
            if index > 0 && index <= drives.len() {
                break index;
            } else {
                println!("Invalid drive number. Please enter a valid number.");
            }
        } else {
            println!("Invalid input. Please enter a valid number.");
        }
    };

    Ok(drives[drive_selector - 1].clone())
}
