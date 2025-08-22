use std::process::Command;

pub fn create_wallet(wallet_name: &str) {
  let output = Command::new("cmd")
    .args(&[
      "/C", &format!("bitcoin-cli createwallet {} false false \"\" false false false", wallet_name),
    ])
    .output()
    .expect("falha ao executar comando");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
    println!("Resultado da criação da wallet:\n{}", stdout);
}