use std::error::Error;
use std::io::Read;
use std::net::TcpListener;
use std::time::SystemTime;
use log::info;

pub fn logging(port: usize, name: &str) -> Result<(), Box<dyn Error>> {
    setup_logger()?;
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addr)?;

    println!("Listener Started");
    info!("Logger Started");

    for stream in listener.incoming() {
        let mut buf: String = "Default".to_string();
        let content = stream?.read_to_string(&mut buf)?;
        info!("connection");
    }
    Ok(())
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
