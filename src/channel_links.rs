use std::{
    collections::HashMap,
    env,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

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
use tokio::sync::RwLock;

pub struct ChannelLinks;

impl TypeMapKey for ChannelLinks {
    type Value = Arc<RwLock<HashMap<ChannelId, ChannelId>>>;
}
