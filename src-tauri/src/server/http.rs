use crate::models::payload::{SessionPayload, StatusResponse};
use crate::services::launcher::launch_python_game;
use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};

pub async fn start_local_server() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let current_dir = env::current_dir().unwrap_or_default();
    let mut exe_dir = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    exe_dir.pop();

    let dist_path = if current_dir.join("dist").exists() {
        current_dir.join("dist")
    } else if current_dir.join("../dist").exists() {
        current_dir.join("../dist")
    } else if exe_dir.join("dist").exists() {
        exe_dir.join("dist")
    } else {
        current_dir.join("dist")
    };

    println!(
        "📂 [Axum] Buscando archivos estáticos en: {:?}",
        dist_path.canonicalize().unwrap_or(dist_path.clone())
    );

    let index_file = dist_path.join("index.html");
    let serve_dir = ServeDir::new(&dist_path).not_found_service(ServeFile::new(&index_file));

    let app = Router::new()
        .route("/status", get(handle_status))
        .route("/session/start", post(handle_start_session))
        .fallback_service(serve_dir)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🌐 [Axum] Servidor activo para la red local en http://{}", addr);

    if let Ok(listener) = tokio::net::TcpListener::bind(addr).await {
        let _ = axum::serve(listener, app).await;
    } else {
        eprintln!("❌ Error al iniciar el servidor en el puerto 8080");
    }
}

async fn handle_status() -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "OK".into(),
        message: "Mesa Interactiva Server Online".into(),
    })
}

async fn handle_start_session(Json(payload): Json<SessionPayload>) -> Json<StatusResponse> {
    println!("📡 Sesión notificada desde la API móvil/web: {:?}", payload);

    let patient_ref = payload.patient_id.as_deref();

    match launch_python_game(patient_ref, &payload.game_id) {
        Ok(_) => {
            let patient_name = payload.patient_id.unwrap_or_else(|| "Paciente_Anonimo".into());
            Json(StatusResponse {
                status: "SUCCESS".into(),
                message: format!("Juego '{}' iniciado correctamente para {}", payload.game_id, patient_name),
            })
        }
        Err(err) => Json(StatusResponse {
            status: "ERROR".into(),
            message: err,
        }),
    }
}