use crate::utils::error::Error;
use enki_shared::audio::AudioData;
use enki_shared::convert_src::convert_src;
use futures::TryStreamExt;
use serde::Deserialize;
use sqlx::{FromRow, Pool, Sqlite};

#[derive(FromRow, Deserialize)]
pub struct Audio {
    pub id: i64,
    pub file: String,
    pub message: String,
    pub response: String,
    pub topic_id: i64,
}

impl Audio {
    pub async fn retrieve_records(
        pool: &Pool<Sqlite>,
        topic_id: i64,
    ) -> Result<Vec<AudioData>, Error> {
        let mut records = Vec::new();
        let mut rows = sqlx::query_as::<_, Audio>(
            "SELECT id, file, message, response, topic_id FROM audio WHERE topic_id = ? ORDER BY id",
        )
        .bind(topic_id)
        .fetch(pool);

        while let Some(row) = rows.try_next().await? {
            records.push(AudioData {
                id: row.id,
                file: row.file,
                prompt: row.message,
                response: row.response,
            });
        }

        Ok(records)
    }

    pub async fn create_audio(
        pool: &Pool<Sqlite>,
        file: &str,
        message: &str,
        topic_id: i64,
    ) -> Result<i64, Error> {
        let file_url = convert_src(file);

        let result = sqlx::query("INSERT INTO audio (file, message, topic_id) VALUES (?, ?, ?)")
            .bind(file_url)
            .bind(message)
            .bind(topic_id)
            .execute(pool)
            .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn update_audio(
        pool: &Pool<Sqlite>,
        response: &str,
        audio_id: i64,
    ) -> Result<i64, Error> {
        let result = sqlx::query("UPDATE audio SET response = ? WHERE id = ?")
            .bind(response)
            .bind(audio_id)
            .execute(pool)
            .await?;

        Ok(result.last_insert_rowid())
    }
}
