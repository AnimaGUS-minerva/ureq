//use std::fmt::{self, Display};
//use std::io::{self, Write};
//use std::ops::Range;
use std::time;

//use log::debug;
use url::Url;

#[cfg(feature = "cookies")]
use cookie::Cookie;

//use crate::agent::RedirectAuthHeaders;
use crate::body::{BodySize, SizedReader};
//use crate::error::{Error, ErrorKind};
use crate::header::{get_header, Header};
use crate::resolve::ArcResolver;
//use crate::response::Response;
//use crate::stream::{self, connect_test, Stream};
use crate::Agent;

#[cfg(test)]
use crate::header;

/// A Unit is fully-built Request, ready to execute.
///
/// *Internal API*
#[derive(Clone)]
pub(crate) struct Unit {
    pub agent: Agent,
    pub method: String,
    pub url: Url,
    pub is_chunked: bool,
    pub headers: Vec<Header>,
    pub deadline: Option<time::Instant>,
}

impl Unit {
    //

    pub(crate) fn new(
        agent: &Agent,
        method: &str,
        url: &Url,
        mut headers: Vec<Header>,
        body: &SizedReader,
        deadline: Option<time::Instant>,
    ) -> Self {
        //

        let (is_transfer_encoding_set, mut is_chunked) = get_header(&headers, "transfer-encoding")
            // if the user has set an encoding header, obey that.
            .map(|enc| {
                let is_transfer_encoding_set = !enc.is_empty();
                let last_encoding = enc.split(',').last();
                let is_chunked = last_encoding
                    .map(|last_enc| last_enc.trim() == "chunked")
                    .unwrap_or(false);
                (is_transfer_encoding_set, is_chunked)
            })
            // otherwise, no chunking.
            .unwrap_or((false, false));

        let mut extra_headers = {
            let mut extra = vec![];

            // chunking and Content-Length headers are mutually exclusive
            // also don't write this if the user has set it themselves
            if !is_chunked && get_header(&headers, "content-length").is_none() {
                // if the payload is of known size (everything beside an unsized reader), set
                // Content-Length,
                // otherwise, use the chunked Transfer-Encoding (only if no other Transfer-Encoding
                // has been set
                match body.size {
                    BodySize::Known(size) => {
                        extra.push(Header::new("Content-Length", &format!("{}", size)))
                    }
                    BodySize::Unknown => {
                        if !is_transfer_encoding_set {
                            extra.push(Header::new("Transfer-Encoding", "chunked"));
                            is_chunked = true;
                        }
                    }
                    BodySize::Empty => {}
                }
            }

            let username = url.username();
            let password = url.password().unwrap_or("");
            if (!username.is_empty() || !password.is_empty())
                && get_header(&headers, "authorization").is_none()
            {
                let encoded = base64::encode(&format!("{}:{}", username, password));
                extra.push(Header::new("Authorization", &format!("Basic {}", encoded)));
            }

            #[cfg(feature = "cookies")]
            extra.extend(extract_cookies(agent, url).into_iter());

            extra
        };

        headers.append(&mut extra_headers);

        Unit {
            agent: agent.clone(),
            method: method.to_string(),
            url: url.clone(),
            is_chunked,
            headers,
            deadline,
        }
    }

    pub fn is_head(&self) -> bool {
        self.method.eq_ignore_ascii_case("head")
    }

    pub fn resolver(&self) -> ArcResolver {
        self.agent.state.resolver.clone()
    }

    #[cfg(test)]
    pub fn header(&self, name: &str) -> Option<&str> {
        header::get_header(&self.headers, name)
    }
    #[cfg(test)]
    pub fn has(&self, name: &str) -> bool {
        header::has_header(&self.headers, name)
    }
    #[cfg(test)]
    pub fn all(&self, name: &str) -> Vec<&str> {
        header::get_all_headers(&self.headers, name)
    }

    // Returns true if this request, with the provided body, is retryable.
    pub(crate) fn is_retryable(&self, body: &SizedReader) -> bool {
        // Per https://tools.ietf.org/html/rfc7231#section-8.1.3
        // these methods are idempotent.
        let idempotent = match self.method.as_str() {
            "DELETE" | "GET" | "HEAD" | "OPTIONS" | "PUT" | "TRACE" => true,
            _ => false,
        };
        // Unsized bodies aren't retryable because we can't rewind the reader.
        // Sized bodies are retryable only if they are zero-length because of
        // coincidences of the current implementation - the function responsible
        // for retries doesn't have a way to replay a Payload.
        let retryable_body = match body.size {
            BodySize::Unknown => false,
            BodySize::Known(0) => true,
            BodySize::Known(_) => false,
            BodySize::Empty => true,
        };

        idempotent && retryable_body
    }
}

