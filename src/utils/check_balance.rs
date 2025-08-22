use std::process::Command;

pub fn check_balance(wallet_name: &str) {
  let output = Command::new("bitcoin-cli")
    .args([
      &format!("-rpcwallet={}", wallet_name),
      "getbalance",
    ])
    .output()
    .expect("Falha ao executar bitcoin-cli getbalance");

    println!("💰 Saldo da carteira '{}':", wallet_name);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
