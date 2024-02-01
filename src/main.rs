use dialoguer::{Confirm, theme::ColorfulTheme};
use std::fs;

fn main() {
    let dirs = [
        "C:/Windows/Temp",
        "C:/Windows.old",
        "C:/Windows/LiveKernelReports",
        "C:/Program Files/rempl",
        "C:/Windows/Downloaded Program Files",
    ];

    let mut _final_size: u64 = 0;

    for dir in dirs {
        match fs::read_dir(dir) {
            Ok(entries) => {
                let total_size: u64 = entries
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| entry.file_type().unwrap().is_file())
                    .map(|entry| entry.metadata().unwrap().len())
                    .sum();
                _final_size += total_size
            }
            Err(_e) => {},
        }
    }

    _final_size = _final_size / (1024 * 1024);

    println!("We found {} MB in unecessary files, wanna remove?", _final_size);

    let _confirmation = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to delete them all?")
        .default(true)
        .show_default(false)
        .wait_for_newline(true)
        .interact()
        .unwrap();

    if _confirmation == true {
        for _dir in dirs {
            let _remove_dir = fs::remove_dir_all(_dir);
        }
        println!("Everything has been removed, now you got {} MB.", _final_size)
    } else {
        println!("I'm sorry, exiting.")
    }
}