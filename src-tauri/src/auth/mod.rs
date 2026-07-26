use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::app::client_api::{Client, UserInformation};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ClientAccount {}

impl ClientAccount {
    pub fn is_expired(&self) -> bool {
        false
    }

    pub fn get_access_token(&self) -> &str {
        ""
    }

    pub fn get_refresh_token(&self) -> &str {
        ""
    }

    pub fn get_expires_at(&self) -> u64 {
        0
    }

    pub async fn update_info(&mut self, _client: &Client) -> Result<()> {
        Ok(())
    }

    pub fn get_user_information(&self) -> Option<UserInformation> {
        None
    }

    pub async fn renew(self) -> Result<ClientAccount> {
        Ok(self)
    }
}
