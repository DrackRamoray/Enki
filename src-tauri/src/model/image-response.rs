use crate::utils::error::Error;
use serde::Deserialize;
use sqlx::{FromRow, Pool, Sqlite};

#[derive(Deserialize, FromRow)]
pub struct ImageResponse {
    pub id: i64,
    pub uri: String,
    pub size: String,
    pub image_id: i32,
    pub topic_id: i32,
}

impl ImageResponse {
    pub async fn create_image_response(
        pool: &Pool<Sqlite>,
        uri: &str,
        size: String,
        image_id: i64,
        topic_id: i64,
    ) -> Result<i64, Error> {
        let result = sqlx::query(
            "INSERT INTO image_response (uri, size, image_id, topic_id) VALUES (?, ?, ?, ?)",
        )
        .bind(uri)
        .bind(size)
        .bind(image_id)
        .bind(topic_id)
        .execute(pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn retrieve_image_responses(
        pool: &Pool<Sqlite>,
        image_id: i64,
    ) -> Result<Vec<ImageResponse>, Error> {
        let result = sqlx::query_as::<_, ImageResponse>(
            "SELECT id, uri, size, image_id, topic_id FROM image_response WHERE image_id = ? ORDER BY id"
        )
            .bind(image_id)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }
}
