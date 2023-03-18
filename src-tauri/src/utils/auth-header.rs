use crate::model::kv::KV;
use crate::utils::error::Error;
use enki_shared::auth::AuthData;
use reqwest::header::HeaderValue;
use sqlx::{Pool, Sqlite};
use std::str::FromStr;
use tauri::http::header::HeaderName;

pub async fn get_auth_header(pool: &Pool<Sqlite>) -> Result<(HeaderName, HeaderValue), Error> {
    let auth = KV::retrieve::<AuthData>(pool, "auth").await?;
    let auth_header = auth.into_auth_header();

    let name = HeaderName::from_str(auth_header.get_name())
        .map(|name| name)
        .map_err(|err| {
            println!("build auth name failed: {:?}", err);
            Error::InvalidHeaderName(err)
        })?;

    let value = HeaderValue::from_str(auth_header.get_value())
        .map(|value| value)
        .map_err(|err| {
            println!("build auth header failed: {:?}", err);
            Error::InvalidHeaderValue(err)
        })?;

    Ok((name, value))
}
