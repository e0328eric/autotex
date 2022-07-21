#![warn(rust_2018_idioms)]
mod commands;
mod compilable;
mod engines;
mod error;
mod utils;

use crate::commands::AutoTeXCommand;
use std::env;
use std::io::ErrorKind;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use signal_hook::consts::signal::SIGINT;
use signal_hook::flag as signal_flag;

fn main() -> error::Result<()> {
    let args = AutoTeXCommand::new()?;
    run_autotex(args)
}

fn run_autotex(args: AutoTeXCommand) -> error::Result<()> {
    let mut tex_info = utils::get_files_info(&args.file_path)?;
    let engine = engines::take_engine(&args.tex_engine)?;
    if args.is_conti_compile {
        // First, collect the modification time for each files
        // in the current directory and its children.
        let mut init_time = tex_info.take_time()?;

        // Then change the directory to compile.
        let curr_dir = env::current_dir()?;
        let trap = Arc::new(AtomicUsize::new(0));
        signal_flag::register_usize(SIGINT, Arc::clone(&trap), SIGINT as usize)?;

        // If it has an error while compile first, then exit whole program.
        let has_error_first = engine.run_engine(&tex_info)?;
        if !has_error_first {
            return Ok(());
        }
        if args.is_view {
            tex_info.run_pdf()?;
        }

        // If not, then show a pdf file if the view option is used
        thread::sleep(Duration::from_secs(1));
        env::set_current_dir(&curr_dir)?;
        println!("Press Ctrl+C to finish the program.");
        while trap.load(Ordering::Relaxed) != SIGINT as usize {
            let compare_time = tex_info.take_time()?;
            if init_time != compare_time {
                tex_info = utils::get_files_info(&args.file_path)?;
                std::fs::remove_file(tex_info.get_main_pdf_file())?;
                engine.run_engine(&tex_info)?;
                env::set_current_dir(&curr_dir)?;
                init_time = tex_info.take_time()?;
                println!("Press Ctrl+C to finish the program.");
            }
            thread::sleep(Duration::from_secs(1));
        }
        println!("\nQuitting");
    } else if !args.is_view {
        match std::fs::remove_file(tex_info.get_main_pdf_file()).map_err(|err| err.kind()) {
            Ok(()) | Err(ErrorKind::NotFound) => {}
            Err(err) => panic!("{}", err),
        }
        engine.run_engine(&tex_info)?;
    } else {
        env::set_current_dir(&tex_info.current_dir)?;
        tex_info.run_pdf()?
    }
    Ok(())
}
