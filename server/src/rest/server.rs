//! Rest server implementation

use super::api;
use crate::grpc::client::GrpcClients;
use crate::shutdown_signal;
use crate::Config;
use axum::{
    error_handling::HandleErrorLayer,
    extract::Extension,
    http::{HeaderValue, StatusCode},
    routing, BoxError, Router,
};
use std::net::SocketAddr;
use tower::{
    buffer::BufferLayer,
    limit::{ConcurrencyLimitLayer, RateLimitLayer},
    ServiceBuilder,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

/// Starts the REST API server for this microservice
///
/// # Example:
/// ```
/// use svc_assets::rest::server::rest_server;
/// use svc_assets::Config;
/// async fn example() -> Result<(), tokio::task::JoinError> {
///     let config = Config::default();
///     tokio::spawn(rest_server(config, None)).await;
///     Ok(())
/// }
/// ```
#[cfg(not(tarpaulin_include))]
// no_coverage: Needs running backends to work.
// Will be tested in integration tests.
pub async fn rest_server(
    config: Config,
    shutdown_rx: Option<tokio::sync::oneshot::Receiver<()>>,
) -> Result<(), ()> {
    rest_info!("(rest_server) entry.");
    let rest_port = config.docker_port_rest;
    let full_rest_addr: SocketAddr = match format!("[::]:{}", rest_port).parse() {
        Ok(addr) => addr,
        Err(e) => {
            rest_error!("(rest_server) invalid address: {:?}, exiting.", e);
            return Err(());
        }
    };

    let cors_allowed_origin = match config.rest_cors_allowed_origin.parse::<HeaderValue>() {
        Ok(url) => url,
        Err(e) => {
            rest_error!(
                "(rest_server) invalid cors_allowed_origin address: {:?}, exiting.",
                e
            );
            return Err(());
        }
    };

    // Rate limiting
    let rate_limit = config.rest_request_limit_per_second as u64;
    let concurrency_limit = config.rest_concurrency_limit_per_service as usize;
    let limit_middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(HandleErrorLayer::new(|e: BoxError| async move {
            rest_warn!("(server) too many requests: {}", e);
            (
                StatusCode::TOO_MANY_REQUESTS,
                "(server) too many requests.".to_string(),
            )
        }))
        .layer(BufferLayer::new(100))
        .layer(ConcurrencyLimitLayer::new(concurrency_limit))
        .layer(RateLimitLayer::new(
            rate_limit,
            std::time::Duration::from_secs(1),
        ));

    //
    // Extensions
    //
    // GRPC Clients
    let grpc_clients = GrpcClients::default(config.clone());

    let app = Router::new()
        .route("/health", routing::get(api::health_check))
        .route("/assets/operators/:id", routing::get(api::get_operator))
        .route("/assets/demo/aircraft", routing::get(api::get_all_aircraft))
        .route(
            "/assets/demo/vertiports",
            routing::get(api::get_all_vertiports),
        )
        .route(
            "/assets/demo/vertipads",
            routing::get(api::get_all_vertipads),
        )
        .route(
            "/assets/operators/:id/assets",
            routing::get(api::get_all_assets_by_operator),
        )
        .route(
            "/assets/operators/:id/grouped",
            routing::get(api::get_all_grouped_assets),
        )
        .route(
            "/assets/operators/:id/grouped/delegated-to",
            routing::get(api::get_all_grouped_assets_delegated_to),
        )
        .route(
            "/assets/operators/:id/grouped/delegated-from",
            routing::get(api::get_all_grouped_assets_delegated_from),
        )
        .route(
            "/assets/aircraft/:id",
            routing::get(api::get_aircraft_by_id),
        )
        .route(
            "/assets/vertipads/:id",
            routing::get(api::get_vertipad_by_id),
        )
        .route(
            "/assets/vertiports/:id",
            routing::get(api::get_vertiport_by_id),
        )
        .route(
            "/assets/groups/:id",
            routing::get(api::get_asset_group_by_id),
        )
        // POST endpoints
        .route("/assets/aircraft", routing::post(api::register_aircraft))
        .route("/assets/vertiports", routing::post(api::register_vertiport))
        .route("/assets/vertipads", routing::post(api::register_vertipad))
        .route("/assets/groups", routing::post(api::register_asset_group))
        // PUT endpoints
        .route("/assets/aircraft", routing::put(api::update_aircraft))
        .route("/assets/vertiports", routing::put(api::update_vertiport))
        .route("/assets/vertipads", routing::put(api::update_vertipad))
        .route("/assets/groups/:id", routing::put(api::update_asset_group))
        // DELETE endpoints
        .route(
            "/assets/aircraft/:id",
            routing::delete(api::remove_aircraft),
        )
        .route(
            "/assets/vertiports/:id",
            routing::delete(api::remove_vertiport),
        )
        .route(
            "/assets/vertipads/:id",
            routing::delete(api::remove_vertipad),
        )
        .route(
            "/assets/groups/:id",
            routing::delete(api::remove_asset_group),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(cors_allowed_origin)
                .allow_headers(Any)
                .allow_methods(Any),
        )
        .layer(limit_middleware)
        .layer(Extension(grpc_clients)); // Extension layer must be last

    //
    // Bind to address
    //
    match axum::Server::bind(&full_rest_addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal("rest", shutdown_rx))
        .await
    {
        Ok(_) => {
            rest_info!("(rest_server) hosted at: {}.", full_rest_addr);
            Ok(())
        }
        Err(e) => {
            rest_error!("(rest_server) could not start server: {}", e);
            Err(())
        }
    }
}
