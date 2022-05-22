use std::convert::TryInto;

use chrono::prelude::*;
use eyre::Result;

use crate::{
    api_client,
    database::Database,
    encryption::{encrypt, load_encoded_key, load_key},
    settings::{Settings, HISTORY_PAGE_SIZE},
};

use atuin_common::api::{
    ProUpgradeResponse
};

pub async fn upgrade(settings: &Settings, callback_port: u16) -> Result<ProUpgradeResponse> {
    let client = api_client::Client::new(
        &settings.sync_address,
        &settings.session_token,
        load_encoded_key(settings)?, // TODO: key not needed
    )?;

    Ok(client.pro_upgrade(callback_port).await?)
}
