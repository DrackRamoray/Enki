use serde::de::DeserializeOwned;
use serde::Deserialize;
use sqlx::{FromRow, Pool, Sqlite};

use crate::utils::error::Error;

#[derive(FromRow, Deserialize)]
pub struct KV {
    pub id: i64,
    pub name: String,
    pub value: String,
}

impl KV {
    pub async fn retrieve<T>(pool: &Pool<Sqlite>, name: &str) -> Result<T, Error>
    where
        T: Default + DeserializeOwned,
    {
        let result = sqlx::query_as::<_, KV>("SELECT id, name, value FROM kv WHERE name = ?")
            .bind(name)
            .fetch_optional(pool)
            .await?;

        if let Some(kv) = result {
            let value = serde_json::from_str::<T>(&kv.value)?;
            return Ok(value);
        }

        Ok(Default::default())
    }

    pub async fn save(pool: &Pool<Sqlite>, name: &str, value: &str) -> Result<i64, Error> {
        let result = sqlx::query("INSERT OR REPLACE INTO kv (name, value) VALUES (?, ?)")
            .bind(name)
            .bind(value)
            .execute(pool)
            .await?;

        Ok(result.last_insert_rowid())
    }
}
