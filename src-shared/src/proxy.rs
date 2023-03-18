use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProxyData {
    pub protocol: String,
    pub server: String,
    pub port: String,
}

impl ProxyData {
    pub fn with_proxy(&self) -> bool {
        self.protocol != "" && self.protocol != "direct"
    }

    pub fn get_proxy(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.server, self.port)
    }
}
