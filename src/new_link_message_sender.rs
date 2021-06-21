use crate::channel_links::{ChannelLinks, SavedChannelLinks};
use serenity::prelude::TypeMapKey;
use std::path::{PathBuf};

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct NewLinkMessageSender {}

impl TypeMapKey for NewLinkMessageSender {
    type Value = UnboundedSender<()>;
}

pub async fn new_link_message_receiver_loop(
    mut receiver: UnboundedReceiver<()>,
    channel_links: <ChannelLinks as TypeMapKey>::Value,
    channel_links_path: PathBuf,
) {
    loop {
        if let Some(_) = receiver.recv().await {
            let links = channel_links.read().await;
            let saved_links: SavedChannelLinks = (&*links).into();

            match serde_json::to_string(&saved_links) {
                Ok(json) => {
                    // We want to wait for the write to finish here, hence we don't use tokio's async fs functionality
                    if let Err(e) = std::fs::write(&channel_links_path, &json) {
                        println!(
                            "Error {} occurred while saving links {} to path {:?}",
                            e, json, channel_links_path
                        );
                    }
                }
                Err(e) => println!("Error trying to convert {:?} to json: {}", saved_links, e),
            }
        } else {
            break;
        }
    }
}
