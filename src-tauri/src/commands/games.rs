use crate::services::launcher::launch_python_game;

#[tauri::command]
pub fn start_game_session(
    patient_id: Option<String>,
    patient_name: Option<String>,
    game_id: String,
    _notes: String,
) -> Result<String, String> {
    
    let display_name = patient_name
        .or(patient_id)
        .unwrap_or_else(|| "Paciente_Anonimo".into());

    match launch_python_game(Some(&display_name), &game_id) {
        Ok(_) => Ok(format!("Juego '{}' iniciado para {}", game_id, display_name)),
        Err(err) => Err(err),
    }
}