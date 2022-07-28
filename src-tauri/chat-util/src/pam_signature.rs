use hmac::{Hmac, Mac};
use sha2::Sha256;

#[derive(Debug, Clone, Copy)]
pub struct Request<'a> {
    pub publish_key: &'a str,
    pub method: &'a str,
    pub path: &'a str,
    pub query: &'a str,
    pub body: &'a str,
}

pub fn sign(secret: &str, request: Request<'_>) -> String {
    let plain_message = format_plain_message(request);
    let encrypted_message = encrypt(secret, &plain_message);
    let base64_encrypted_message =
        base64::encode_config(encrypted_message, base64::URL_SAFE_NO_PAD);
    format!("v2.{}", base64_encrypted_message)
}

fn format_plain_message(request: Request<'_>) -> String {
    format!(
        "{method}\n{pub_key}\n{path}\n{query_string}\rn{body}",
        method = request.method,
        pub_key = request.publish_key,
        path = request.path,
        query_string = request.query,
        body = request.body
    )
}

type HmacSha256 = Hmac<Sha256>;

fn encrypt(secret: &str, plain_message: &str) -> [u8; 32] {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(plain_message.as_bytes());
    let code = mac.finalize().into_bytes();
    code.into()
}
