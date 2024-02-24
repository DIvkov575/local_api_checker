use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use log::info;

pub fn counter(port: usize, name: &str) -> Result<(), Box<dyn Error>> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addr)?;

    println!("Listener Started");
    println!("Broken!!");

    for _ in listener.incoming() { increment(name)?; }

    Ok(())
}

fn increment(name: &str) -> Result<(), Box<dyn Error>> {
    if Path::new(name).exists() {
        let mut file = File::options()
            .read(true)
            .write(true)
            .open(&name)?;
        // file.read(&mut buffer)?;

        // let value = String::from_utf8_lossy(&buffer).parse::<usize>()? + 1;
        // // println!("{:?}", value);
        // file.write_all(&value.to_string().into_bytes())?;


    } else {
        let mut file = File::create(name)?;
        let buf = "0".to_string().into_bytes();
        file.write_all(&buf)?;
    }


    Ok(())
}