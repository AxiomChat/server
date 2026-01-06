use crate::{server::Server, utils::client::Client};
use std::sync::Arc;

crate::logger!(LOGGER "Voice chat");

pub fn voice(server: &Arc<Server>, client: &Client, data: &[u8]) -> crate::Result<()> {
    let prefix = (10u16).to_le_bytes();

    let mut payload = Vec::with_capacity(prefix.len() + data.len());
    payload.extend_from_slice(&prefix);
    payload.extend_from_slice(data);

    client.send_bin(&payload)?;
    Ok(())
}
