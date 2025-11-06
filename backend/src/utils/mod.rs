use serde::{de::DeserializeOwned, Serialize};

pub fn vec_stringify<T: ToString>(vec: Vec<T>) -> Vec<String> {
    let mut ret = Vec::new();
    for element in vec {
        ret.push(element.to_string());
    }
    ret
}

pub fn validate_email_name(email: &str) -> Result<(), String> {
    Ok(())
}

pub fn decode_jwt<C: Clone + DeserializeOwned>(
    jwt: &str,
    secret: &[u8],
) -> Result<C, jsonwebtoken::errors::Error> {
    Ok(jsonwebtoken::decode::<C>(
        jwt,
        &jsonwebtoken::DecodingKey::from_secret(&secret),
        &jsonwebtoken::Validation::default(),
    )?
    .claims)
}

pub fn generate_jwt<C: Serialize>(claims: &C, secret: &[u8]) -> String {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(&secret),
    )
    .unwrap_or(String::from(""))
}

pub fn bcrypt_hash(value: &str) -> Result<String, bcrypt::BcryptError> {
    Ok(bcrypt::hash(value, bcrypt::DEFAULT_COST)?)
}
