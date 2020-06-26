extern crate signal;
mod engines;
mod error;
mod utils;

use std::env;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use signal::trap::Trap;
use signal::Signal::SIGINT;

use crate::error::AutoTeXErr;

// continuous compile option
const CONTINUS_COMPILE: &str = "-v";

fn main() -> error::Result<()> {
    let mut args = env::args().collect::<Vec<String>>()[1..].to_vec();
    if args.is_empty() {
        return Err(AutoTeXErr::TakeFilesErr);
    }
    let filename = Path::new(&args.pop().unwrap()).to_path_buf();
    compile_engine(&filename, &args)
}

fn compile_engine(filepath: &PathBuf, options: &Vec<String>) -> error::Result<()> {
    let tex_info = utils::get_files_info(&filepath)?;
    let engine = engines::take_engine(
        &options
            .iter()
            .filter(|x| engines::ENGINE_OPTIONS.contains(&x.as_str()))
            .collect(),
    )?;
    // Check whether "-v" option is used.
    // If so, then run tex continuously.
    if options.contains(&String::from(CONTINUS_COMPILE)) {
        let mut init_time = utils::take_time(&tex_info)?;
        let curr_dir = env::current_dir()?;
        let trap = Trap::trap(&[SIGINT]);
        engines::run_engine(&engine, &tex_info)?;
        utils::run_pdf(&tex_info)?;
        env::set_current_dir(&curr_dir)?;
        println!("Press Ctrl+C to finish the program.");
        while let None = trap.wait(Instant::now()) {
            let compare_time = utils::take_time(&tex_info)?;
            if init_time != compare_time {
                engines::run_engine(&engine, &tex_info)?;
                env::set_current_dir(&curr_dir)?;
                init_time = utils::take_time(&tex_info)?;
                println!("Press Ctrl+C to finish the program.");
            }
            thread::sleep(Duration::from_secs(1));
        }
        println!("\nQuitting");
    } else {
        engines::run_engine(&engine, &tex_info)?;
    }
    Ok(())
}
