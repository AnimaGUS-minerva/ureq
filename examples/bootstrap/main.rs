use std::io::{self, Read};
//use std::io::Read;
use std::sync::{Arc};
use std::time::Duration;
use std::net::TcpStream;
use std::{env, error, fmt, result};
use std::process;

use ureq::MbedTlsConnector;
use ureq::brski_connect;

use log::{error, info};
use ureq;

#[derive(Debug)]
struct Oops(String);

impl From<io::Error> for Oops {
    fn from(e: io::Error) -> Oops {
        Oops(e.to_string())
    }
}

impl From<ureq::Error> for Oops {
    fn from(e: ureq::Error) -> Oops {
        Oops(e.to_string())
    }
}

impl error::Error for Oops {}

impl fmt::Display for Oops {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

type Result<T> = result::Result<T, Oops>;

// fn get(agent: &ureq::Agent, url: &str) -> Result<Vec<u8>> {
//     let response = agent.get(url).call()?;
//     let mut reader = response.into_reader();
//     let mut bytes = vec![];
//     reader.read_to_end(&mut bytes)?;
//     Ok(bytes)
// }

// fn get_and_write(agent: &ureq::Agent, url: &str) {
//     info!("🕷️ {}", url);
//     match get(agent, url) {
//         Ok(_) => info!("Good: ✔️ {}\n", url),
//         Err(e) => error!("Bad: ⚠️ {} {}\n", url, e),
//     }
// }

fn main() -> Result<()> {
    let _args = env::args();
    env_logger::init();

    info!("bootstrap PID: {}", process::id());

    let connector = Arc::new(MbedTlsConnector::new(mbedtls::ssl::config::AuthMode::None));
    let agent = ureq::builder()
        .tls_connector(connector.clone())
        .timeout_connect(Duration::from_secs(5))
        .timeout(Duration::from_secs(20))
        .build();

    /* establish the connection */
    let conn = TcpStream::connect("[::2]:8443").unwrap();

    /* do the TLS bits */
    let _stream = brski_connect(connector, agent, conn).unwrap();


    Ok(())
}

/*
 * Local Variables:
 * compile-command: "cargo build --example bootstrap --features=\"mbedtls\""
 * mode: rust
 * End:
 */
