use std::ops::Deref;

use actix_web::{
    dev::Payload,
    http::header,
    FromRequest, HttpMessage, HttpRequest,
};
use futures::future::ok;

/// Holds a value that determines whether or not this request wanted a plaintext response.
///
/// We assume anything with the text/plain Accept or Content-Type headers want plaintext,
/// and also anything calling us from the console or that we can't identify.
pub struct IsPlaintextRequest(pub bool);

impl Deref for IsPlaintextRequest {
    type Target = bool;

    fn deref(&self) -> &bool {
        &self.0
    }
}

impl FromRequest for IsPlaintextRequest {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if req.content_type() == "text/plain" {
            return ok(IsPlaintextRequest(true));
        }

        match req
            .headers()
            .get(header::USER_AGENT)
            .and_then(|u| u.to_str().unwrap().split('/').next())
        {
            None | Some("Wget" | "curl" | "HTTPie") => ok(IsPlaintextRequest(true)),
            _ => ok(IsPlaintextRequest(false)),
        }
    }
}

