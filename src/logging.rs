use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::SystemTime;
use log::info;

pub fn logging(port: usize, name: &str) -> Result<(), Box<dyn Error>> {
    /// logging command
    /// port - localhost port
    /// name - file path to store logs

    setup_logger()?;
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addr)?;

    println!("Listener Started");
    info!("Logger Started");

    listener.incoming().for_each(|x| handle_stream(x.unwrap()));

    Ok(())
}


fn handle_stream(mut stream: TcpStream) {
    /// Function which executes for each new stream connection
    /// Logs to stdout + file and responds to tcp request with http OK

    info!("connection");
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}

fn setup_logger() -> Result<(), fern::InitError> {
    /// create logger connection
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                message
            ))
        })
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
