use axum::{routing::get, Router, extract::{State, Json}};
use serde::{Deserialize, Serialize};
use tower_service::Service;
use worker::*;


#[derive(Clone)]
struct AppState {
    locale: &'static str,
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/test", get(test))
        .with_state(state)
}

#[event(fetch)]
async fn main(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {

    console_error_panic_hook::set_once();
    const LOCALE: &str = "en";
    let state = AppState { locale: LOCALE};

    Ok(router(state).call(req).await?)


}

pub async fn root(State(state): State<AppState>) -> &'static str {
    state.locale
}

#[derive(Serialize, Deserialize, Debug)]
struct TestResponse {
    locale: &'static str,
}

pub async fn test(State(state): State<AppState>) -> Json<TestResponse> {
    let response = TestResponse {
        locale: state.locale
    };

    Json(response)
}
