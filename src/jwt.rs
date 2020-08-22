use uuid::Uuid;
use chrono::prelude::*;
use chrono::{Duration};
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const SECRET: &str = "sooper-secret";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claims {
  #[serde(with = "jwt_numeric_date")]
  iat: DateTime<Utc>, // issued at
  #[serde(with = "jwt_numeric_date")]
  exp: DateTime<Utc>, // expires
  jti: String, // Case sensitive unique identifier of the token even among different issuers. 
  sub: String, // User ID
  name: String, // User name
  role: String, // User role
  permission: Vec<String> // Permissions associated with role
}


pub fn create_token(
  exp_seconds: i64,
  identity: &str,
  user_name: &str,
  user_role: &str,
  user_permission: &Vec<String>
) -> Result<String, Box<dyn std::error::Error>> {
  let iat = Utc::now();
  let exp = iat + Duration::seconds(exp_seconds);
  let claims = Claims {
    iat,
    exp,
    jti: Uuid::new_v4().to_string(),
    sub: identity.to_string(),
    name: user_name.to_string(),
    role: user_role.to_string(),
    permission: user_permission.clone(),
  };
  let token = jsonwebtoken::encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(SECRET.as_ref())
  )?;
  Ok(token)
}


mod jwt_numeric_date {
  //! Custom serialization of DateTime<Utc> to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
  use chrono::{DateTime, TimeZone, Utc};
  use serde::{self, Deserialize, Deserializer, Serializer};

  /// Serializes a DateTime<Utc> to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
  pub fn serialize<S>(date: &DateTime<Utc>, seriaizer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let timestamp = date.timestamp();
    seriaizer.serialize_i64(timestamp)
  }

  /// Attempts to deserialize an i64 and use as a Unix timestamp
  pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
  where
    D: Deserializer<'de>
  {
    Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
      .single() // If there are multiple or no valid DateTimes from timestamp, return None
      .ok_or_else(|| serde::de::Error::custom("Invalid Unix timestamp value"))
  }
}