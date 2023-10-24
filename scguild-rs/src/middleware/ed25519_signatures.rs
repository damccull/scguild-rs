use std::{cell::RefCell, convert::TryInto, rc::Rc};

use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    web::Bytes,
    Error, FromRequest,
};
use futures::{future::LocalBoxFuture, FutureExt};

use futures::future::{ready, Ready};

use ed25519_dalek::{PublicKey, Signature, Verifier};

pub struct VerifyEd25519Signature {
    public_key: Rc<PublicKey>,
}
impl VerifyEd25519Signature {
    pub fn new(public_key: PublicKey) -> Self {
        VerifyEd25519Signature {
            public_key: Rc::new(public_key),
        }
    }
}
impl<S: 'static, Req> Transform<S, ServiceRequest> for VerifyEd25519Signature
where
    S: Service<ServiceRequest, Response = ServiceResponse<Req>, Error = Error> + 'static,
{
    type Response = ServiceResponse<Req>;
    type Error = Error;
    type InitError = ();
    type Transform = VerifyEd25519SignatureMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(VerifyEd25519SignatureMiddleware {
            public_key: self.public_key.clone(),
            service: Rc::new(RefCell::new(service)),
        }))
    }
}

pub struct VerifyEd25519SignatureMiddleware<S> {
    public_key: Rc<PublicKey>,
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
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Clone the RC pointers so they can be moved into the async block
        let public_key = self.public_key.clone();
        let svc = self.service.clone();

        async move {
            let (hreq, mut payload) = req.into_parts();

            let body = Bytes::from_request(&hreq, &mut payload).await;
            let body = match body {
                Ok(x) => x,
                Err(e) => return Err(ErrorUnauthorized(format!("unable to get payload: {}", e))),
            };

            // Grab the signature and timestamp from the headers and transform them to Option<&str>
            let signature = hreq
                .headers()
                .get("X-Signature-Ed25519")
                .and_then(|signature| signature.to_str().ok());
            let signature = match signature {
                None => {
                    return Err(ErrorUnauthorized("missing X-Signature-Ed25519 header"));
                }
                Some(s) => s,
            };
            let timestamp = hreq
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
            let message = &body;

            if verify(&message[..], timestamp, signature, public_key).is_err() {
                return Err(ErrorUnauthorized("invalid signature"));
            }

            let req = ServiceRequest::from_parts(hreq, payload_from_bytes(body));

            let res = svc.call(req).await?;
            Ok(res)
        }
        .boxed_local()
    }
}

fn payload_from_bytes(bytes: Bytes) -> dev::Payload {
    let (_, mut h1_payload) = actix_http::h1::Payload::create(true);
    h1_payload.unread_data(bytes);
    dev::Payload::from(h1_payload)
}

fn verify(
    message: &[u8],
    timestamp: &str,
    signature: &str,
    public_key: Rc<PublicKey>,
) -> Result<(), ValidationError> {
    let public_key = public_key;

    // Concatenate timestamp+body then verify this against the provided signature.
    let mut timestamped_message = Vec::from(timestamp);
    timestamped_message.extend_from_slice(message);

    let signature_bytes = match hex::decode(&signature).ok() {
        Some(val) => val,
        None => {
            return Err(ValidationError::KeyConversionError(
                "unable to decode hex".to_string(),
            ))
        }
    };

    let signature_bytes: [u8; 64] = signature_bytes
        .try_into()
        .map_err(|_| ValidationError::KeyConversionError("invalid signature length".to_string()))?;

    let signature = match Signature::from_bytes(&signature_bytes) {
        Ok(x) => x,
        Err(e) => {
            return Err(ValidationError::KeyConversionError(format!(
                "could not parse signature bytes: {}",
                e
            )))
        }
    };

    match public_key.verify(timestamped_message.as_slice(), &signature) {
        Ok(val) => val,
        Err(_) => return Err(ValidationError::InvalidSignatureError),
    };

    Ok(())
}
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// For anything related to conversion errors
    #[error("Key conversion error: {0}")]
    KeyConversionError(String),
    /// For invalid keys
    #[error("The key had an invalid signature")]
    InvalidSignatureError,
    /// Any unexpected errors
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
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
