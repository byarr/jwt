use std::error::Error;
use data_encoding::{BASE64URL_NOPAD};
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum JwtDecodeError {
    BadFormat
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
enum JwtType {
    JWS,
    JWE,
}

impl Display for JwtDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for JwtDecodeError{}

pub fn decode_jwt(input: &str) -> Result<(), Box<dyn Error>>{

    // see https://datatracker.ietf.org/doc/html/rfc7519#section-7.2
    let parts: Vec<_> = input.split('.').collect();
    if parts.len() < 2 {
        // 1.
        return Err(JwtDecodeError::BadFormat.into());
    }

    let header_bytes = BASE64URL_NOPAD.decode(parts[0].as_bytes())?;
    let header = String::from_utf8(header_bytes)?;
    println!("{:?}", header);
    // 4.
    let header_obj: Value = serde_json::from_str(&header)?;
    println!("{:?}", header_obj);
    // TODO 5. Verify that the resulting JOSE Header includes only parameters
    //         and values whose syntax and semantics are both understood and
    //         supported or that are specified as being ignored when not
    //         understood.

    let token_type = if header_obj.is_object() && header_obj.as_object().map(|o| o.contains_key("enc")).unwrap_or(false) {
        JwtType::JWE
    } else {
        JwtType::JWS
    };

    println!("Token type: {:?}", token_type);

    let body_bytes = BASE64URL_NOPAD.decode(parts[1].as_bytes())?;
    let body = String::from_utf8(body_bytes)?;
    let body_obj: Value = serde_json::from_str(&body)?;
    println!("{:?}", body_obj);

    Ok(())
}