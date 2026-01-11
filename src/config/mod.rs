use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MslConfig {
    pub docker: DockerConfig,
    pub frp: FrpConfig,
    pub msl: GeneralConfig,
    pub appearance: AppearanceConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DockerConfig {
    pub enabled: bool,
    pub use_podman: bool,
    pub image: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrpConfig {
    pub enabled: bool,
    pub client_path: String,
    pub config_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    pub log_level: String,
    pub auto_restart: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum ImageMode {
    Protocol,
    Ascii,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppearanceConfig {
    pub image_render_mode: ImageMode,
}

impl Default for MslConfig {
    fn default() -> Self {
        Self {
            docker: DockerConfig {
                enabled: false,
                use_podman: false,
                image: "openjdk:21".into(),
            },
            frp: FrpConfig {
                enabled: false,
                client_path: "./frpc".into(),
                config_path: "configs/MSL/frpc.ini".into(),
            },
            msl: GeneralConfig {
                log_level: "info".into(),
                auto_restart: false,
            },
            appearance: AppearanceConfig {
                image_render_mode: ImageMode::Unknown,
            }
        }
    }
}