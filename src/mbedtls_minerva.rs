// Reimplement mbedtls.rs based on the minerva_mbedtls crate

use std::fmt;
use std::io;
use crate::{Error, ReadWrite, TlsConnector};

use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub struct MbedTlsConnector {
    pub context: Arc<Mutex<Context>>,
}

#[allow(dead_code)]
pub(crate) fn default_tls_config() -> std::sync::Arc<dyn TlsConnector> {
    Arc::new(MbedTlsConnector::new(
        mbedtls::ssl::config::AuthMode::Required,
    ))
}

impl MbedTlsConnector {
    pub fn new(mode: mbedtls::ssl::config::AuthMode) -> MbedTlsConnector {
        let entropy = Arc::new(entropy_new());
        let mut config = Config::new(Endpoint::Client, Transport::Stream, Preset::Default);
        let rng = Arc::new(CtrDrbg::new(entropy, None).unwrap());
        config.set_rng(rng);
        config.set_authmode(mode);
        let ctx = Context::new(Arc::new(config));
        MbedTlsConnector {
            context: Arc::new(Mutex::new(ctx)),
        }
    }
}

pub fn wrap_stream_with_connector(
    mtc:       &MbedTlsConnector,
    sock:      TcpStream,
) -> Result<Box<MbedTlsStream>, Error> {
    let mut ctx = mtc.context.lock().unwrap();
    match ctx.establish(sock, None) {
        Err(_) => {
            let io_err = io::Error::new(io::ErrorKind::InvalidData, MbedTlsError);
            return Err(io_err.into());
        }
        Ok(()) => Ok(MbedTlsStream::new(mtc))
    }
}

pub struct MbedTlsStream {
    //pub context: Arc<Mutex<Context>>, //tcp_stream: TcpStream,
}
