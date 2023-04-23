use axum::{routing::get, Router, Server, extract::State, response::IntoResponse, Json};
use std::sync::{Mutex, Arc};
use sysinfo::{CpuExt, System, SystemExt};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(healthz))
        .route("/api/cpu", get(cpus_get))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new())),
        });
    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    server.await.unwrap();

    println!("Listening on address: {}", addr);
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

async fn healthz() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}

#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {

    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();

    let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    Json(v)
}
