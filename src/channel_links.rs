use std::{
    collections::HashMap,
    env,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use serde::{Deserialize, Serialize};

use serenity::{
    async_trait,
    framework::standard::{
        macros::{command, group, hook},
        Args, CommandResult, StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use serenity::model::id::ChannelId;
use std::collections::hash_map::RandomState;
use tokio::sync::RwLock;

pub struct ChannelLinks;

pub const CHANNEL_LINKS_PATH: &'static str = "channel_links.json";

impl TypeMapKey for ChannelLinks {
    type Value = Arc<RwLock<HashMap<ChannelId, ChannelId>>>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SavedChannelLink {
    pub from: ChannelId,
    pub to: ChannelId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedChannelLinks {
    pub links: Vec<SavedChannelLink>,
}

impl From<&HashMap<ChannelId, ChannelId>> for SavedChannelLinks {
    fn from(map: &HashMap<ChannelId, ChannelId, RandomState>) -> Self {
        Self {
            links: map
                .into_iter()
                .map(|(k, v)| SavedChannelLink { from: *k, to: *v })
                .collect(),
        }
    }
}

impl Into<HashMap<ChannelId, ChannelId>> for SavedChannelLinks {
    fn into(self) -> HashMap<ChannelId, ChannelId, RandomState> {
        self.links
            .into_iter()
            .map(|link| (link.from, link.to))
            .collect()
    }
}
