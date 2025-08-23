use crate::utils::{*};
use std::fs::OpenOptions;
use std::io::Write;

mod utils {
	pub mod msg;
	pub mod create_wallet;
	pub mod import_key;
	pub mod check_label;
	//pub mod getallinfo;
	pub mod check_balance;
	pub mod random;
	pub mod ranges;
	pub mod unload_delete;
	pub mod wif_encoder;
}

fn main() {
	let wallet_name_base = "wallet_name_";
	let label_base = "label_";
	let range = "70"; // ✅ Defina o range fixo aqui
	let total_loops = 10000; // ✅ Quantas carteiras você quer testar

	msg::msg();

	for i in 0..total_loops {
		let wallet_name = format!("{}{:02}", wallet_name_base, i);
		let label = format!("{}{:02}", label_base, i);

		let privkey = match random::gerar_hex_por_range(range) {
			Some(hex) => {
				println!("🔑 Chave HEX gerada: {}", hex);
				hex
			}
			None => {
				println!("❌ Range inválido: {}", range);
				continue;
			}
		};

		let padded_key = format!("{:0>64}", privkey);

		let wif = match wif_encoder::key_to_wif(&padded_key) {
			Ok(wif) => wif,
			Err(e) => {
				println!("❌ Erro ao converter para WIF: {}", e);
				continue;
			}
		};

		println!("📨 WIF: {}", &wif);


		create_wallet::create_wallet(&wallet_name);
		import_key::import_privkey(&wallet_name, &wif, &label);
		//check_label::get_addresses_by_label(&wallet_name, &label);
		//getallinfo::get_wallet_info(&wallet_name);

		let saldo = check_balance::check_balance(&wallet_name);
		println!("💰 Saldo: {}", saldo);

		if saldo > 0.00000000 {
			let mut file = OpenOptions::new()
				.create(true)
				.append(true)
				.open("carteiras_com_saldo.txt")
				.expect("Erro ao abrir/criar o arquivo");

			writeln!(file, "Saldo: {}, WIF: {}", saldo, wif)
				.expect("Erro ao escrever no arquivo");
		}

		unload_delete::unload_and_delete_wallet(&wallet_name);
	}

	println!("✅ Scanner finalizado com {} tentativas.", total_loops);
}