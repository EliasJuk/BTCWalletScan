use colored::*;
use std::process::Command;

fn main() {
  let output = Command::new("cmd")
    .args(&["/C", "echo Hello from CMD!"])
    .output()
    .expect("falha ao executar comando");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{} {}","Saída do CMD:".bold().yellow(), stdout);
}