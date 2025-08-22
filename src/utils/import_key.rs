use std::process::Command;

pub fn import_privkey(wallet_name: &str, privkey: &str, label: &str) {
  let output = Command::new("bitcoin-cli")
    .args([
      &format!("-rpcwallet={}", wallet_name),
      "importprivkey",
      privkey,
      label,
      "false",
    ])
    .output()
    .expect("Falha ao executar bitcoin-cli importprivkey");

    println!("🔐 Importando chave privada:");
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}