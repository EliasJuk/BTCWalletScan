use colored::*;

pub fn msg() {
    println!("{}", "Certifique-se que o Bitcoin Core está sincronizado e finalizado".bold().yellow());
    println!("{}", "Use o comando: bitcoind -deprecatedrpc=create_bdb".bold().green());
    println!("{}", "...".bold().green());
}