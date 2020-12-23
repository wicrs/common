use std::str::FromStr;

use crate::ID;
use serde::{Deserialize, Serialize};

/// Always sent in requests that require authentication, sent as URL query (`?user={User ID}&token={Auth Token}`)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserToken {
    user: String,
    token: String,
}

/// Data required for /api/v1/user/addaccount
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CreateAccountQuery {
    name: String,
    is_bot: bool,
}

/// Data required for /api/v1/hubs/create
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HubCreateQuery {
    account: ID,
    name: String,
}

/// Data required for /api/v1/hubs/create_channel
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ChannelCreateQuery {
    account: ID,
    hub: ID,
    name: String,
}

/// Data required for /api/v1/hubs/send_message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MessageSendQuery {
    account: ID,
    hub: ID,
    channel: ID,
    message: String,
}

/// Data required for /api/v1/hubs/channels
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ChannelsQuery {
    account: ID,
    hub: ID,
}

/// Data required for /api/v1/hubs/messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LastMessagesQuery {
    account: ID,
    hub: ID,
    channel: ID,
    count: u128,
}

/// Data required for /api/v1/auth/{Service}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthQuery {
    state: String,
    code: String,
    expires: Option<u128>,
}

/// Response for /api/v1/auth/{Service}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AccountTokenResponse {
    id: String,
    token: String,
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
