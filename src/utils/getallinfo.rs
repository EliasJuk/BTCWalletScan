use std::process::Command;

pub fn get_wallet_info(wallet_name: &str) {
  let output = Command::new("bitcoin-cli")
    .args([
      &format!("-rpcwallet={}", wallet_name),
      "getwalletinfo",
    ])
    .output()
    .expect("Falha ao executar bitcoin-cli getwalletinfo");

    println!("🧾 Informações da carteira '{}':", wallet_name);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}