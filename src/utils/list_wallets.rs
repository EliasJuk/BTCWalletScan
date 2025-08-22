use colored::*;
use crate::utils::command::run_command;

/// Lista somente as carteiras **carregadas na memória do Bitcoin Core**
pub fn list_loaded_wallets() {
  println!("{}", "\n... Carteiras carregadas no momento:\n".bold().green());

  let (stdout, stderr) = run_command("bitcoin-cli listwallets");

  if !stderr.trim().is_empty() {
    println!("Erro: {}", stderr.red());
    return;
  }

  println!("Loaded wallets: {}", stdout);
}

/// Lista todas as carteiras **existentes no diretório do Bitcoin Core**
pub fn list_all_wallets() {
  println!("{}", "\n... Todas as carteiras no disco:\n".bold().cyan());

  let (stdout, stderr) = run_command("bitcoin-cli listwalletdir");

  if !stderr.trim().is_empty() {
    println!("Erro: {}", stderr.red());
    return;
  }

  println!("Wallets no disco: {}", stdout);
}