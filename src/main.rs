//! Entry point to the program.
//! Should correctly get CLI arguments, and successfully parse all logs.
//! Final aggregate result must be valid, in order to do further processing correctly.
//! 
//! Hardcode : let number_of_workers = 5;
//! The logs directory could potentially contain hundreds of files.
//! In order to make this efficient, we should start n threads, depending on the CPU, and when one of the threads has finished, spawn a new one.
//! This way, we will avoid creating hundreds of threads, and avoid bottlenecks.
//! This should definitely be implemented at some point.
//! 
//! Consider a rewrite, to move all to logic to the lib and lib.rs
mod log_parser_lib;
mod arguments_lib;
mod utils;

use std::{collections::HashMap, thread::JoinHandle};
use log_parser_lib::log_parser::LogParser;
use log_parser_lib::owner_usage_struct::OwnerUsage;
use arguments_lib::cli_args::CLIArgs;
use utils::utils::get_file_names;

fn main() {
    let cli_args = CLIArgs::build(&mut std::env::args().skip(1)).unwrap_or_else(|error| {
        eprint!("CLI Arguments parsing error: {}", error);

        std::process::exit(1);
    });

    let log_dir = cli_args.get_logs_dir();
    let log_files = get_file_names(&log_dir);

    if log_files.is_empty() {
        eprint!("No files found in the given dir!");

        std::process::exit(1);
    }

    let start = std::time::Instant::now();
    // Initialize variables for thread sharing and com.
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(log_files.len());
    let (tx, rx) = std::sync::mpsc::channel();
    let mut aggregate: HashMap<u32, OwnerUsage> = HashMap::new();
    let number_of_workers = 5;

    let mut log_files_iter = log_files.into_iter();

    for _ in 0 .. number_of_workers {
        match log_files_iter.next() {
            Some(log_file) => {
                let log_file_full_path = format!("{}/{}", log_dir, log_file);
                let tx_clone = tx.clone();

                let log_handle = std::thread::spawn(move || {
                    let log_parse_result = LogParser::new(&log_file_full_path);

                    if let Err(_) = tx_clone.send(log_parse_result.parse()) {
                        panic!("One of the threads could not send the result through the channel. It is not safe to continue.");
                    }
                });

                handles.push(log_handle);
            }, 

            None => {
                break;
            }
        }
    }
    /*
     * tx has not changed ownership, so rx blocks indefinitely.
     * So drop it manually.
     */
    drop(tx);
    /*
     * Results are comming in here. Each of them must succeed.
     * If that's not the case, just terminate the program.
     *
     * It's not safe to have some partial data of the usage.
     */
    for log_parser_result in rx {
        match log_parser_result {
            Ok(owner_usage_hash_map) => {
                for (owner_id, owner_usage) in owner_usage_hash_map {
                    let entry = aggregate.entry(owner_id).or_insert(OwnerUsage::default());

                    if entry
                        .add_video_plays(owner_usage.get_video_plays())
                        .is_none()
                    {
                        println!(
                            "[FATAL ERROR]: Possible overflow occured when adding video_plays!"
                        );

                        std::process::exit(1);
                    }

                    if entry
                        .add_ad_impressions(owner_usage.get_ad_impressions())
                        .is_none()
                    {
                        println!(
                            "[FATAL ERROR]: Possible overflow occured when adding ad_impressions!"
                        );
                    }
                }
            }

            Err(error) => {
                println!("FATAL ERROR OCCURED : {}", error.to_string());

                std::process::exit(1);
            }
        };
    }
    // Check if any od the spawned threads paniced.
    for handle in handles {
        if let Err(e) = handle.join() {
            if let Some(s) = e.downcast_ref::<&str>() {
                println!("Panic message: {}", s);
            } else if let Some(s) = e.downcast_ref::<String>() {
                println!("Panic message: {}", s);
            } else {
                println!("Thread panicked with unknown type.");
            }

            std::process::exit(1);
        }
    }

    println!("Final aggregate result : {:?}", aggregate);

    let duration = start.elapsed();

    println!("Done! Finished in {:.2?} seconds", duration);
}
