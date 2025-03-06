<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

# treasure_your_passwords

[//]: # (auto_cargo_toml_to_md start)

**Use SSH private key to store your passwords locally and make them strong**  
***version: 0.0.41 date: 2025-03-06 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/treasure_your_passwords)***

 ![maintained](https://img.shields.io/badge/maintained-green)
 ![work-in-progress](https://img.shields.io/badge/work_in_progress-yellow)
 ![rustlang](https://img.shields.io/badge/rustlang-orange)

[//]: # (auto_cargo_toml_to_md end)

 ![License](https://img.shields.io/badge/license-MIT-blue.svg)
 ![Rust](https://github.com/bestia-dev/treasure_your_passwords/workflows/rust_fmt_auto_build_test/badge.svg)
 ![treasure_your_passwords](https://bestia.dev/webpage_hit_counter/get_svg_image/779107454.svg)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-507-green.svg)](https://github.com/bestia-dev/treasure/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-65-blue.svg)](https://github.com/bestia-dev/treasure/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-58-purple.svg)](https://github.com/bestia-dev/treasure/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/treasure/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/treasure/)

[//]: # (auto_lines_of_code end)

Hashtags: #maintained #work-in-progress #rustlang 
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  

## create the SSH key

Create the SSH key and protect it with a passcode.

```bash
ssh-keygen -t ed25519 -f "vault_ssh_1" -C "vault for secret tokens"
```

Save the file `ssh_private_key_bare_file_name.cfg` with the content `vault_ssh_1`. The program `treasure` will read this file to find the SSH private key in the `~/.ssh` folder.

## Use SSH private key to store passwords

With one SSH private key, we can store many secret tokens.

```bash
treasure list
treasure store token_name
treasure show token_name
treasure delete token_name
```

## convert to strong password

I'd like to have a CLI where to input a humane easy to memorize password and convert it into a strong password.  

```bash
treasure strong
```

Then sign it with a private key (this encryption is reversible using the public key).  
Then hash it (this is a one way encryption, so nobody can come back to the first secret).  
Finally, convert it into a string long 32 characters with ascii7 characters (lowercase, uppercase, numeric and special characters).  
What makes this conversion secure is: only the user of the private key can convert the easy password into the same strong_password.

Strong passwords must use the clipboard. The risk is that it can stay in the clipboard and can be read from the clipboard.

## Development details

Read the development details in a separate md file:
[DEVELOPMENT.md](DEVELOPMENT.md)

## Releases changelog

Read the releases changelog in a separate md file:
[RELEASES.md](RELEASES.md)

## TODO

- better readme

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
