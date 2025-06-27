use dioxus::logger::tracing::{error, info};
use ory_kratos_client::apis::configuration::Configuration;
use ory_kratos_client::apis::metadata_api::{is_alive, is_ready};
use ory_kratos_client::apis::{courier_api, frontend_api, identity_api, metadata_api};

pub async fn status_check(configuration: &Configuration) -> String {
  // let version = match metadata_api::get_version(configuration).await {
  //   Ok(r) => &r.version,
  //   Err(e) => e.UnknownValue,
  // };

  match is_alive(configuration).await {
    Ok(r) => info!("Kratos liveliness check: {}", r.status),
    Err(e) => error!("Kratos liveliness check failed! Error: {:?}", e.to_string()),
  };

  match is_ready(configuration).await {
    Ok(r) => info!("Kratos readiness check: {}", r.status),
    Err(e) => error!("Kratos readiness check failed! Error: {:?}", e.to_string()),
  };

  match metadata_api::get_version(configuration).await {
    Ok(r) => r.version,
    Err(e) => e.to_string(),
  }
}
