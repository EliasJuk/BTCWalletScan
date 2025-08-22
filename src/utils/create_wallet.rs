use std::process::Command;

pub fn create_wallet(wallet_name: &str) {
    // 🪙 Criar carteira usando bitcoin-cli
    let create_wallet_output = Command::new("bitcoin-cli")
        .args([
            "createwallet",
            wallet_name,
            "false", // disable_private_keys
            "false", // blank
            "",      // passphrase
            "false", // avoid_reuse
            "false", // descriptors
            "false", // load_on_startup
        ])
        .output()
        .expect("Falha ao executar bitcoin-cli createwallet");

    println!("🪙 Criando carteira '{}':", wallet_name);
    println!("stdout: {}", String::from_utf8_lossy(&create_wallet_output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&create_wallet_output.stderr));
}