use core::slice::SlicePattern;
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
    web::BytesMut,
    Error, HttpMessage,
};
use futures::stream::StreamExt;

use futures::future::{err, ok, Either, Ready};
use futures::Future;

use ed25519_dalek::{PublicKey, Signature, Verifier};

//use crate::hex;

use hex;

pub struct VerifyEd25519Signature;

impl<S: 'static, B> Transform<S> for VerifyEd25519Signature
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = VerifyEd25519SignatureMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(VerifyEd25519SignatureMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct VerifyEd25519SignatureMiddleware<S> {
    // In a refcell as seen
    // https://github.com/actix/examples/blob/ddfb4706425885bfffec0a13b216ff08f93a47d2/basics/middleware/src/read_request_body.rs#L36
    service: Rc<RefCell<S>>,
}
impl<S, B> Service for VerifyEd25519SignatureMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    #[allow(clippy::type_complexity)]
    //type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    //type Future = Pin<Box<Either<S::Future, Ready<Result<Self::Response, Self::Error>>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let mut svc = self.service.clone();

        Box::pin(async move {
            let mut body = BytesMut::new();
            let mut stream = req.take_payload();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }

            //let body = String::from_utf8(body.to_vec()).ok();

            println!("request body: {:?}", body);

            // Grab the signature and timestamp from the headers and transform them to Option<&str>
            let sig = req
                .headers()
                .get("X-Signature-Ed25519")
                .and_then(|signature| signature.to_str().ok());
            let timestamp = req
                .headers()
                .get("X-Signature-Timestamp")
                .and_then(|timestamp| timestamp.to_str().ok());
            let ts = match timestamp {
                None => {
                    let e = err(ErrorUnauthorized("not authorized"));
                    return e;
                }
                Some(s) => s,
            };
            let message = println!("Sig: {:?}, Timestamp: {:?}", sig, timestamp);

            // if let (Some(signature), Some(timestamp)) = (sig, timestamp) {
            //     println!("Got both");
            // }

            let res = svc.call(req).await?;

            println!("response: {:?}", res.headers());
            let e = ok(self.service.call(req));
            e
        })

        //------------

        // println!("Hi from start. You requested: {}", req.path());
        // // Note: Probably don't need this as middleware can be registered on a scope
        // // // Don't even check this if it's not in the API path
        // // if !req.path().starts_with("/api") {
        // //     return Either::Left(self.service.call(req));
        // // }

        // // Grab the signature and timestamp from the headers and transform them to Option<&str>
        // let sig = req
        //     .headers()
        //     .get("X-Signature-Ed25519")
        //     .and_then(|signature| signature.to_str().ok());
        // let timestamp = req
        //     .headers()
        //     .get("X-Signature-Timestamp")
        //     .and_then(|timestamp| timestamp.to_str().ok());

        // let message = req.

        // if let (Some(signature), Some(timestamp)) = (sig, timestamp) {
        //     verify("ok".to_string(), signature);
        //     return Either::Left(self.service.call(req));
        // }

        // Either::Right(err(ErrorUnauthorized("not authorized")))
        // let fut = self.service.call(req);

        // Box::pin(async move {
        //     let res = fut.await?;

        //     println!("Hi from response");
        //     Ok(res)
        // })
    }
}

fn verify(message: String, signature: &str) -> Result<(), ValidationError> {
    let pubkey = hex::decode("97c0eac82876c508e26959019fba50b5b80a7305ab148f9a4ef5a787005f5fab")
        .ok()
        .unwrap();

    // Concatenate timestamp+body then verify this against the provided signature.

    let public_key: PublicKey = PublicKey::from_bytes(pubkey.as_slice()).unwrap();

    let signature_bytes = hex::decode(&signature).ok().unwrap();

    let signature_bytes: [u8; 64] =
        signature_bytes
            .try_into()
            .map_err(|_| ValidationError::KeyConversionError {
                name: "Signature Length",
            })?;

    let signature = Signature::new(signature_bytes);

    let result = public_key.verify(message.as_bytes(), &signature);

    println!("{:#?}", public_key);
    Ok(())
}
pub enum ValidationError {
    /// For anything related to conversion errors
    KeyConversionError {
        /// What error?
        name: &'static str,
    },
    /// For invalid keys
    InvalidSignatureError,
}

#[cfg(test)]
mod tests {
    #[test]
    fn validate_returns_true_for_valid_signature() {}

    #[test]
    fn validate_returns_false_for_invalid_signature() {}
}
