use crate::services::launcher::launch_python_game;

#[tauri::command]
pub fn start_game_session(
    patient_id: Option<String>,
    game_id: String,
    _notes: String,
) -> Result<String, String> {
    let patient_ref = patient_id.as_deref();

    match launch_python_game(patient_ref, &game_id) {
        Ok(_) => {
            let patient_name = patient_id.unwrap_or_else(|| "Paciente_Anonimo".into());
            Ok(format!("Juego '{}' iniciado para {}", game_id, patient_name))
        }
        Err(err) => Err(err),
    }
}