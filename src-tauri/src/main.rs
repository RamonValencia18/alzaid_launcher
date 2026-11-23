#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    app_lib::run();
    if let Err(e) = dotenvy::dotenv() {
        println!("⚠️ [Env] No se encontró el archivo .env o no se pudo cargar: {}", e);
    } else {
        println!("✅ [Env] Variables de entorno cargadas correctamente.");
    }
}