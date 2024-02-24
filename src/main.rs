use log::{debug, info, LevelFilter, warn};
use std::error::Error;
use std::io::{Read, read_to_string, Write};
use std::path::Path;
use std::time::SystemTime;
use clap::Parser;
use homedir::get_my_home;
use thiserror::Error;
use crate::MyError::{broken_config_dir, cfg_dir_dne, file_instead_of_config_dir};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t=8080)]
    port: usize,
    #[arg(short, long, default_value="tmp")]
    name: String,
    #[arg(short, long)]
    home_path: Option<String>,
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>>{
    setup_logger()?;


    let args = Args::parse();
    let home_dir;
    if let Some(home_path) = args.home_path { home_dir = home_path }
    else { home_dir = get_my_home().unwrap().unwrap(); }

    let config_dir = Path::new(&home_dir).join("local_counter");
    if config_dir.exists() {
        if config_dir.is_file() { return Err(file_instead_of_config_dir.into()) }
        else {
            if !config_dir.join("logs").exists() { return Err(broken_config_dir.into()) }
            else {

            } }
    } else { return Err(cfg_dir_dne.into()) }

    // let value: usize;
    // let mut file: File;
    // let file_path = Path::new(&home_dir).join(args.name);
    // if file_path.exists() {
    //     file = File::open(file_path)?;
    //     file.write_all(0usize)
    // } else {
    //     file =
    // }
    //
    // let mut buffer: [u8];
    // file.read(&mut buffer)?;
    // value = buffer.into() + 1usize;

    // let mut file = OpenOptions::new().create(true).write(true).read(true).open(home_dir.join(args.name))?;
    // file.write_all(value.into())?;


    Ok(())
}


#[derive(Error, Debug)]
#[error("MyError")]
enum MyError {
    file_instead_of_config_dir,
    broken_config_dir,
    cfg_dir_dne,
}