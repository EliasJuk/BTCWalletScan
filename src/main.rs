use crate::utils::{*};

mod utils {
  pub mod msg;
  pub mod create_wallet;
  pub mod import_key;
  pub mod check_label;
  pub mod getallinfo;
  pub mod check_balance;
}

fn main() {
  let wallet_name = "wallet_0044";
  let privkey = "L21AkScUt7anosjfeg6i2oqajvLmAftDVf6jwNjiJ6GpAcynMNEG";
  let label = "chave_teste";

  // AVISO!
  msg::msg();

  // SCRIPT!
  create_wallet::create_wallet(wallet_name);
  import_key::import_privkey(wallet_name, privkey, label);
  check_label::get_addresses_by_label(wallet_name, label);
  getallinfo::get_wallet_info(wallet_name);
  check_balance::check_balance(wallet_name);
}