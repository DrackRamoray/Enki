use crate::model::image_response::ImageResponse;
use crate::utils::error::Error;
use enki_shared::{
    image_edit_record::ImageEditRecord, image_generate_record::ImageGenerateRecord,
    image_variate_record::ImageVariateRecord,
};
use futures::TryStreamExt;
use serde::Deserialize;
use sqlx::{FromRow, Pool, Sqlite};

#[derive(Deserialize, FromRow)]
pub struct Image {
    pub id: i64,
    pub message: Option<String>,
    pub original_image: Option<String>,
    pub mask: Option<String>,
    pub category: String,
    pub topic_id: i64,
}

impl Image {
    pub async fn create_image(
        pool: &Pool<Sqlite>,
        message: Option<String>,
        original_image: Option<String>,
        mask: Option<String>,
        category: String,
        topic_id: i64,
    ) -> Result<i64, Error> {
        let result = sqlx::query(
            "INSERT INTO image (message, original_image, mask, category, topic_id ) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(message)
        .bind(original_image)
        .bind(mask)
        .bind(category)
        .bind(topic_id)
        .execute(pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn retrieve_generate_record(
        pool: &Pool<Sqlite>,
        topic_id: i64,
    ) -> Result<Vec<ImageGenerateRecord>, Error> {
        let mut result = Vec::new();

        let mut rows = sqlx::query_as::<_, Image>(
            "SELECT id, message, original_image, mask, category, topic_id FROM image WHERE topic_id = ? ORDER BY id",
        )
        .bind(topic_id)
        .fetch(pool);

        while let Some(row) = rows.try_next().await? {
            let responses = ImageResponse::retrieve_image_responses(pool, row.id).await?;
            let record = ImageGenerateRecord {
                prompt: row.message.unwrap_or_default(),
                images: responses.into_iter().map(|item| item.uri).collect(),
            };
            result.push(record);
        }

        Ok(result)
    }

    pub async fn retrieve_edit_record(
        pool: &Pool<Sqlite>,
        topic_id: i64,
    ) -> Result<Vec<ImageEditRecord>, Error> {
        let mut result = Vec::new();

        let mut rows = sqlx::query_as::<_, Image>(
            "SELECT id, message, original_image, mask, category, topic_id FROM image WHERE topic_id = ? ORDER BY id",
        )
        .bind(topic_id)
        .fetch(pool);

        while let Some(row) = rows.try_next().await? {
            let responses = ImageResponse::retrieve_image_responses(pool, row.id).await?;
            let record = ImageEditRecord {
                image: row.original_image.unwrap_or_default(),
                mask: row.mask,
                prompt: row.message.unwrap_or_default(),
                images: responses.into_iter().map(|item| item.uri).collect(),
            };
            result.push(record);
        }

        Ok(result)
    }

    pub async fn retrieve_variate_record(
        pool: &Pool<Sqlite>,
        topic_id: i64,
    ) -> Result<Vec<ImageVariateRecord>, Error> {
        let mut result = Vec::new();

        let mut rows = sqlx::query_as::<_, Image>(
            "SELECT id, message, original_image, mask, category, topic_id FROM image WHERE topic_id = ? ORDER BY id",
        )
        .bind(topic_id)
        .fetch(pool);

        while let Some(row) = rows.try_next().await? {
            let responses = ImageResponse::retrieve_image_responses(pool, row.id).await?;
            let record = ImageVariateRecord {
                image: row.original_image.unwrap_or_default(),
                images: responses.into_iter().map(|item| item.uri).collect(),
            };
            result.push(record);
        }

        Ok(result)
    }
}
