//! src/bin/treasure_your_passwords/main.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # treasure_your_passwords
//!
//! **Use SSH private key to store your passwords locally and make them strong**  
//! ***version: 0.0.55 date: 2025-03-11 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/treasure_your_passwords)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!
//!  ![License](https://img.shields.io/badge/license-MIT-blue.svg)
//!  ![Rust](https://github.com/bestia-dev/treasure_your_passwords/workflows/rust_fmt_auto_build_test/badge.svg)
//!  ![treasure_your_passwords](https://bestia.dev/webpage_hit_counter/get_svg_image/779107454.svg)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-511-green.svg)](https://github.com/bestia-dev/treasure/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-209-blue.svg)](https://github.com/bestia-dev/treasure/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-56-purple.svg)](https://github.com/bestia-dev/treasure/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/treasure/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/treasure/)
//!
//! Hashtags: #maintained #ready-for-use #rustlang  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  
//!
//! ## create the SSH key
//!
//! Create the SSH key and protect it with a passcode.
//!
//! ```bash
//! ssh-keygen -t ed25519 -f "vault_ssh_1" -C "vault for secret tokens"
//! ```
//!
//! Save the file `ssh_private_key_bare_file_name.cfg` with the content `vault_ssh_1`.  
//! The program `treasure` will read this file to find the SSH private key in the `~/.ssh` folder.
//!
//! ## Use SSH private key to store passwords
//!
//! With one SSH private key, we can store many secret tokens.
//!
//! ```bash
//! treasure list
//! treasure store token_name
//! treasure show token_name
//! treasure delete token_name
//! ```
//!
//! ## convert to strong password
//!
//! I'd like to have a CLI where to input a humane easy to memorize password and convert it into a strong password.  
//!
//! ```bash
//! treasure strong
//! ```
//!
//! Then sign it with a private key (this encryption is reversible using the public key).  
//! Then hash it (this is a one way encryption, so nobody can come back to the first secret).  
//! Finally, convert it into a string long 32 characters with ascii7 characters (lowercase, uppercase, numeric and special characters).  
//! What makes this conversion secure is: only the user of the private key can convert the easy password into the same strong_password.
//!
//! Strong passwords must use the clipboard. The risk is that it can stay in the clipboard and can be read from the clipboard.
//!
//! ## Development details
//!
//! Read the development details in a separate md file:
//! [DEVELOPMENT.md](DEVELOPMENT.md)
//!
//! ## Releases changelog
//!
//! Read the releases changelog in a separate md file:
//! [RELEASES.md](RELEASES.md)
//!
//! ## TODO
//!
//! - better readme
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

mod encrypt_decrypt_with_ssh_key_mod;
use encrypt_decrypt_with_ssh_key_mod as ende;

pub use ende::{GREEN, RED, RESET, YELLOW};
// import trait
use secrecy::ExposeSecret;

/// # entry point into the bin-executable
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
            None => eprintln!("{RED}Error: Missing arguments `token_name`.{RESET}"),
        },
        Some("show") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(token_name) => {
                show_token(token_name);
            }
            None => eprintln!("{RED}Error: Missing arguments `token_name`.{RESET}"),
        },
        Some("delete") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(token_name) => {
                delete_token(token_name);
            }
            None => eprintln!("{RED}Error: Missing arguments `token_name`.{RESET}"),
        },
        _ => eprintln!("{RED}Error: Unrecognized arguments. Try `treasure --help`{RESET}"),
    }
}

/// # print help
fn print_help() {
    println!(
        r#"
  {YELLOW}Welcome to treasure CLI
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

/// # panics if it cannot read file_bare_name
fn read_bare_file_name() -> String {
    let Ok(file_bare_name) = std::fs::read_to_string("ssh_private_key_bare_file_name.cfg") else {
        panic!("{RED}Cannot read file ssh_private_key_bare_file_name.cfg{RESET}");
    };
    file_bare_name
}

/// # convert to strong password
fn convert_to_strong_password() {
    let file_bare_name = read_bare_file_name();
    let strong_password = ende::generate_strong_password_mod::generate_strong_password(&file_bare_name).unwrap();
    println!("{}", strong_password);
}

/// # list token names
fn list_token_names() {
    let file_bare_name = read_bare_file_name();
    let vec_string = ende::secret_vault_mod::list_tokens_from_vault(&file_bare_name).unwrap();
    println!("{:?}", vec_string);
}

/// # store token
fn store_token(token_name: &str) {
    let file_bare_name = read_bare_file_name();
    ende::secret_vault_mod::store_secret_token_to_vault(&file_bare_name, token_name).unwrap();
}

/// # show token
fn show_token(token_name: &str) {
    let file_bare_name = read_bare_file_name();
    let secret_token = ende::secret_vault_mod::show_secret_token_from_vault(&file_bare_name, token_name).unwrap();
    println!("{}", secret_token.expose_secret());
}

/// # delete token
fn delete_token(token_name: &str) {
    let file_bare_name = read_bare_file_name();
    ende::secret_vault_mod::delete_token_from_vault(&file_bare_name, token_name).unwrap();
}
