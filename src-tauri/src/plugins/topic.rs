use crate::model::topic::Topic;
use crate::plugins::database::DbInstance;
use crate::utils::error::Error;
use enki_shared::topic::Topic as TopicData;
use enki_shared::topic_category::TopicCategory;
use tauri::plugin::Plugin;
use tauri::{Invoke, Runtime, State};

#[tauri::command]
async fn create_topic(
    db: State<'_, DbInstance>,
    title: String,
    category: TopicCategory,
) -> Result<TopicData, Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    let result = sqlx::query("INSERT INTO topic (title, category) VALUES (?, ?)")
        .bind(title)
        .bind(category.to_string())
        .execute(pool)
        .await
        .map(|result| result)
        .map_err(|err| {
            println!("insert topic failed: {:?}", err);
            Error::SqlError(err)
        })?;

    sqlx::query_as::<_, Topic>("SELECT * FROM topic WHERE id = $1")
        .bind(result.last_insert_rowid())
        .fetch_one(pool)
        .await
        .map(|result| result.into_topic())
        .map_err(|err| {
            println!(
                "query topic failed(id = {}): {:?}",
                result.last_insert_rowid(),
                err
            );
            Error::SqlError(err)
        })
}

#[tauri::command]
async fn edit_topic(db: State<'_, DbInstance>, id: i64, title: String) -> Result<(), Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    sqlx::query("UPDATE topic SET title = ? WHERE id = ?")
        .bind(title)
        .bind(id)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|err| {
            println!("edit topic failed(id = {}): {:?}", id, err);
            Error::SqlError(err)
        })
}

#[tauri::command]
async fn remove_topic(db: State<'_, DbInstance>, id: i64) -> Result<(), Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    sqlx::query("UPDATE topic SET status = 0 WHERE id =$1")
        .bind(id)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|err| {
            println!("remove topic failed(id = {}): {:?}", id, err);
            Error::SqlError(err)
        })
}

#[tauri::command]
async fn get_topic_list(
    db: State<'_, DbInstance>,
    title: Option<String>,
    category: Option<TopicCategory>,
) -> Result<Vec<TopicData>, Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    let select = "SELECT * FROM topic WHERE status = 1";
    let category_clause = if category.is_some() {
        " AND category = ?"
    } else {
        ""
    };
    let title_clause = if title.is_some() {
        " AND title LIKE '%' || ? || '%'"
    } else {
        ""
    };
    let query = format!(
        "{}{}{} ORDER BY id DESC",
        select, category_clause, title_clause
    );

    let executor = sqlx::query_as::<_, Topic>(&query);

    let executor = if category.is_some() {
        executor.bind(category.unwrap().to_string())
    } else {
        executor
    };

    let executor = if title.is_some() {
        executor.bind(title.unwrap())
    } else {
        executor
    };

    executor
        .fetch_all(pool)
        .await
        .map(|results| {
            results
                .into_iter()
                .map(|result| result.into_topic())
                .collect()
        })
        .map_err(|err| {
            println!("get topic list failed: {:?}", err);
            Error::SqlError(err)
        })
}

pub struct TopicPlugin<R: Runtime> {
    invoke_handler: Box<dyn Fn(Invoke<R>) -> bool + Send + Sync>,
}

impl<R: Runtime> TopicPlugin<R> {
    pub fn new() -> Self {
        Self {
            invoke_handler: Box::new(tauri::generate_handler![
                create_topic,
                edit_topic,
                remove_topic,
                get_topic_list
            ]),
        }
    }
}

impl<R: Runtime> Plugin<R> for TopicPlugin<R> {
    fn name(&self) -> &'static str {
        "topic"
    }

    fn extend_api(&mut self, message: Invoke<R>) -> bool {
        (self.invoke_handler)(message)
    }
}
