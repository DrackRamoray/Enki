mod gpt;
mod model;
mod plugins;
mod utils;

use plugins::audio_transcript::AudioTranscriptPlugin;
use plugins::audio_translate::AudioTranslatePlugin;
use plugins::auth::AuthPlugin;
use plugins::chat::ChatPlugin;
use plugins::database::{DatabasePluginBuilder, Migration, MigrationKind};
use plugins::image_edit::ImageEditPlugin;
use plugins::image_generate::ImageGeneratePlugin;
use plugins::image_variate::ImageVariatePlugin;
use plugins::proxy::ProxyPlugin;
use plugins::topic::TopicPlugin;

pub fn launch() {
    tauri::Builder::default()
        .plugin(
            DatabasePluginBuilder::default()
                .add_migrations(
                    "enki.db".to_string(),
                    vec![Migration {
                        version: 1,
                        description: "create chat and image",
                        sql: include_str!("../migrations/1.sql"),
                        kind: MigrationKind::Up,
                    }],
                )
                .build(),
        )
        .plugin(AuthPlugin::new())
        .plugin(ProxyPlugin::new())
        .plugin(TopicPlugin::new())
        .plugin(ChatPlugin::new())
        .plugin(ImageGeneratePlugin::new())
        .plugin(ImageEditPlugin::new())
        .plugin(ImageVariatePlugin::new())
        .plugin(AudioTranscriptPlugin::new())
        .plugin(AudioTranslatePlugin::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
