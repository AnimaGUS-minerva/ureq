// This library derives from unit.rs,
// it provides the needed mechanism to do RFC8995 (BRSKI) over https

//use std::fmt::{self, Display};
//use std::io::{self, Write};

//use log::debug;

use std::net::TcpStream;
use std::time::Duration;
use std::thread::sleep;
use url::Url;
use std::sync::Arc;
//use std::sync::Mutex;
use crate::unit::Unit;
//use crate::stream::Stream;
use crate::agent::Agent;
//use crate::response::Response;
use crate::request::Request;
use crate::header::Header;
//use crate::body::{self, BodySize, Payload, SizedReader};
use crate::body::{Payload};
use crate::error::{Error, ErrorKind};
//use crate::agent::RedirectAuthHeaders;
//use crate::connect::{connect_inner,can_propagate_authorization_on_redirect};
use crate::mbedtls::MbedTlsConnector;
use crate::mbedtls::wrap_stream_with_connector;

use minerva_voucher::{Voucher, attr::*, SignatureAlgorithm, Sign};

static KEY_PEM_F2_00_02: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/data/00-D0-E5-F2-00-02/key.pem"));

use std::io::{self, Cursor, Write};

pub fn asn1_signature_from(sig: &[u8]) -> io::Result<Vec<u8>> {
    let sig_len = sig.len();
    let half = sig_len / 2;
    let h = half as u8;

    let mut asn1 = vec![0u8; sig_len + 8];
    let mut writer = Cursor::new(&mut asn1[..]);
    writer.write(&[48, 2 * h + 6, 2, h + 1, 0])?;
    writer.write(&sig[..half])?; // r
    writer.write(&[2, h + 1, 0])?;
    writer.write(&sig[half..])?; // s

    Ok(asn1)
}

pub fn is_asn1_signature(sig: &[u8]) -> bool {
    let sig_len = sig.len();
    let seq_len = sig_len - 2;

    let int1_pos = 2;
    let int1_len = sig.get(int1_pos + 1);
    if int1_len.is_none() { return false; }
    let int1_len = *int1_len.unwrap() as usize;

    let int2_pos = int1_pos + 1 + int1_len + 1;
    let int2_len = sig.get(int2_pos + 1);
    if int2_len.is_none() { return false; }
    let int2_len = *int2_len.unwrap() as usize;

    sig[0] == 48 &&
        sig[1] as usize == seq_len &&
        sig[int1_pos] == 2 &&
        sig[int2_pos] == 2 &&
        int1_len + int2_len + 4 == seq_len
}

pub fn brski_connect(
    connector: Arc<MbedTlsConnector>,
    agent:  Agent,
    sock:   TcpStream,
) -> Result<Request, Error> {

    //let tls_conf = &agent.config.tls_config;
    //let tls_stream = tls_conf.connect("", sock)?;

    // or can we use connect??
    let mbedtls_stream = wrap_stream_with_connector(&connector,
                                                    sock).unwrap();

    //let https_stream = Stream::new(tls_stream);
    let body = Payload::Text("Hello", "utf-8".to_string());

    let _unit = Unit::new(&agent,
                         &"POST".to_string(),
                         &Url::parse("https://localhost/.well-known/brski/requestvoucher").unwrap(),
                         vec![Header::new("User-Agent", "Minerva Bootstrap")], /* headers */
                         &body.into_read(),
                          None);

    let mbedtls_context    = mbedtls_stream.context.lock().unwrap();
    let certificate_list   = mbedtls_context.peer_cert().unwrap();
    //let mut num = 0;
    let mut cert1: Option<mbedtls::alloc::Box<mbedtls::x509::Certificate>> = None;

    if let Some(certificates) = certificate_list {
        // only use first certificate returned
        for cert in certificates {
            match cert1 {
                None => { cert1 = Some(cert.clone()) },
                _ => {}
            }
            //println!("[{}] cert: {:?}", num, cert.clone());
            //num = num + 1;
        }
    } else {
        return Err(ErrorKind::InvalidUrl
                   .msg(format!("no certificate found")));
    }

    // now we have the peer certificate copied into cert1.
    println!("cert1: {:?}", cert1);

    let mut vrq = Voucher::new_vrq();

    vrq.set(Attr::Assertion(Assertion::Proximity))
        .set(Attr::CreatedOn(1599086034))
        .set(Attr::SerialNumber(b"00-D0-E5-F2-00-02".to_vec()));

    // This is required when the `Sign` trait is backed by mbedtls v3.
    #[cfg(feature = "v3")]
    minerva_voucher::init_psa_crypto();

    vrq.sign(KEY_PEM_F2_00_02, SignatureAlgorithm::ES256);

    let _cbor = vrq.serialize().unwrap();

    Err(ErrorKind::InvalidUrl
                .msg(format!("code incomplete")))
}

//pub fn brski_request(
//    _stream: Stream,
//) -> Result<Request, Error> {
//
//    Err(ErrorKind::InvalidUrl.msg("request incomplete"))
//}
