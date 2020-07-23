#![warn(rust_2018_idioms)]
mod engines;
mod error;
mod help;
mod utils;

use std::env;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use signal::trap::Trap;
use signal::Signal::SIGINT;

use crate::error::AutoTeXErr;

// continuous compile option
const CONTINUS_COMPILE_OPTION: &str = "-v";
const HELP_OPTION: &str = "--help";

fn main() -> error::Result<()> {
    let mut args = env::args().collect::<Vec<String>>()[1..].to_vec();
    if args.is_empty() {
        return Err(AutoTeXErr::TakeFilesErr);
    }
    if args.contains(&HELP_OPTION.to_string()) {
        println!("{}", help::HELP_STRING);
        Ok(())
    } else {
        let filename = Path::new(&args.pop().unwrap()).to_path_buf();
        compile(&filename, &args)
    }
}

fn compile(filepath: &PathBuf, options: &[String]) -> error::Result<()> {
    let tex_info = utils::get_files_info(&filepath)?;
    let engines = &options
        .iter()
        .filter(|x| engines::ENGINE_OPTIONS.contains(&x.as_str()))
        .collect::<Vec<_>>();
    let engine = engines::take_engine(&engines)?;
    // Check whether "-v" option is used.
    // If so, then run tex continuously.
    if options.contains(&CONTINUS_COMPILE_OPTION.to_string()) {
        let mut init_time = tex_info.take_time()?;
        let curr_dir = env::current_dir()?;
        let trap = Trap::trap(&[SIGINT]);
        engine.run_engine(&tex_info)?;
        tex_info.run_pdf()?;
        thread::sleep(Duration::from_secs(1));
        env::set_current_dir(&curr_dir)?;
        println!("Press Ctrl+C to finish the program.");
        while trap.wait(Instant::now()).is_none() {
            let compare_time = tex_info.take_time()?;
            if init_time != compare_time {
                engine.run_engine(&tex_info)?;
                env::set_current_dir(&curr_dir)?;
                init_time = tex_info.take_time()?;
                println!("Press Ctrl+C to finish the program.");
            }
            thread::sleep(Duration::from_secs(1));
        }
        println!("\nQuitting");
    } else {
        engine.run_engine(&tex_info)?;
    }
    Ok(())
}
