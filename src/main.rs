use clap::Parser;

use crate::services::registry::autologon_create_registry;

mod services;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    login: String,
    #[arg(short, long)]
    senha: String,
}

/*
HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon
AutoAdminLogon = 1
DefaultUserName = "nome de usuário"
DefaultPassword = "senha"

 */
fn main() {
    let args = Args::parse();

    for _ in 0..1 {
        println!("Hello {}!", args.login);
        
        let user = args.login.as_str();
        let password = args.senha.as_str();
        
        match autologon_create_registry(user, password)  {
            Ok(()) => println!("Gravado nos registros com sucesso!"),
            Err(err) => println!("Erro = {}", err.to_string())
        };
    }
}