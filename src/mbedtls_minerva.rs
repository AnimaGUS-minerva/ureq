// Reimplement mbedtls.rs based on the minerva_mbedtls crate

use std::fmt;
use std::io;
use crate::{Error, ReadWrite, TlsConnector};

use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub struct MbedTlsConnector {
    //pub context: Arc<Mutex<Context>>,
}

pub fn wrap_stream_with_connector(
    mtc:       &MbedTlsConnector,
    sock:      TcpStream,
) -> Result<Box<MbedTlsStream>, Error> {
    Ok(Box::new(MbedTlsStream {})) // dummy
}

pub struct MbedTlsStream {
    //pub context: Arc<Mutex<Context>>, //tcp_stream: TcpStream,
}
