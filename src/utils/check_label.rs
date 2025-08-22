use std::process::Command;

pub fn get_addresses_by_label(wallet_name: &str, label: &str) {
    let output = Command::new("bitcoin-cli")
        .args([
            &format!("-rpcwallet={}", wallet_name),
            "getaddressesbylabel",
            label,
        ])
        .output()
        .expect("Falha ao executar bitcoin-cli getaddressesbylabel");

    println!("📬 Endereços associados ao rótulo '{}':", label);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
