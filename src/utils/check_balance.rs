use std::process::Command;

pub fn check_balance(wallet_name: &str) -> f64 {
    let output = Command::new("bitcoin-cli")
        .args([
            &format!("-rpcwallet={}", wallet_name),
            "getbalance",
        ])
        .output()
        .expect("Falha ao executar bitcoin-cli getbalance");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("💰 Saldo da carteira '{}':", wallet_name);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // Tenta converter o stdout para f64
    stdout.trim().parse::<f64>().unwrap_or(0.0)
}