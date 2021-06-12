use std::{
    cell::RefCell,
    convert::TryInto,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    web::{Buf, BytesMut},
    Error, HttpMessage, HttpRequest,
};
use futures::stream::StreamExt;

use futures::future::{ok, ready, Ready};
use futures::Future;

use ed25519_dalek::{PublicKey, Signature, Verifier};

//use crate::hex;

//use hex;

pub struct VerifyEd25519Signature;

impl<S: 'static, Req> Transform<S, ServiceRequest> for VerifyEd25519Signature
where
    S: Service<ServiceRequest, Response = ServiceResponse<Req>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<Req>;
    type Error = Error;
    type InitError = ();
    type Transform = VerifyEd25519SignatureMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(VerifyEd25519SignatureMiddleware {
            service: Rc::new(RefCell::new(service)),
        }))
        // ok(VerifyEd25519SignatureMiddleware {
        //     service: Rc::new(RefCell::new(service)),
        // })
    }
}

pub struct VerifyEd25519SignatureMiddleware<S> {
    // In a refcell as seen
    // https://github.com/actix/examples/blob/ddfb4706425885bfffec0a13b216ff08f93a47d2/basics/middleware/src/read_request_body.rs#L36
    service: Rc<RefCell<S>>,
}
impl<S: 'static, Req> Service<ServiceRequest> for VerifyEd25519SignatureMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<Req>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<Req>;
    type Error = Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let mut svc = self.service.clone();

        Box::pin(async move {
            // Grab the body as some bytes
            let mut body = BytesMut::new();
            let mut stream = req.take_payload();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }

            // Grab the signature and timestamp from the headers and transform them to Option<&str>
            let signature = req
                .headers()
                .get("X-Signature-Ed25519")
                .and_then(|signature| signature.to_str().ok());
            let signature = match signature {
                None => {
                    return Err(ErrorUnauthorized("missing X-Signature-Ed25519 header"));
                }
                Some(s) => s,
            };
            let timestamp = req
                .headers()
                .get("X-Signature-Timestamp")
                .and_then(|timestamp| timestamp.to_str().ok());
            let timestamp = match timestamp {
                None => {
                    return Err(ErrorUnauthorized("missing X-Signature-Timestamp header"));
                }
                Some(s) => s,
            };

            // Create the message to validate by prepending the body with the signature timestamp
            //let mut message = Vec::from(timestamp);
            //message.extend_from_slice(body.bytes());
            let message = body;

            if let Err(e) = verify(&message[..], timestamp, signature) {
                return Err(ErrorUnauthorized("invalid signature"));
            }

            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}

fn verify(message: &[u8], timestamp: &str, signature: &str) -> Result<(), ValidationError> {
    let pubkey = hex::decode("0363649faf7a83d0bc0d9faa9c6a5efa8adc772190b8072210bc825895ca3570")
        .ok()
        .unwrap();

    // Concatenate timestamp+body then verify this against the provided signature.
    let mut timestamped_message = Vec::from(timestamp);
    timestamped_message.extend_from_slice(message);

    // TODO: Get this from dotenv and never unwrap it without error handling
    let public_key: PublicKey = PublicKey::from_bytes(pubkey.as_slice()).unwrap();

    let signature_bytes = match hex::decode(&signature).ok() {
        Some(val) => val,
        None => {
            return Err(ValidationError::KeyConversionError {
                name: "unable to decode hex",
            })
        }
    };

    let signature_bytes: [u8; 64] =
        signature_bytes
            .try_into()
            .map_err(|_| ValidationError::KeyConversionError {
                name: "signature length",
            })?;

    let signature = Signature::new(signature_bytes);

    match public_key.verify(timestamped_message.as_slice(), &signature) {
        Ok(val) => val,
        Err(_) => return Err(ValidationError::InvalidSignatureError),
    };

    Ok(())
}
#[derive(Debug)]
pub enum ValidationError {
    /// For anything related to conversion errors
    KeyConversionError {
        /// What error?
        name: &'static str,
    },
    /// For invalid keys
    InvalidSignatureError,
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn validate_returns_true_for_valid_signature() {
//         todo!()
//     }

//     #[test]
//     fn validate_returns_false_for_invalid_signature() {
//         todo!()
//     }
// }
