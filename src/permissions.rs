use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(PartialEq, Hash, Eq, Serialize, Deserialize, Clone, Debug)]
pub enum PermissionSetting {
    TRUE,
    FALSE,
    NONE,
}

#[derive(PartialEq, Hash, Eq, Serialize, Deserialize, Clone, Debug)]
pub enum HubPermission {
    All,
    ViewChannels,
    ConfigureChannels,
    Administrate,
    CreateChannel,
    DeleteChannel,
    CreateCategory,
    DeleteCategory,
    ArrangeChannels,
    SendMessage,
    ReadMessage,
    Invite,
    Unmute,
    Mute,
    Kick,
    Ban,
    Unban,
    AddBot,
}

pub type HubPermissions = HashMap<HubPermission, PermissionSetting>;

#[derive(PartialEq, Hash, Eq, Serialize, Deserialize, Clone, Debug)]
pub enum ChannelPermission {
    SendMessage,
    ReadMessage,
    ViewChannel,
    Configure,
    MuteUser,
    All,
}

impl ChannelPermission {
    pub fn hub_equivalent(&self) -> HubPermission {
        match self {
            ChannelPermission::SendMessage => HubPermission::SendMessage,
            ChannelPermission::ReadMessage => HubPermission::ReadMessage,
            ChannelPermission::ViewChannel => HubPermission::ViewChannels,
            ChannelPermission::Configure => HubPermission::ConfigureChannels,
            ChannelPermission::MuteUser => HubPermission::Mute,
            ChannelPermission::All => HubPermission::All,
        }
    }
}

pub type ChannelPermissions = HashMap<ChannelPermission, PermissionSetting>;
