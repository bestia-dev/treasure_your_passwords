//! src/bin/treasure_your_passwords/main.rs

mod encrypt_decrypt_with_ssh_key_mod;
use encrypt_decrypt_with_ssh_key_mod as ende;

pub use ende::{GREEN, RED, RESET, YELLOW};
use secrecy::ExposeSecret;

/// entry point into the bin-executable
fn main() {
    // logging is essential for every project
    pretty_env_logger::init();

    // super simple argument parsing. There are crates that can parse more complex arguments.
    match std::env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("strong") => convert_to_strong_password(),
        Some("list") => list_token_names(),
        Some("store") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(token_name) => {
                store_token(token_name);
            }
            None => println!("{RED}Error: Missing arguments `token_name`.{RESET}"),
        },
        Some("show") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(token_name) => {
                show_token(token_name);
            }
            None => println!("{RED}Error: Missing arguments `token_name`.{RESET}"),
        },
        Some("delete") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(token_name) => {
                delete_token(token_name);
            }
            None => println!("{RED}Error: Missing arguments `token_name`.{RESET}"),
        },
        _ => println!("{RED}Error: Unrecognized arguments. Try `treasure --help`{RESET}"),
    }
}

/// print help
fn print_help() {
    println!(
        r#"
    {YELLOW}Welcome to treasure CLI!
    Please, treasure your passwords!
    This small CLI can store your passwords encrypted with your SSH private key.
    With the same private key it can convert a simple human readable seed to a strong password.
    Write the ssh private key bare file name into the file 'ssh_private_key_bare_file_name.cfg'. {RESET}

{GREEN}treasure --help{RESET}
{GREEN}treasure strong {RESET}
{GREEN}treasure list{RESET}
{GREEN}treasure store token_name{RESET}
{GREEN}treasure show token_name{RESET}
{GREEN}treasure delete token_name{RESET}

    {YELLOW}© 2025 bestia.dev  MIT License github.com/bestia-dev/treasure_your_passwords{RESET}
"#
    );
}

/// convert to strong password
fn convert_to_strong_password() {
    let file_bare_name = std::fs::read_to_string("ssh_private_key_bare_file_name.cfg").unwrap();
    let strong_password = ende::generate_strong_password_mod::generate_strong_password(&file_bare_name).unwrap();
    println!("{}", strong_password);
}

fn list_token_names() {
    let file_bare_name = std::fs::read_to_string("ssh_private_key_bare_file_name.cfg").unwrap();
    let vec_string = ende::secret_vault_mod::list_token_from_vault(&file_bare_name).unwrap();
    println!("{:?}", vec_string);
}

/// store token
fn store_token(token_name: &str) {
    let file_bare_name = std::fs::read_to_string("ssh_private_key_bare_file_name.cfg").unwrap();
    ende::secret_vault_mod::store_secret_token_to_vault(&file_bare_name, token_name).unwrap();
}

/// show token
fn show_token(token_name: &str) {
    let file_bare_name = std::fs::read_to_string("ssh_private_key_bare_file_name.cfg").unwrap();
    let secret_token = ende::secret_vault_mod::show_secret_token_from_vault(&file_bare_name, token_name).unwrap();
    println!("{}", secret_token.expose_secret());
}

/// delete token
fn delete_token(token_name: &str) {
    let file_bare_name = std::fs::read_to_string("ssh_private_key_bare_file_name.cfg").unwrap();
    ende::secret_vault_mod::delete_token_from_vault(&file_bare_name, token_name).unwrap();
}
