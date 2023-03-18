use crate::utils::config_path::{app_config_dir_ignore, path_mapper};
use futures::future::BoxFuture;
use sqlx::sqlite::Sqlite;
use sqlx::{
    error::BoxDynError,
    migrate::{
        MigrateDatabase, Migration as SqlxMigration, MigrationSource, MigrationType, Migrator,
    },
    Pool,
};
use std::fs::create_dir_all;
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Manager, RunEvent, Runtime,
};
use tokio::sync::Mutex;

#[derive(Default, Debug)]
pub struct DbInstance(Mutex<Option<Pool<Sqlite>>>);

impl DbInstance {
    pub fn get_instance(&self) -> &Mutex<Option<Pool<Sqlite>>> {
        &self.0
    }
}

#[derive(Debug)]
pub enum MigrationKind {
    Up,
    Down,
}

impl From<MigrationKind> for MigrationType {
    fn from(kind: MigrationKind) -> Self {
        match kind {
            MigrationKind::Up => Self::ReversibleUp,
            MigrationKind::Down => Self::ReversibleDown,
        }
    }
}

#[derive(Debug)]
pub struct Migration {
    pub version: i64,
    pub description: &'static str,
    pub sql: &'static str,
    pub kind: MigrationKind,
}

#[derive(Debug)]
struct MigrationList(Vec<Migration>);

impl MigrationSource<'static> for MigrationList {
    fn resolve(self) -> BoxFuture<'static, Result<Vec<SqlxMigration>, BoxDynError>> {
        Box::pin(async move {
            let mut migrations = Vec::new();
            for migration in self.0 {
                if matches!(migration.kind, MigrationKind::Up) {
                    migrations.push(SqlxMigration::new(
                        migration.version,
                        migration.description.into(),
                        migration.kind.into(),
                        migration.sql.into(),
                    ));
                }
            }
            Ok(migrations)
        })
    }
}

#[derive(Default)]
pub struct DatabasePluginBuilder {
    db_path: String,
    migrations: Option<MigrationList>,
}

impl DatabasePluginBuilder {
    #[must_use]
    pub fn add_migrations(mut self, db_path: String, migrations: Vec<Migration>) -> Self {
        self.db_path = db_path;
        self.migrations = Some(MigrationList(migrations));
        self
    }

    pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
        PluginBuilder::new("sqlite")
            .invoke_handler(tauri::generate_handler![])
            .setup(|app, _| {
                create_dir_all(app_config_dir_ignore(app))
                    .expect("Creating app config directory failed.");

                tauri::async_runtime::block_on(async move {
                    let instance = DbInstance::default();
                    let mut lock = instance.0.lock().await;
                    let db_path = path_mapper(app_config_dir_ignore(app), &self.db_path);

                    if !Sqlite::database_exists(&db_path).await.unwrap_or(false) {
                        Sqlite::create_database(&db_path).await?;
                    }

                    let pool: Pool<Sqlite> = Pool::connect(&db_path).await?;

                    if let Some(migrations) = self.migrations {
                        let migrator = Migrator::new(migrations).await?;
                        migrator.run(&pool).await?;
                    }

                    let _ = lock.insert(pool);

                    drop(lock);

                    app.manage(instance);

                    Ok(())
                })
            })
            .on_event(|app, event| {
                if let RunEvent::Exit = event {
                    tauri::async_runtime::block_on(async move {
                        let instance = &*app.state::<DbInstance>();
                        let instance = instance.0.lock().await;
                        instance
                            .as_ref()
                            .map(move |inst| async move { inst.close().await });
                    });
                }
            })
            .build()
    }
}
