//! src/bin/treasure_your_passwords/main.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # treasure_your_passwords
//!
//! **Use SSH private key to store your passwords locally and make them strong**  
//! ***version: 0.0.70 date: 2025-03-29 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/treasure_your_passwords)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!
//!  ![License](https://img.shields.io/badge/license-MIT-blue.svg)
//!  ![Rust](https://github.com/bestia-dev/treasure_your_passwords/workflows/rust_fmt_auto_build_test/badge.svg)
//!  ![treasure_your_passwords](https://bestia.dev/webpage_hit_counter/get_svg_image/779107454.svg)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-655-green.svg)](https://github.com/bestia-dev/treasure_your_passwords/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-198-blue.svg)](https://github.com/bestia-dev/treasure_your_passwords/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-75-purple.svg)](https://github.com/bestia-dev/treasure_your_passwords/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/treasure_your_passwords/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/treasure_your_passwords/)
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

// region: Public API constants
// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
/// ANSI color
pub const RED: &str = "\x1b[31m";
/// ANSI color
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
/// ANSI color
pub const YELLOW: &str = "\x1b[33m";
/// ANSI color
#[allow(dead_code)]
pub const BLUE: &str = "\x1b[34m";
/// ANSI color
pub const RESET: &str = "\x1b[0m";
// endregion: Public API constants

// import trait
use secrecy::ExposeSecret;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TreasureConfig {
    pub treasure_private_key_file_name: String,
}

/// Application state (static) is initialized only once in the main() function.
///
/// And then is accessible all over the code.
pub static TREASURE_CONFIG: std::sync::OnceLock<TreasureConfig> = std::sync::OnceLock::new();

/// Entry point into the bin-executable.
fn main() {
    std::panic::set_hook(Box::new(panic_set_hook));
    tracing_init();
    treasure_config_initialize();
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
// region: general functions

/// Initialize tracing to file logs/automation_tasks_rs.log
///
/// The folder logs/ is in .gitignore and will not be committed.
pub fn tracing_init() {
    // uncomment this line to enable tracing to file
    // let file_appender = tracing_appender::rolling::daily("logs", "treasure_your_password.log");

    let offset = time::UtcOffset::current_local_offset().expect("should get local offset!");
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(
        offset,
        time::macros::format_description!("[hour]:[minute]:[second].[subsecond digits:6]"),
    );

    // Filter out logs from: hyper_util, reqwest
    // A filter consists of one or more comma-separated directives
    // target[span{field=value}]=level
    // examples: tokio::net=info
    // directives can be added with the RUST_LOG environment variable:
    // export RUST_LOG=automation_tasks_rs=trace
    // Unset the environment variable RUST_LOG
    // unset RUST_LOG
    let filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive("hyper_util=error".parse().unwrap_or_else(|e| panic!("{e}")))
        .add_directive("reqwest=error".parse().unwrap_or_else(|e| panic!("{e}")));

    tracing_subscriber::fmt()
        .with_file(true)
        .with_max_level(tracing::Level::DEBUG)
        .with_timer(timer)
        .with_line_number(true)
        .with_ansi(false)
        //.with_writer(file_appender)
        .with_env_filter(filter)
        .init();
}

/// The original Rust report of the panic is ugly for the end user
///
/// I use panics extensively to stop the execution. I am lazy to implement a super complicated error handling.
/// I just need to stop the execution on every little bit of error. This utility is for developers. They will understand me.
/// For errors I print the location. If the message contains "Exiting..." than it is a "not-error exit" and  the location is not important.
fn panic_set_hook(panic_info: &std::panic::PanicHookInfo) {
    let mut string_message = "".to_string();
    if let Some(message) = panic_info.payload().downcast_ref::<String>() {
        string_message = message.to_owned();
    }
    if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
        string_message.push_str(message);
    }

    tracing::debug!("{string_message}");
    eprintln!("{string_message}");

    if !string_message.contains("Exiting...") {
        let file = panic_info.location().unwrap().file();
        let line = panic_info.location().unwrap().line();
        let column = panic_info.location().unwrap().column();
        tracing::debug!("Location: {file}:{line}:{column}");
        eprintln!("Location: {file}:{line}:{column}");
    }
}

// endregion: general functions

/// Application state (static) is initialized only once in the main() function.
///
/// And then is accessible all over the code.
fn treasure_config_initialize() {
    if TREASURE_CONFIG.get().is_some() {
        return;
    }

    let treasure_config_json = std::fs::read_to_string("treasure_config.json").unwrap();
    let treasure_config: TreasureConfig = serde_json::from_str(&treasure_config_json).unwrap();
    let _ = TREASURE_CONFIG.set(treasure_config);
}

/// Print help on the terminal.
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

/// Convert to strong password.
fn convert_to_strong_password() {
    let strong_password = ende::generate_strong_password_mod::generate_strong_password().unwrap();
    println!("{}", strong_password);
}

/// List token names.
fn list_token_names() {
    let vec_string = ende::secret_vault_mod::list_tokens_from_vault().unwrap();
    println!("{:?}", vec_string);
}

/// Store token, encrypted.
fn store_token(token_name: &str) {
    ende::secret_vault_mod::store_secret_token_to_vault(token_name).unwrap();
}

/// Show token, decrypted.
fn show_token(token_name: &str) {
    let secret_token = ende::secret_vault_mod::show_secret_token_from_vault(token_name).unwrap();
    println!("{}", secret_token.expose_secret());
}

/// Delete token.
fn delete_token(token_name: &str) {
    ende::secret_vault_mod::delete_token_from_vault(token_name).unwrap();
}
