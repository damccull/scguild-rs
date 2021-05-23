use std::{
    convert::TryInto,
    pin::Pin,
    task::{Context, Poll},
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error,
};

use futures::future::{err, ok, Either, Ready};
use futures::Future;

use ed25519_dalek::{PublicKey, Signature, Verifier};

//use crate::hex;

use hex;

pub struct VerifyEd25519Signature;

impl<S, B> Transform<S> for VerifyEd25519Signature
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
        ok(VerifyEd25519SignatureMiddleware { service })
    }
}

pub struct VerifyEd25519SignatureMiddleware<S> {
    service: S,
}
impl<S, B> Service for VerifyEd25519SignatureMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    #[allow(clippy::type_complexity)]
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>; //Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());
        if !req.path().starts_with("/api") {
            return Either::Left(self.service.call(req));
        }
        if let Some(signature) = req
            .headers()
            .get("X-Signature-Ed25519")
            .and_then(|signature| signature.to_str().ok())
        {
            verify("ok".to_string(), signature);
            return Either::Left(self.service.call(req));
        }

        Either::Right(err(ErrorUnauthorized("not authorized")))
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
