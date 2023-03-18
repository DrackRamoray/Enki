use crate::model::kv::KV;
use crate::plugins::database::DbInstance;
use crate::utils::error::Error;
use enki_shared::auth::AuthData;
use tauri::plugin::Plugin;
use tauri::{Invoke, Runtime, State};

#[tauri::command]
async fn save_auth_data(db: State<'_, DbInstance>, token: String) -> Result<(), Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    let auth = serde_json::to_string(&AuthData { token })?;

    let _ = KV::save(pool, "auth", auth.as_str()).await?;

    Ok(())
}

#[tauri::command]
async fn load_auth_data(db: State<'_, DbInstance>) -> Result<AuthData, Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    KV::retrieve::<AuthData>(pool, "auth").await
}

pub struct AuthPlugin<R: Runtime> {
    invoke_handler: Box<dyn Fn(Invoke<R>) -> bool + Send + Sync>,
}

impl<R: Runtime> AuthPlugin<R> {
    pub fn new() -> Self {
        Self {
            invoke_handler: Box::new(tauri::generate_handler![save_auth_data, load_auth_data]),
        }
    }
}

impl<R: Runtime> Plugin<R> for AuthPlugin<R> {
    fn name(&self) -> &'static str {
        "auth"
    }

    fn extend_api(&mut self, message: Invoke<R>) -> bool {
        (self.invoke_handler)(message)
    }
}
