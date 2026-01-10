use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MslConfig {
    pub docker: DockerConfig,
    pub frp: FrpConfig,
    pub msl: GeneralConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DockerConfig {
    pub enabled: bool,
    pub use_podman: bool,
    pub image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrpConfig {
    pub enabled: bool,
    pub client_path: String,
    pub config_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub log_level: String,
    pub auto_restart: bool,
}

impl Default for MslConfig {
    fn default() -> Self {
        Self {
            docker: DockerConfig {
                enabled: false,
                use_podman: false,
                image: "openjdk:21".to_string(),
            },
            frp: FrpConfig {
                enabled: false,
                client_path: "./frpc".to_string(),
                config_path: "configs/MSL/frpc.ini".to_string(),
            },
            msl: GeneralConfig {
                log_level: "info".to_string(),
                auto_restart: false,
            },
        }
    }
}