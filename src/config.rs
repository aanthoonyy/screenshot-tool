// Chatgpt shit need to tweak later there is for sure a better way to do this 
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    save_dir: Option<PathBuf>,
    filename_format: Option<String>,
    extension: Option<String>,
    copy_to_clipboard: Option<bool>,
    open_editor: Option<bool>,
}

impl Config {
    pub fn load() -> Self {
        let config_path = dirs::config_dir()
            .map(|d| d.join("shrust").join("config.toml"))
            .expect("could not resolve config directory");

        let mut config = Config {
            save_dir: None,
            filename_format: None,
            extension: None,
            copy_to_clipboard: None,
            open_editor: None,
        };

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)
                .expect("failed to read config.toml");

            let parsed: Config = toml::from_str(&contents)
                .expect("invalid TOML format");

            config = parsed;
        }

        if config.save_dir.is_none() {
            config.save_dir = dirs::picture_dir();
        }

        if config.filename_format.is_none() {
            config.filename_format = Some("screenshot-%Y-%m-%d-%H-%M-%S".to_string());
        }

        if config.extension.is_none() {
            config.extension = Some("png".to_string());
        }

        if config.copy_to_clipboard.is_none() {
            config.copy_to_clipboard = Some(true);
        }

        if config.open_editor.is_none() {
            config.open_editor = Some(true);
        }

        config
    }

    pub fn save_dir(&self) -> &PathBuf {
        self.save_dir.as_ref().expect("save_dir should always be set")
    }

    pub fn filename_format(&self) -> &str {
        self.filename_format.as_deref().unwrap()
    }

    pub fn extension(&self) -> &str {
        self.extension.as_deref().unwrap()
    }

    pub fn copy_to_clipboard(&self) -> bool {
        self.copy_to_clipboard.unwrap()
    }

    pub fn open_editor(&self) -> bool {
        self.open_editor.unwrap()
    }
}
