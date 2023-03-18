use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AuthData {
    pub token: String,
}

impl AuthData {
    pub fn into_auth_header(self) -> AuthHeader {
        AuthHeader {
            name: "Authorization".to_owned(),
            value: format!("Bearer {}", self.token),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AuthHeader {
    pub name: String,
    pub value: String,
}

impl AuthHeader {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_value(&self) -> &str {
        self.value.as_str()
    }
}
