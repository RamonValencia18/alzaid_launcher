mod commands;
mod models;
mod server;
mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            tauri::async_runtime::spawn(async {
                server::http::start_local_server().await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::games::start_game_session
        ])
        .run(tauri::generate_context!())
        .expect("Error al ejecutar la aplicación Tauri");
}