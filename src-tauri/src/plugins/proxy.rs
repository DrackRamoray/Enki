use crate::model::kv::KV;
use crate::plugins::database::DbInstance;
use crate::utils::error::Error;
use enki_shared::proxy::ProxyData;
use tauri::plugin::Plugin;
use tauri::{Invoke, Runtime, State};

#[tauri::command]
async fn save_proxy_data(
    db: State<'_, DbInstance>,
    protocol: String,
    server: String,
    port: String,
) -> Result<(), Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    let proxy = serde_json::to_string(&ProxyData {
        protocol,
        server,
        port,
    })?;

    let _ = KV::save(pool, "proxy", proxy.as_str()).await?;

    Ok(())
}

#[tauri::command]
async fn load_proxy_data(db: State<'_, DbInstance>) -> Result<ProxyData, Error> {
    let instance = db.get_instance().lock().await;
    let pool = instance.as_ref().ok_or(Error::DatabaseNotLoaded)?;

    KV::retrieve::<ProxyData>(pool, "proxy").await
}

pub struct ProxyPlugin<R: Runtime> {
    invoke_handler: Box<dyn Fn(Invoke<R>) -> bool + Send + Sync>,
}

impl<R: Runtime> ProxyPlugin<R> {
    pub fn new() -> Self {
        Self {
            invoke_handler: Box::new(tauri::generate_handler![save_proxy_data, load_proxy_data]),
        }
    }
}

impl<R: Runtime> Plugin<R> for ProxyPlugin<R> {
    fn name(&self) -> &'static str {
        "proxy"
    }

    fn extend_api(&mut self, message: Invoke<R>) -> bool {
        (self.invoke_handler)(message)
    }
}
