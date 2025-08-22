use std::{thread, time::Duration};
use crate::utils::{*};

mod utils {
  pub mod command;
  pub mod create_legacy_wallet;
  pub mod list_wallets;
  pub mod import_wallet;
  pub mod msg;
  pub mod load_wallet;
}

fn main() {
    let wallet_name = "wallet_00014";
    let privkey = "L21AkScUt7anosjfeg6i2oqajvLmAftDVf6jwNjiJ6GpAcynMNEG";
    let label = "chave_teste";

  // MSG
  msg::msg();

  // Criar carteira Legacy
  create_legacy_wallet::create_wallet(wallet_name);

  // Pequeno delay para o node estabilizar
  thread::sleep(Duration::from_secs(2));

  // Listar wallets carregadas e todas no disco
  list_wallets::list_loaded_wallets();
  list_wallets::list_all_wallets();

  // Carregar Wallet
  load_wallet::loadwallet(wallet_name);

  // Pequeno delay para o node estabilizar
  thread::sleep(Duration::from_secs(2));

  // Importar Chave Privada
  import_wallet::import_privkey(wallet_name, privkey, label);
}