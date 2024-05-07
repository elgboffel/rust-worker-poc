use axum::{routing::get, Router, extract::{State, Json}};
use serde::{Deserialize, Serialize};
use tower_service::Service;
use worker::*;
use common::add;
use axum::http::Response;
use axum::body::Body;
use utoipa::{OpenApi, ToSchema};
use utoipauto::utoipauto;

#[utoipauto(paths = "./apps/worker-rust/src")]
#[derive(OpenApi)]
#[openapi(paths(openapi))]
pub struct ApiDoc;

#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[derive(Clone)]
struct AppState {
    locale: &'static str,
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/api-docs/openapi.json", get(openapi))
        .route("/", get(root))
        .route("/test", get(test))
        .with_state(state)
}

#[event(fetch)]
async fn main(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<Response<Body>> {

    console_error_panic_hook::set_once();

    const LOCALE: &str = "en";
    let state = AppState { locale: LOCALE};

    let mut response = router(state).call(req).await?;

    Ok(response)
}

pub async fn root(State(_state): State<AppState>) -> String {
    "test".to_string()
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct TestResponse {
    locale: String,
}

#[utoipa::path(
    get,
    path = "/test",
    responses(
        (status = 200, description = "Test response", body = TestResponse)
    )
)]
pub async fn test(State(state): State<AppState>) -> Json<TestResponse> {
    let value = TestResponse {
        locale: format!("{}, {}", state.locale, add(1, 3).to_string())
    };

    Json(value)
}
