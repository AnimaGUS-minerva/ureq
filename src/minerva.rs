// This library derives from unit.rs,
// it provides the needed mechanism to do RFC8995 (BRSKI) over https

//use std::fmt::{self, Display};
//use std::io::{self, Write};

//use log::debug;

use std::net::TcpStream;
use std::time::Duration;
use std::thread::sleep;
use url::Url;
use crate::unit::Unit;
use crate::stream::Stream;
use crate::agent::Agent;
use crate::response::Response;
use crate::request::Request;
use crate::header::Header;
//use crate::body::{self, BodySize, Payload, SizedReader};
use crate::body::{Payload, SizedReader};
use crate::error::{Error, ErrorKind};
//use crate::agent::RedirectAuthHeaders;
use crate::connect::{connect_inner,can_propagate_authorization_on_redirect};


pub fn brski_connect(
    sock:   TcpStream,
    agent:  Agent
) -> Result<Request, Error> {

    let tls_conf = &agent.config.tls_config;
    let tls_stream = tls_conf.connect("", sock)?;

    let _https_stream = Stream::new(tls_stream);
    let body = Payload::Text("Hello", "utf-8".to_string());

    let _unit = Unit::new(&agent,
                         &"POST".to_string(),
                         &Url::parse("https://localhost/.well-known/brski/requestvoucher").unwrap(),
                         vec![Header::new("User-Agent", "Minerva Bootstrap")], /* headers */
                         &body.into_read(),
                         None);

    sleep(Duration::new(20,0));


    Err(ErrorKind::InvalidUrl
                .msg(format!("code incomplete")))
}

//pub fn brski_request(
//    _stream: Stream,
//) -> Result<Request, Error> {
//
//    Err(ErrorKind::InvalidUrl.msg("request incomplete"))
//}
