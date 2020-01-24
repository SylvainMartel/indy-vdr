extern crate clap;
use clap::{App, Arg};

pub struct Config {
    pub genesis: String,
    pub socket: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
}

pub fn load_config() -> Result<Config, String> {
    let matches = App::new("indy-vdr-proxy")
        .version("0.1.0")
        // .author("Andrew Whitehead")
        .about("Proxy requests to a Hyperledger Indy ledger")
        .arg(
            Arg::with_name("genesis")
                .short("g")
                .value_name("GENESIS")
                .help("Path to the ledger genesis transactions")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("host")
                .short("h")
                .value_name("HOST")
                .default_value("0.0.0.0")
                .help("Set the local address to listen on")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .value_name("PORT")
                .help("Sets the local port to listen on")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("socket")
                .short("s")
                .value_name("SOCKET")
                .help("Sets the UNIX socket path listen on")
                .takes_value(true),
        )
        .get_matches();

    let genesis = matches
        .value_of("genesis")
        .unwrap_or("genesis.txn")
        .to_owned();

    if matches.occurrences_of("socket") > 0 {
        if matches.occurrences_of("host") > 0 {
            return Err("Cannot specify both host and socket".to_owned());
        }
    } else {
        if matches.occurrences_of("port") == 0 {
            return Err("Port number or socket must be specified".to_owned());
        }
    }

    let socket = matches.value_of("socket").map(str::to_owned);
    let host = matches.value_of("host").map(str::to_owned);
    let port = if let Some(port) = matches.value_of("port") {
        Some(port.parse::<u16>().map_err(|_| "Invalid port number")?)
    } else {
        None
    };

    Ok(Config {
        genesis,
        socket,
        host,
        port,
    })
}