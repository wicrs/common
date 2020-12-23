use std::str::FromStr;

use crate::ID;
use serde::{Deserialize, Serialize};

/// Always sent in requests that require authentication, sent as URL query (`?user={User ID}&token={Auth Token}`)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserToken {
    pub user: String,
    pub token: String,
}

/// Data required for /api/v1/user/addaccount
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CreateAccountQuery {
    pub name: String,
    pub is_bot: bool,
}

/// Data required for /api/v1/hubs/create
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HubCreateQuery {
    pub account: ID,
    pub name: String,
}

/// Data required for /api/v1/hubs/create_channel
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ChannelCreateQuery {
    pub account: ID,
    pub hub: ID,
    pub name: String,
}

/// Data required for /api/v1/hubs/send_message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MessageSendQuery {
    pub account: ID,
    pub  hub: ID,
    pub channel: ID,
    pub message: String,
}

/// Data required for /api/v1/hubs/channels
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ChannelsQuery {
    pub account: ID,
    pub hub: ID,
}

/// Data required for /api/v1/hubs/messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LastMessagesQuery {
    pub account: ID,
    pub hub: ID,
    pub channel: ID,
    pub count: u128,
}

/// Data required for /api/v1/auth/{Service}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthQuery {
    pub state: String,
    pub code: String,
    pub expires: Option<u128>,
}

/// Response for /api/v1/auth/{Service}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AccountTokenResponse {
    pub id: String,
    pub token: String,
}

/// Sent as part of the URL path when authenticating e.g. /api/v1/login/{Service}
pub enum Service {
    GitHub,
}

impl FromStr for Service {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "github" => Ok(Self::GitHub),
            _ => Err(()),
        }
    }
}

impl ToString for Service {
    fn to_string(&self) -> String {
        match self {
            &Service::GitHub => "github".to_string(),
        }
    }
}
