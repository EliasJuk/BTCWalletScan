use crate::utils::command::run_command;
use colored::*;

/// Importa uma chave privada para uma wallet específica
pub fn import_privkey(wallet_name: &str, privkey: &str, label: &str) {
    println!("{}", format!("\n... Importando chave para a wallet '{}'", wallet_name).bold().green());

    let cmd = format!(
        "bitcoin-cli -rpcwallet=\"{}\" importprivkey \"{}\" \"{}\" false",
        wallet_name, privkey, label
    );

    let (stdout, stderr) = run_command(&cmd);

    if !stderr.trim().is_empty() {
        println!("Erro: {}", stderr.red());
    } else {
        println!("Chave importada com sucesso! Resultado:\n{}", stdout);
    }
}