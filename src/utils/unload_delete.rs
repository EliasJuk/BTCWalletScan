use std::process::Command;
use std::fs;
use std::path::Path;

/// Descarrega a carteira do Bitcoin Core e tenta deletar sua pasta local
pub fn unload_and_delete_wallet(wallet_name: &str) {
    println!("📤 Descarregando carteira '{}'", wallet_name);

    let unload_output = Command::new("bitcoin-cli")
        .args(["unloadwallet", wallet_name])
        .output()
        .expect("Falha ao executar bitcoin-cli unloadwallet");

    println!("stdout: {}", String::from_utf8_lossy(&unload_output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&unload_output.stderr));

    // Caminho da carteira no sistema
    let wallet_path = format!(
        "C:\\Users\\anonymous\\AppData\\Roaming\\Bitcoin\\wallets\\{}",
        wallet_name
    );

    if Path::new(&wallet_path).exists() {
        match fs::remove_dir_all(&wallet_path) {
            Ok(_) => println!("🗑️ Carteira '{}' deletada com sucesso!", wallet_name),
            Err(e) => eprintln!("❌ Erro ao deletar a carteira: {}", e),
        }
    } else {
        println!("⚠️ Caminho da carteira não encontrado: {}", wallet_path);
    }
}