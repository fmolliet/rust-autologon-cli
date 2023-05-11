use winreg::enums::*;
use winreg::RegKey;

pub fn autologon_create_registry(user: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Abre ou cria a chave de registro
    let key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .create_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon")?;

    // Define o valor de AutoAdminLogon como 1
    let ativated:u32 = 1;
    key.0.set_value("AutoAdminLogon", &ativated)?;

    // Define o valor de DefaultUserName como "nome de usuário"
    key.0.set_value("DefaultUserName", &user)?;

    // Define o valor de DefaultPassword como "senha"
    key.0.set_value("DefaultPassword", &password)?;

    Ok(())
}