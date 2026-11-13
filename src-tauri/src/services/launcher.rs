use std::process::{Child, Command};

pub fn launch_python_game(patient_id: Option<&str>, _game_id: &str) -> Result<Child, String> {
    let patient_name = patient_id.unwrap_or("Paciente_Anonimo");
    let script_path = r".\minijuegos\Drag Objects\drag_objects.py";

    Command::new("python")
        .arg(script_path)
        .arg(patient_name)
        .spawn()
        .map_err(|err| format!("Error al lanzar Python: {}", err))
}