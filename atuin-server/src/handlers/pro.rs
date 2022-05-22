use std::collections::HashMap;

use axum::{Extension, Json};
use tracing::{debug, error, instrument};
use eyre::{Result};
use uuid::Uuid;

use serde::{Deserialize};

use reqwest::{
    StatusCode,
};

use super::{ErrorResponse, ErrorResponseStatus};
use crate::{
    database::{Database, Postgres},
    models::{User},
    settings::Settings,
};

use atuin_common::api::*;

#[derive(Deserialize)]
struct StripeCheckoutSessionResponse {
    url: String,
}

#[instrument(skip_all, fields(user.id = user.id))]
pub async fn upgrade(
    Json(req): Json<ProUpgradeRequest>,
    user: User,
    _db: Extension<Postgres>,
) -> Result<Json<ProUpgradeResponse>, ErrorResponseStatus<'static>> {
    debug!("request to upgrade to pro 💸");

    let success_url = format!("http://127.0.0.1:{}/success", req.callback_port);
    let cancel_url = format!("http://127.0.0.1:{}/cancel", req.callback_port);
    let user_id = user.id.to_string();
    let mut map = HashMap::new();
    map.insert("line_items[0][price]", "price_1L1RXVDnXJakRqOztV2nezpo");
    map.insert("line_items[0][quantity]", "1");
    map.insert("success_url", success_url.as_str());
    map.insert("cancel_url", cancel_url.as_str());
    map.insert("customer_email", &user.email);
    map.insert("mode", "subscription");
    map.insert("subscription_data[trial_period_days]", "30");
    map.insert("subscription_data[metadata][user_id]", &user_id);
    map.insert("client_reference_id", &user_id);

    let client = reqwest::Client::new();

    let user_name = "sk_test_51L1RSfDnXJakRqOz9J07ece3BWE8e0lg5Uy8eu07ixY3rkdhNTmPTKd3vfh9cM2GLfK8I9Uqvxfdbsx6z05p2KGN00b94GbqfI".to_string();
    let password: Option<String> = None;
    let resp = client
        .post("https://api.stripe.com/v1/checkout/sessions")
        .basic_auth(user_name, password)
        .form(&map)
        .send()
        .await;

    if let Err(e) = resp {
        error!("failed to create checkout session: {}", e);
        return Err(ErrorResponse::reply("failed to create checkout session1")
            .with_status(StatusCode::INTERNAL_SERVER_ERROR));
    }

    let session = resp.unwrap();

    if !session.status().is_success() {
        let body = session.text().await.unwrap();
        error!("failed to create checkout session: {}", body);
        return Err(ErrorResponse::reply("fml")
                .with_status(StatusCode::INTERNAL_SERVER_ERROR))
    }

    let checkout_url = session
        .json::<StripeCheckoutSessionResponse>()
        .await
        .unwrap()
        .url;

    debug!("checkout url {}", checkout_url);

    Ok(Json(ProUpgradeResponse{checkout_url: checkout_url}))
}
