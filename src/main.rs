mod logging;

use std::error::Error;
use std::io::{Read, read_to_string, Write};
use clap::Parser;
use crate::logging::logging;


fn main() -> Result<(), Box<dyn Error>>{
    Args::parse().command.run()?;
    Ok(())
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    pub command: Command
}

#[derive(Parser, Debug)]
pub enum Command {
    #[command(about="log")]
    log {
        #[arg(short, long, default_value_t=8080)]
        port: usize,
        #[arg(short, long, default_value="./local_counter.log")]
        name: String,
    }
}

impl Command {
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        use Command::*;
        match self {
            log { port, name } => logging(port, &name).unwrap(),
        }
        Ok(())
    }
}
