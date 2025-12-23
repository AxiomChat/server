use crate::{server::Server, types::message::ServerMessage, utils::client::Client};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params", rename_all = "snake_case")]
pub enum Indicator {
    Typing { user_id: String, channel_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorContext {
    pub indicator: Indicator,
    pub expires: u16,
}

impl Server {
    pub fn spawn_indicator_thread(self: &Arc<Self>) {
        let server = self.clone();
        std::thread::spawn(move || {
            loop {
                let mut remove = Vec::new();
                for (i, indicator) in server.indicators.lock().unwrap().iter_mut().enumerate() {
                    indicator.expires -= 1;
                    if indicator.expires == 0 {
                        remove.push(i);
                    }
                }
                for r in remove {
                    server.indicators.lock().unwrap().remove(r);
                }
            }
        });
    }
}

crate::logger!(LOGGER "Typing Indicator");

pub fn start_typing(server: &Arc<Server>, client: &Client, channel_id: &str) -> crate::Result<()> {
    let indicator = IndicatorContext {
        indicator: Indicator::Typing {
            user_id: client.get_uuid().context("Failed to get uuid")?,
            channel_id: channel_id.to_string(),
        },
        expires: 2, // 2 secs
    };

    server.broadcast(ServerMessage::Indicator(indicator.clone()));

    server.indicators.lock().unwrap().push(indicator);

    Ok(())
}
