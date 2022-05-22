


use eyre::Result;

use crate::{
    api_client,
    encryption::{load_encoded_key},
    settings::{Settings},
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

    client.pro_upgrade(callback_port).await
}
