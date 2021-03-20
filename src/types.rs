use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{
    permissions::{ChannelPermissions, HubPermissions},
    uuid_from_num_string, ID,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// An Account, a "subidenty" for a user, exists so that a user can have multiple accounts without signing up multiple times, if
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Account {
    pub id: ID,
    pub username: String,
    pub created: u128,
    pub parent_id: String,
    pub is_bot: bool,
    pub in_hubs: Vec<ID>,
}

impl Account {
    /// Hashes the IDs of hubs the account is a member of and outputs a GenericAccount
    pub fn to_generic(&self) -> GenericAccount {
        let mut hasher = Sha256::new();
        let mut hubs_hashed = Vec::new();
        for hub in self.in_hubs.clone() {
            hasher.update(hub.to_string());
            hubs_hashed.push(format!("{:x}", hasher.finalize_reset()));
        }
        GenericAccount {
            id: self.id.clone(),
            created: self.created.clone(),
            username: self.username.clone(),
            parent_id: self.parent_id.clone(),
            is_bot: self.is_bot.clone(),
            hubs_hashed,
        }
    }
}

/// A version of an Account, uses a hashed version of the list of hubs the user is a member of, the is_bot variable indicates whether or not the account is dedicated to automation/chatbot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GenericAccount {
    pub id: ID,
    pub username: String,
    pub created: u128,
    pub parent_id: String,
    pub is_bot: bool,
    pub hubs_hashed: Vec<String>,
}

/// Represents a user, keeps track of which accounts it owns and their metadata.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub id: String,
    pub email: String,
    pub created: u128,
    pub service: String,
    pub accounts: HashMap<ID, Account>,
}

impl User {
    /// Converts a HashMap of Accounts into a a HashMap of Generic Accounts.
    pub fn accounts_generic(users: &HashMap<ID, Account>) -> HashMap<ID, GenericAccount> {
        users
            .iter()
            .map(|e| (e.0.clone(), e.1.to_generic()))
            .collect()
    }

    /// Converts the standard user into a GenericUser.
    pub fn to_generic(&self) -> GenericUser {
        GenericUser {
            id: self.id.clone(),
            created: self.created.clone(),
            users: Self::accounts_generic(&self.accounts),
        }
    }
}

/// Represents the publicly available information on a user, (excludes their email address and the service they signed up with) also only includes the generic version of accounts.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GenericUser {
    pub id: String,
    pub created: u128,
    pub users: HashMap<ID, GenericAccount>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HubMember {
    pub account: ID,
    pub joined: u128,
    pub hub: ID,
    pub nickname: String,
    pub ranks: Vec<ID>,
    pub hub_permissions: HubPermissions,
    pub channel_permissions: HashMap<ID, ChannelPermissions>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PermissionGroup {
    pub id: ID,
    pub name: String,
    pub members: Vec<ID>,
    pub hub_permissions: HubPermissions,
    pub channel_permissions: HashMap<ID, ChannelPermissions>,
    pub created: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Hub {
    pub channels: HashMap<ID, Channel>,
    pub members: HashMap<ID, HubMember>,
    pub bans: HashSet<ID>,
    pub owner: ID,
    pub ranks: HashMap<ID, PermissionGroup>,
    pub default_rank: ID,
    pub name: String,
    pub id: ID,
    pub created: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Channel {
    #[serde(skip)]
    pub messages: Vec<Message>,
    pub id: ID,
    pub hub_id: ID,
    pub name: String,
    pub created: u128,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub id: ID,
    pub sender: ID,
    pub created: u128,
    pub content: String,
}

impl ToString for Message {
    fn to_string(&self) -> String {
        format!(
            "{},{},{},{}",
            self.id.as_u128(),
            self.sender.as_u128(),
            self.created,
            self.content.replace('\n', r#"\n"#)
        )
    }
}

impl FromStr for Message {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(4, ',');
        if let Some(id_str) = parts.next() {
            if let Ok(id) = uuid_from_num_string(id_str) {
                if let Some(sender_str) = parts.next() {
                    if let Ok(sender) = uuid_from_num_string(sender_str) {
                        if let Some(created_str) = parts.next() {
                            if let Ok(created) = created_str.parse::<u128>() {
                                if let Some(content) = parts.next() {
                                    return Ok(Self {
                                        id,
                                        sender,
                                        created,
                                        content: content.replace(r#"\n"#, "\n"),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        return Err(());
    }
}
