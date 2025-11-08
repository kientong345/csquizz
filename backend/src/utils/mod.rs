use serde::{Deserialize, Deserializer, Serialize, Serializer, de::DeserializeOwned};

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

#[allow(non_snake_case)]
pub fn serializeCamelCase<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    // Sử dụng một intermediate struct với camelCase
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Camel<'a, T: Serialize>(&'a T);

    Camel(value).serialize(serializer)
}

// Helper để deserialize snake_case
pub fn deserialize_snake_case<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "snake_case")]
    struct Snake<T>(T);

    Snake::deserialize(deserializer).map(|s| s.0)
}
