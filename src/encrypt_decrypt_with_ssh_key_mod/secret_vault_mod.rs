// secret_vault_mod.rs

//! Secret vault can store many secret tokens.
//!
//! With one SSH private key, we can store many secret tokens.

use secrecy::{SecretBox, SecretString};

use super::encrypt_decrypt_mod as ende;
use crate::{BLUE, GREEN, RED, RESET, YELLOW};

/// List token names from vault.
///
/// No need to decrypt the tokens here.
pub(crate) fn list_tokens_from_vault() -> anyhow::Result<Vec<String>> {
    let mut ret_vec_string = vec![];
    println!("  {YELLOW}Check if the encrypted file exists.{RESET}");
    let private_key_file_name = crate::TREASURE_CONFIG.get().unwrap().treasure_private_key_file_name.to_string();

    let encrypted_path_struct = ende::PathStructInSshFolder::new(format!("{private_key_file_name}.enc"))?;
    if !encrypted_path_struct.exists() {
        println!("  {YELLOW}Encrypted file {encrypted_path_struct} does not exist.{RESET}");
        println!("  {YELLOW}Create the vault and store a secret using the store command. {RESET}");
        anyhow::bail!("Encrypted file not found.");
    }
    println!("  {YELLOW}Open and read the encrypted file.{RESET}");
    let encrypted_text_with_metadata: String = ende::open_file_b64_get_string(encrypted_path_struct.get_full_file_path())?;
    // parse json
    let vec_encrypted_text_with_metadata: Vec<ende::EncryptedTextWithMetadata> = serde_json::from_str(&encrypted_text_with_metadata)?;
    println!("  {YELLOW}Decrypt the file with ssh-agent or private key.{RESET}");
    for encrypted_text_with_metadata in vec_encrypted_text_with_metadata.iter() {
        let iter_token_name = if let Some(iter_token_name) = encrypted_text_with_metadata.token_name.as_ref() {
            iter_token_name
        } else {
            continue;
        };
        // here we have the token_name
        ret_vec_string.push(iter_token_name.to_owned());
    }
    Ok(ret_vec_string)
}

/// Get secret token from vault.
///
/// If exists, decrypt it from file.  
pub(crate) fn show_secret_token_from_vault(token_name: &str) -> anyhow::Result<SecretString> {
    println!("  {YELLOW}Check if the ssh private key exists.{RESET}");
    let private_key_file_name = crate::TREASURE_CONFIG.get().unwrap().treasure_private_key_file_name.to_string();

    println!("  {YELLOW}Check if the encrypted file exists.{RESET}");
    let encrypted_path_struct = ende::PathStructInSshFolder::new(format!("{private_key_file_name}.enc"))?;
    if !encrypted_path_struct.exists() {
        println!("  {YELLOW}Encrypted file {encrypted_path_struct} does not exist.{RESET}");
        println!("  {YELLOW}Create the vault and store a secret using the store command. {RESET}");
        anyhow::bail!("Encrypted file not found.");
    }

    println!("  {YELLOW}Open and read the encrypted file.{RESET}");
    let encrypted_text_with_metadata: String = ende::open_file_b64_get_string(encrypted_path_struct.get_full_file_path())?;
    // parse json
    let vec_encrypted_text_with_metadata: Vec<ende::EncryptedTextWithMetadata> = serde_json::from_str(&encrypted_text_with_metadata)?;
    println!("  {YELLOW}Decrypt the file with ssh-agent or private key.{RESET}");
    for encrypted_text_with_metadata in vec_encrypted_text_with_metadata.iter() {
        let iter_token_name = if let Some(iter_token_name) = encrypted_text_with_metadata.token_name.as_ref() {
            iter_token_name
        } else {
            continue;
        };
        if iter_token_name != token_name {
            continue;
        }
        // here we are sure that this is the equal token_name
        let plain_seed_bytes_32bytes = ende::decode64_from_string_to_32bytes(&encrypted_text_with_metadata.plain_seed_string)?;
        // use the private key from the encrypted file
        let private_key_path_struct = ende::PathStructInSshFolder::new(encrypted_text_with_metadata.private_key_file_name.clone())?;
        if !private_key_path_struct.exists() {
            eprintln!("{RED}Error: Private key {private_key_path_struct} does not exist. Cannot decrypt.{RESET}");
            anyhow::bail!("Private key file not found.");
        }

        let secret_passcode_32bytes: SecretBox<[u8; 32]> =
            ende::sign_seed_with_ssh_agent_or_private_key_file(&private_key_path_struct, plain_seed_bytes_32bytes)?;

        // decrypt the secret access token string
        let secret_access_token: SecretString =
            ende::decrypt_symmetric(secret_passcode_32bytes, encrypted_text_with_metadata.plain_encrypted_text.clone())?;
        return Ok(secret_access_token);
    }
    anyhow::bail!("Token with this name not found.");
}

/// Store secret token to vault.
///
/// If exists, decrypt it from file.  
pub(crate) fn store_secret_token_to_vault(token_name: &str) -> anyhow::Result<()> {
    let mut vec_encrypted_text_with_metadata: Vec<ende::EncryptedTextWithMetadata> = vec![];
    println!("  {YELLOW}Check if the ssh private key exists.{RESET}");
    let private_key_file_name = crate::TREASURE_CONFIG.get().unwrap().treasure_private_key_file_name.to_string();

    let private_key_path_struct = ende::PathStructInSshFolder::new(private_key_file_name.clone())?;
    if !private_key_path_struct.exists() {
        eprintln!("{RED}Error: Private key {private_key_path_struct} does not exist.{RESET}");
        println!("  {YELLOW}Create the private key in bash terminal:{RESET}");
        println!(r#"{GREEN}ssh-keygen -t ed25519 -f "{private_key_path_struct}" -C "vault for secret tokens"{RESET}"#);
        anyhow::bail!("Private key file not found.");
    }

    println!("  {YELLOW}Check if the encrypted file exists.{RESET}");
    let encrypted_path_struct = ende::PathStructInSshFolder::new(format!("{private_key_file_name}.enc"))?;
    if encrypted_path_struct.exists() {
        println!("  {YELLOW}Open and read the encrypted file.{RESET}");
        let encrypted_text_with_metadata: String = ende::open_file_b64_get_string(encrypted_path_struct.get_full_file_path())?;
        // parse json
        vec_encrypted_text_with_metadata = serde_json::from_str(&encrypted_text_with_metadata)?;
    }
    println!();
    println!("{BLUE}Enter the secret token to encrypt:{RESET}");
    let secret_access_token = secrecy::SecretString::from(
        inquire::Password::new("")
            .without_confirmation()
            .with_display_mode(inquire::PasswordDisplayMode::Masked)
            .prompt()?,
    );

    // prepare the random bytes, sign it with the private key, that is the true passcode used to encrypt the secret
    let (plain_seed_bytes_32bytes, plain_seed_string) = ende::random_seed_32bytes_and_string()?;
    // first try to use the private key from ssh-agent, else use the private file with user interaction
    let secret_passcode_32bytes: SecretBox<[u8; 32]> =
        ende::sign_seed_with_ssh_agent_or_private_key_file(&private_key_path_struct, plain_seed_bytes_32bytes)?;
    let plain_encrypted_text = ende::encrypt_symmetric(secret_passcode_32bytes, secret_access_token)?;

    // delete the token before writing it with the same token_name
    if !vec_encrypted_text_with_metadata.is_empty() && delete_token_from_vault(token_name).is_ok() {
        let encrypted_text_with_metadata: String = ende::open_file_b64_get_string(encrypted_path_struct.get_full_file_path())?;
        // parse json
        vec_encrypted_text_with_metadata = serde_json::from_str(&encrypted_text_with_metadata)?;
    }

    // prepare a struct to save as encoded string
    let encrypted_text_with_metadata = ende::EncryptedTextWithMetadata {
        private_key_file_name: private_key_path_struct.get_file_name().to_string(),
        plain_seed_string,
        plain_encrypted_text,
        access_token_expiration: None,
        refresh_token_expiration: None,
        token_name: Some(token_name.to_owned()),
    };
    vec_encrypted_text_with_metadata.push(encrypted_text_with_metadata);

    let file_text = serde_json::to_string_pretty(&vec_encrypted_text_with_metadata)?;
    // encode it just to obscure it a little bit
    let file_text = ende::encode64_from_string_to_string(&file_text);

    std::fs::write(encrypted_path_struct.get_full_file_path(), file_text)?;
    println!("  {YELLOW}Encrypted text saved to file.{RESET}");
    Ok(())
}

/// Delete secret token from vault.
///
/// If exists, delete it from file.  
pub(crate) fn delete_token_from_vault(token_name: &str) -> anyhow::Result<()> {
    println!("  {YELLOW}Check if the ssh private key exists.{RESET}");
    let private_key_file_name = crate::TREASURE_CONFIG.get().unwrap().treasure_private_key_file_name.to_string();

    let private_key_path_struct = ende::PathStructInSshFolder::new(private_key_file_name.clone())?;
    if !private_key_path_struct.exists() {
        eprintln!("{RED}Error: Private key {private_key_path_struct} does not exist.{RESET}");
        println!("  {YELLOW}Create the private key in bash terminal:{RESET}");
        println!(r#"{GREEN}ssh-keygen -t ed25519 -f "{private_key_path_struct}" -C "vault for secret tokens"{RESET}"#);
        anyhow::bail!("Private key file not found.");
    }

    println!("  {YELLOW}Check if the encrypted file exists.{RESET}");
    let encrypted_path_struct = ende::PathStructInSshFolder::new(format!("{private_key_file_name}.enc"))?;
    if !encrypted_path_struct.exists() {
        println!("  {YELLOW}Encrypted file {encrypted_path_struct} does not exist.{RESET}");
        println!("  {YELLOW}Create the vault and store a secret using the store command. {RESET}");
        anyhow::bail!("Encrypted file not found.");
    }

    println!("  {YELLOW}Open and read the encrypted file.{RESET}");
    let encrypted_text_with_metadata: String = ende::open_file_b64_get_string(encrypted_path_struct.get_full_file_path())?;
    // parse json
    let mut vec_encrypted_text_with_metadata: Vec<ende::EncryptedTextWithMetadata> = serde_json::from_str(&encrypted_text_with_metadata)?;
    println!("  {YELLOW}Decrypt the file with ssh-agent or private key.{RESET}");
    let mut index_to_delete = None;

    for (index, encrypted_text_with_metadata) in vec_encrypted_text_with_metadata.iter().enumerate() {
        let iter_token_name = if let Some(iter_token_name) = encrypted_text_with_metadata.token_name.as_ref() {
            iter_token_name
        } else {
            continue;
        };
        if iter_token_name != token_name {
            continue;
        }
        index_to_delete = Some(index);
        break;
    }
    if let Some(index_to_delete) = index_to_delete {
        println!("  {YELLOW}Delete token {token_name} from vault. {RESET}");

        vec_encrypted_text_with_metadata.remove(index_to_delete);

        let file_text = serde_json::to_string_pretty(&vec_encrypted_text_with_metadata)?;
        // encode it just to obscure it a little bit
        let file_text = ende::encode64_from_string_to_string(&file_text);

        std::fs::write(encrypted_path_struct.get_full_file_path(), file_text)?;
        println!("  {YELLOW}Encrypted text saved to file.{RESET}");
    } else {
        anyhow::bail!("Token with this name not found.");
    }

    Ok(())
}
