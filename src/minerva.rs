// This library derives from unit.rs,
// it provides the needed mechanism to do RFC8995 (BRSKI) over https

//use std::fmt::{self, Display};
//use std::io::{self, Write};

use log::debug;

use crate::unit::Unit;
use crate::stream::Stream;
use crate::response::Response;
//use crate::body::{self, BodySize, Payload, SizedReader};
use crate::body::{Payload, SizedReader};
use crate::error::{Error, ErrorKind};
//use crate::agent::RedirectAuthHeaders;
use crate::connect::{connect_inner,can_propagate_authorization_on_redirect};

/// Perform a connection. Follows redirects.
pub(crate) fn brski_connect(
    mut unit: Unit,
    _provided_stream: Stream,
    use_pooled: bool,
    mut body: SizedReader,
) -> Result<Response, Error> {
    let mut history = vec![];

    let mut resp = loop {
        let resp = connect_inner(&unit, use_pooled, body, &history)?;

        // handle redirects
        if !(300..399).contains(&resp.status()) || unit.agent.config.redirects == 0 {
            break resp;
        }
        if history.len() + 1 >= unit.agent.config.redirects as usize {
            return Err(ErrorKind::TooManyRedirects.msg(format!(
                "reached max redirects ({})",
                unit.agent.config.redirects
            )));
        }
        // the location header
        let location = match resp.header("location") {
            Some(l) => l,
            None => break resp,
        };

        let url = &unit.url;
        let method = &unit.method;
        // join location header to current url in case it is relative
        let new_url = url.join(location).map_err(|e| {
            ErrorKind::InvalidUrl
                .msg(format!("Bad redirection: {}", location))
                .src(e)
        })?;

        // perform the redirect differently depending on 3xx code.
        let new_method = match resp.status() {
            // this is to follow how curl does it. POST, PUT etc change
            // to GET on a redirect.
            301 | 302 | 303 => match &method[..] {
                "GET" | "HEAD" => unit.method,
                _ => "GET".into(),
            },
            // never change the method for 307/308
            // only resend the request if it cannot have a body
            // NOTE: DELETE is intentionally excluded: https://stackoverflow.com/questions/299628
            307 | 308 if ["GET", "HEAD", "OPTIONS", "TRACE"].contains(&method.as_str()) => {
                unit.method
            }
            _ => break resp,
        };

        let keep_auth_header = can_propagate_authorization_on_redirect(
            &unit.agent.config.redirect_auth_headers,
            url,
            &new_url,
        );

        debug!("redirect {} {} -> {}", resp.status(), url, new_url);
        history.push(unit.url);
        body = Payload::Empty.into_read();

        // reuse the previous header vec on redirects.
        let mut headers = unit.headers;

        // on redirects we don't want to keep "content-length". we also might want to
        // strip away "authorization" to ensure credentials are not leaked.
        headers.retain(|h| {
            !h.is_name("content-length") && (!h.is_name("authorization") || keep_auth_header)
        });

        // recreate the unit to get a new hostname and cookies for the new host.
        unit = Unit::new(
            &unit.agent,
            &new_method,
            &new_url,
            headers,
            &body,
            unit.deadline,
        );
    };
    resp.history = history;
    Ok(resp)
}

