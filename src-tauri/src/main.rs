use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Command;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Serialize, Deserialize, Debug)]
struct SessionPayload {
    patient_id: Option<String>,
    game_id: String,
    notes: Option<String>,
}

#[derive(Serialize)]
struct StatusResponse {
    status: String,
    message: String,
}

async fn start_local_server() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 🟢 1. Localización robusta de la carpeta 'dist'
    let current_dir = env::current_dir().unwrap_or_default();
    let mut exe_dir = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    exe_dir.pop(); // Quita el nombre del .exe

    // Probar las posibles ubicaciones reales de 'dist'
    let dist_path = if current_dir.join("dist").exists() {
        // Ejecución desde la raíz (alzaid_launcher/dist)
        current_dir.join("dist")
    } else if current_dir.join("../dist").exists() {
        // Ejecución desde src-tauri (alzaid_launcher/src-tauri/../dist)
        current_dir.join("../dist")
    } else if exe_dir.join("dist").exists() {
        // Ejecutable de producción desplegado junto a carpeta dist
        exe_dir.join("dist")
    } else {
        // Fallback por defecto a nivel raíz
        current_dir.join("dist")
    };

    println!("📂 [Axum] Buscando archivos estáticos en: {:?}", dist_path.canonicalize().unwrap_or(dist_path.clone()));

    let index_file = dist_path.join("index.html");

    // 🟢 2. Servir estáticos con fallback SPA/Astro Routing
    let serve_dir = ServeDir::new(&dist_path)
        .not_found_service(ServeFile::new(&index_file));

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

    let patient_name = payload.patient_id.unwrap_or_else(|| "Paciente_Anonimo".into());
    let script_path = r".\minijuegos\Drag Objects\drag_objects.py";

    // Ejecutamos el script de Python en el servidor (la PC principal)
    let child = Command::new("python")
        .arg(script_path)
        .arg(&patient_name)
        .spawn();

    match child {
        Ok(_) => Json(StatusResponse {
            status: "SUCCESS".into(),
            message: format!("Juego '{}' iniciado correctamente para {}", payload.game_id, patient_name),
        }),
        Err(err) => Json(StatusResponse {
            status: "ERROR".into(),
            message: format!("Error al lanzar Python: {}", err),
        }),
    }
}

#[tauri::command]
fn start_game_session(
    patient_id: Option<String>,
    game_id: String,
    _notes: String,
) -> Result<String, String> {
    let patient_name = patient_id.unwrap_or_else(|| "Paciente_Anonimo".into());
    let script_path = r".\minijuegos\Drag Objects\drag_objects.py";

    let child = Command::new("python")
        .arg(script_path)
        .arg(&patient_name)
        .spawn();

    match child {
        Ok(_) => Ok(format!("Juego '{}' iniciado para {}", game_id, patient_name)),
        Err(err) => Err(format!("Error: {}", err)),
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            tauri::async_runtime::spawn(async {
                start_local_server().await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![start_game_session])
        .run(tauri::generate_context!())
        .expect("Error al ejecutar Tauri");
}