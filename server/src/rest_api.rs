//! Implementation of REST APIs.

/// Types Used in REST Messages
pub mod rest_types {
    include!("../../openapi/types.rs");
}
use std::time::SystemTime;

pub use rest_types::*;

use axum::{extract::Path, Extension, Json};
use hyper::StatusCode;
use uuid::Uuid;

use crate::{
    grpc_clients::GrpcClients,
    req_debug,
    structs::Aircraft,
    structs::AssetsInfo,
    structs::Operator,
    structs::Vertipad,
    structs::Vertiport,
    structs::{AssetGroup, Basics},
};

//===========================================================
// Helpers
//===========================================================

/// Check if a string is a valid UUID.
fn is_uuid(s: &str) -> bool {
    uuid::Uuid::try_parse(s).is_ok()
}

//===========================================================
// REST API Implementations
//===========================================================

/// Get info about an operator by id.
#[utoipa::path(
    get,
    path="/operators/{id}",
    responses(
        (status = 200, description = "Operator found in database"),
        (status = 404, description = "Operator not found in database"),
        (status = 400, description = "Invalid operator id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Operator id"),
    )
)]
pub async fn get_operator(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Path(operator_id): Path<String>,
) -> Result<Json<Operator>, (StatusCode, String)> {
    req_debug!("get_operator({})", operator_id);
    if !is_uuid(&operator_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid operator id".to_string()));
    }
    // Get Client
    let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_operator) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    //TODO
    Ok(Json(Operator::random()))
}

//-----------------------------------------------------------
// Get assets by operator
//-----------------------------------------------------------
#[utoipa::path(
    get,
    path="/operators/{id}/assets",
    responses(
        (status = 200, description = "Assets found from database for operator {id}"),
        (status = 404, description = "Operator not found in database"),
        (status = 400, description = "Invalid operator id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Operator id"),
    )
)]
/// Get all assets belonging to an operator.
pub async fn get_all_assets(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Path(operator_id): Path<String>,
) -> Result<Json<Vec<Uuid>>, (StatusCode, String)> {
    req_debug!("get_all_assets({})", operator_id);
    if !is_uuid(&operator_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid operator id".to_string()));
    }
    // Get Client
    let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_all_assets) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    //TODO
    Ok(Json(vec![]))
}

/// Get all grouped assets belonging to an operator.
///
/// These are the assets NOT being delegated to or from this operator.
///
/// Returns a list of grouped asset ids.
#[utoipa::path(
    get,
    path="/operators/{id}/grouped",
    responses(
        (status = 200, description = "Grouped assets found from database for operator {id}"),
        (status = 404, description = "Operator not found in database"),
        (status = 400, description = "Invalid operator id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Operator id"),
    )
)]
pub async fn get_all_grouped_assets(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Path(operator_id): Path<String>,
) -> Result<Json<Vec<Uuid>>, (StatusCode, String)> {
    req_debug!("get_all_grouped_assets({})", operator_id);
    if !is_uuid(&operator_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid operator id".to_string()));
    }
    // Get Client
    let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_all_grouped_assets) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    //TODO
    Ok(Json(vec![]))
}

/// Get all grouped assets delegated to an operator.
#[utoipa::path(
    get,
    path="/operators/{id}/grouped/delegated-to",
    responses(
        (status = 200, description = "Grouped assets delegated to operator {id} found from database"),
        (status = 404, description = "Operator not found in database"),
        (status = 400, description = "Invalid operator id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Operator id"),
    )
)]
pub async fn get_all_grouped_assets_delegated_to(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Path(operator_id): Path<String>,
) -> Result<Json<Vec<Uuid>>, (StatusCode, String)> {
    req_debug!("get_all_grouped_assets_delegated_to({})", operator_id);
    if !is_uuid(&operator_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid operator id".to_string()));
    }
    // Get Client
    let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_all_grouped_assets_delegated_to) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    //TODO
    Ok(Json(vec![]))
}

/// Get all grouped assets delegated from an operator.
#[utoipa::path(
    get,
    path="/operators/{id}/grouped/delegated-from",
    responses(
        (status = 200, description = "Grouped assets delegated from operator {id} found from database"),
        (status = 404, description = "Operator not found in database"),
        (status = 400, description = "Invalid operator id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Operator id"),
    )
)]
pub async fn get_all_grouped_assets_delegated_from(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Path(operator_id): Path<String>,
) -> Result<Json<Vec<Uuid>>, (StatusCode, String)> {
    req_debug!("get_all_grouped_assets_delegated_from({})", operator_id);
    if !is_uuid(&operator_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid operator id".to_string()));
    }
    // Get Client
    let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_all_grouped_assets_delegated_from) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    //TODO
    Ok(Json(vec![]))
}

//-----------------------------------------------------------
// Get assets by asset id
//-----------------------------------------------------------

/// Get an [`Aircraft`] by its id.
#[utoipa::path(
    get,
    path="/aircraft/{id}",
    responses(
        (status = 200, description = "Aircraft {id} found from database"),
        (status = 404, description = "Aircraft not found in database"),
        (status = 400, description = "Invalid aircraft id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Aircraft id"),
    )
)]
pub async fn get_aircraft_by_id(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Path(aircraft_id): Path<String>,
) -> Result<Json<Aircraft>, (StatusCode, String)> {
    req_debug!("get_aircraft_by_id({})", aircraft_id);
    if !is_uuid(&aircraft_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid aircraft id".to_string()));
    }

    // Get Client
    let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_aircraft_by_id) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    //TODO
    Ok(Json(Aircraft::random()))
}

/// Get an [`Vertipad`] by its id.
#[utoipa::path(
    get,
    path="/vertipads/{id}",
    responses(
        (status = 200, description = "Vertipad {id} found from database"),
        (status = 404, description = "Vertipad not found in database"),
        (status = 400, description = "Invalid vertipad id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Vertipad id"),
    )
)]
pub async fn get_vertipad_by_id(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Path(vertipad_id): Path<String>,
) -> Result<Json<Vertipad>, (StatusCode, String)> {
    req_debug!("get_vertipad_by_id({})", vertipad_id);
    if !is_uuid(&vertipad_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid vertipad id".to_string()));
    }

    // Get Client
    let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_vertipad_by_id) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    //TODO
    Ok(Json(Vertipad::random()))
}

/// Get an [`Vertiport`] by its id.
#[utoipa::path(
    get,
    path="/vertiports/{id}",
    responses(
        (status = 200, description = "Vertiport {id} found from database"),
        (status = 404, description = "Vertiport not found in database"),
        (status = 400, description = "Invalid vertiport id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Vertiport id"),
    )
)]
pub async fn get_vertiport_by_id(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Path(vertiport_id): Path<String>,
) -> Result<Json<Vertiport>, (StatusCode, String)> {
    req_debug!("get_vertiport_by_id({})", vertiport_id);
    if !is_uuid(&vertiport_id) {
        return Err((StatusCode::BAD_REQUEST, "Invalid vertiport id".to_string()));
    }
    // Get Client
    let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_vertiport_by_id) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();
    //TODO
    Ok(Json(Vertiport::random()))
}

/// Get an [`AssetGroup`] by its id.
#[utoipa::path(
    get,
    path="/assets/groups/{id}",
    responses(
        (status = 200, description = "Asset group {id} found from database"),
        (status = 404, description = "Asset group not found in database"),
        (status = 400, description = "Invalid asset group id"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    ),
    params(
        ("id" = String, Path, description = "Asset group id"),
    )
)]
pub async fn get_asset_group_by_id(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Path(asset_group_id): Path<String>,
) -> Result<Json<AssetGroup>, (StatusCode, String)> {
    req_debug!("get_asset_group_by_id({})", asset_group_id);
    if !is_uuid(&asset_group_id) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid asset group id".to_string(),
        ));
    }
    // Get Client
    let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_asset_group_by_id) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    //TODO
    Ok(Json(AssetGroup::random()))
}

//-----------------------------------------------------------
// Register assets
//-----------------------------------------------------------

/// Register an [`Aircraft`] in the database.
#[utoipa::path(
    post,
    path="/aircraft",
    request_body=RegisterAircraftPayload,
    responses(
        (status = 200, description = "Aircraft registered in database"),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn register_aircraft(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<RegisterAircraftPayload>,
) -> Result<String, (StatusCode, String)> {
    req_debug!("register_aircraft()");

    // validate payload
    // to check with the database to validate the registration number
    //
    // if !payload.is_valid() {
    //     return Err((
    // StatusCode::BAD_REQUEST,
    //         "Invalid payload format".to_string(),
    //     ));
    // }

    let _aircraft = Aircraft {
        basics: Basics {
            id: Uuid::new_v4(),
            name: payload.name,
            group_id: payload.group_id,
            owner: payload.owner,
            whitelist: payload.whitelist,
            created_at: SystemTime::now(),
            updated_at: None,
            status: payload.status,
        },
        model: payload.model,
        manufacturer: payload.manufacturer,
        serial_number: payload.serial_number,
        registration_number: payload.registration_number,
        description: payload.description,
        max_payload_kg: payload.max_payload_kg,
        max_range_km: payload.max_range_km,
    };

    // Get Client
    let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_asset_group_by_id) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    //TODO
    Ok(_aircraft.id().to_string())
}

/// Register an [`Vertiport`] in the database.
#[utoipa::path(
    post,
    path="/vertiports",
    request_body=RegisterVertiportPayload,
    responses(
        (status = 200, description = "Vertiport registered in database"),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn register_vertiport(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<RegisterVertiportPayload>,
) -> Result<String, (StatusCode, String)> {
    req_debug!("register_vertiport()");

    // validate payload
    // to check with the database to validate the registration number
    //
    // if !payload.is_valid() {
    //     return Err((
    // StatusCode::BAD_REQUEST,
    //         "Invalid payload format".to_string(),
    //     ));
    // }

    let _vertiport = Vertiport {
        basics: Basics {
            id: Uuid::new_v4(),
            name: payload.name,
            group_id: payload.group_id,
            owner: payload.owner,
            whitelist: payload.whitelist,
            created_at: SystemTime::now(),
            updated_at: None,
            status: payload.status,
        },
        location: payload.location,
        description: payload.description,
        vertipads: payload.vertipads,
    };

    // Get Client
    let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_asset_group_by_id) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    //TODO
    Ok(_vertiport.id().to_string())
}

/// Register an [`Vertipad`] in the database.
///
/// Also inserts the vertipad into the vertiport's vertipad list.
#[utoipa::path(
    post,
    path="/vertipads",
    request_body=RegisterVertipadPayload,
    responses(
        (status = 200, description = "Vertipad registered in database"),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn register_vertipad(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<RegisterVertipadPayload>,
) -> Result<String, (StatusCode, String)> {
    req_debug!("register_vertipad()");

    // validate payload
    // to check with the database to validate the registration number
    //
    // if !payload.is_valid() {
    //     return Err((
    // StatusCode::BAD_REQUEST,
    //         "Invalid payload format".to_string(),
    //     ));
    // }

    let _vertipad = Vertipad {
        id: Uuid::new_v4(),
        location: payload.location,
        status: payload.status,
        vertiport_id: payload.vertiport_id,
    };

    // Get Client
    let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_asset_group_by_id) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    //TODO
    Ok(_vertipad.id.to_string())
}

//-----------------------------------------------------------
// Group management
//-----------------------------------------------------------

/// Register an [`AssetGroup`] in the database.
#[utoipa::path(
    post,
    path="/assets/groups",
    request_body=RegisterAssetGroupPayload,
    responses(
        (status = 200, description = "AssetGroup registered in database"),
        (status = 422, description = "Request body is invalid format"),
        (status = 503, description = "Could not connect to other microservice dependencies")
    )
)]
pub async fn register_asset_group(
    Extension(mut grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<RegisterAssetGroupPayload>,
) -> Result<String, (StatusCode, String)> {
    req_debug!("register_asset_group()");

    // validate payload
    // to check with the database to validate the registration number
    //
    // if !payload.is_valid() {
    //     return Err((
    // StatusCode::BAD_REQUEST,
    //         "Invalid payload format".to_string(),
    //     ));
    // }

    let _asset_group = AssetGroup {
        id: Uuid::new_v4(),
        name: payload.name,
        owner: payload.owner,
        created_at: SystemTime::now(),
        updated_at: None,
        delegatee: None,
        assets: payload.assets,
    };

    // Get Client
    let _client_option = grpc_clients.storage.get_client().await;
    // if client_option.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(get_asset_group_by_id) {}", &error_msg);
    //     return Err((StatusCode::SERVICE_UNAVAILABLE, error_msg));
    // }
    // let mut client = client_option.unwrap();

    //TODO
    Ok(_asset_group.id.to_string())
}
