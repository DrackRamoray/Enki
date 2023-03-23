use crate::model::kv::KV;
use crate::utils::auth_header::get_auth_header;
use crate::utils::error::Error;
use enki_shared::proxy::ProxyData;
use reqwest::header::HeaderMap;
use reqwest::Client;
use reqwest::Proxy;
use sqlx::{Pool, Sqlite};
use std::time::Duration;

pub async fn create_client(pool: &Pool<Sqlite>) -> Result<Client, Error> {
    let proxy = KV::retrieve::<ProxyData>(pool, "proxy").await?;
    let client = Client::builder();

    let client = match proxy.with_proxy() {
        true => match Proxy::https(proxy.get_proxy()) {
            Ok(https_proxy) => client.proxy(https_proxy),
            Err(err) => return Err(Error::ReqError(err)),
        },
        false => client,
    };

    let mut headers = HeaderMap::new();
    let (name, value) = get_auth_header(pool).await?;

    headers.insert(&name, value);

    client
        .default_headers(headers)
        .timeout(Duration::from_secs(300))
        .build()
        .map(|client| client)
        .map_err(|err| {
            println!("build client failed: {:?}", err);
            Error::ReqError(err)
        })
}
