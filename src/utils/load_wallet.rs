use colored::*;
use crate::utils::command::run_command;

pub fn loadwallet(wallet_name: &str) {
  // Tentar carregar wallet explicitamente
  let cmd_load = format!("bitcoin-cli loadwallet {}", wallet_name);
  let (_stdout, stderr) = run_command(&cmd_load);
  if !stderr.trim().is_empty() && !stderr.contains("already loaded") {
      println!("Erro ao carregar wallet: {}", stderr.red());
      return;
  }
}