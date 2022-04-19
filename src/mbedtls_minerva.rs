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

#[test]
fn ffi_minerva_mbedtls() {
    use minerva_mbedtls::psa_crypto::{self, ffi::*};

    psa_crypto::init().unwrap();
    psa_crypto::initialized().unwrap();

    //
    // Test the `md` bindings
    //

    let msg: &[u8] = /* jada */ &[132, 106, 83, 105, 103, 110, 97, 116, 117, 114, 101, 49, 65, 160, 64, 88, 185, 161, 26, 0, 15, 70, 140, 166, 5, 105, 112, 114, 111, 120, 105, 109, 105, 116, 121, 6, 193, 26, 87, 247, 248, 30, 8, 193, 26, 89, 208, 48, 0, 14, 109, 74, 65, 68, 65, 49, 50, 51, 52, 53, 54, 55, 56, 57, 11, 105, 97, 98, 99, 100, 49, 50, 51, 52, 53, 13, 120, 124, 77, 70, 107, 119, 69, 119, 89, 72, 75, 111, 90, 73, 122, 106, 48, 67, 65, 81, 89, 73, 75, 111, 90, 73, 122, 106, 48, 68, 65, 81, 99, 68, 81, 103, 65, 69, 78, 87, 81, 79, 122, 99, 78, 77, 85, 106, 80, 48, 78, 114, 116, 102, 101, 66, 99, 48, 68, 74, 76, 87, 102, 101, 77, 71, 103, 67, 70, 100, 73, 118, 54, 70, 85, 122, 52, 68, 105, 102, 77, 49, 117, 106, 77, 66, 101, 99, 47, 103, 54, 87, 47, 80, 54, 98, 111, 84, 109, 121, 84, 71, 100, 70, 79, 104, 47, 56, 72, 119, 75, 85, 101, 114, 76, 53, 98, 112, 110, 101, 75, 56, 115, 103, 61, 61];
    let mut digest = vec![0; 32]; // enough for SHA256
    let ret = unsafe {
        md(
            md_info_from_type(MD_SHA256), // this points to a static memory address
            msg.as_ptr(), msg.len(), digest.as_mut_ptr())
    };
    assert_eq!(ret, 0);
    assert_eq!(digest, [45, 106, 33, 97, 249, 125, 54, 185, 225, 237, 251, 191, 101, 21, 189, 9, 181, 239, 153, 225, 101, 54, 111, 15, 208, 136, 97, 182, 140, 57, 230, 157]);

    //
    // Test the `ssl_config` bindings
    //

    // just create and free a config
    let mut config = ssl_config::default();
    unsafe {
        ssl_config_init(&mut config);
        ssl_config_defaults(&mut config, SSL_IS_CLIENT, SSL_TRANSPORT_STREAM, SSL_PRESET_DEFAULT);

        ssl_config_free(&mut config);
    }
}