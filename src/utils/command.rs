use std::process::Command;

/// Executa um comando no shell e retorna (stdout, stderr)
pub fn run_command(cmd: &str) -> (String, String) {
    let output = Command::new("cmd")
        .args(&["/C", cmd])
        .output()
        .expect("falha ao executar comando");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    (stdout, stderr)
}