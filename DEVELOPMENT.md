# Development details of treasure_your_passwords

## Project name and program name

The project (package) name is long and descriptive `treasure_your_passwords`. The program name to execute in CLI should be short and concise: `treasure`.

It complicates the automation scripts to have this 2 names different. To make it simpler, in development I use only the long name.

Then finally in bash terinal, I make an alias to use it with the short name.

```bash
alias treasure = ./treasure_your_passwords
```

## CRUSTDE - Containerized Rust Development Environment

I recommend using the CRUSTDE - Containerized Rust Development Environment to write Rust projects. Follow the instructions here <https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod>.  

It is an isolated development environment that will not mess with you system.
It will work on Linux (tested on Debian) and inside WSL (Windows Subsystem for Linux).

You just need to install the newer alternative to Docker: [podman](https://podman.io/). Then you download the prepared container image from DockerHub (3GB). And then a little juggling with ssh keys. All this is simplified by running a few bash scripts. Just follow the easy instructions.  

The container image contains cargo, rustc, wasm-pack, basic-http-server, cargo-auto and other utils that a Rust project needs.  

## Workflow with automation_tasks_rs and cargo-auto

For easy workflow, use the automation tasks that are already coded in the sub-project `automation_tasks_rs`. This is a basic workflow:

```bash
cargo auto build
cargo auto release
cargo auto doc
cargo auto test
cargo auto commit_and push
cargo auto github_new_release
```

Every task finishes with instructions how to proceed.  
The [cargo-auto](https://github.com/automation-tasks-rs/cargo-auto) and [dev_bestia_cargo_completion](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion) are already installed inside the CRUSTDE container.

You can open the automation sub-project in VSCode and then code your own tasks in Rust.

```bash
code automation_tasks_rs
```

## super simple argument parsing

I use a super simple code to parse CLI arguments inside the `src/bin/treasure_your_passwords/main.rs`. There are crate libraries that enable very complex argument parsing if needed.

## Modules

The code is separated into module files.

## Markdown

README.md and all the doc-comments are in markdown. To separate paragraphs in markdown use an empty line between them.
I tried other variants like double-space or backslash, but an empty line is the most used in the wild.

## Error handling thiserror and anyhow

Rule number one is never to use `.unwrap()` in your real Rust code. It is a sign, you are not Error handling properly.
Maybe `unwrap()` can be fine for some fast learning examples, but for any real-life Rust code, you must use some `Error handling`. There are many different ways to do that in Rust. I choose the pair of libraries `thiserror` and `anyhow`. The first is made for libraries, the second is made for bin-executables.  
The library needs an Enum with all the possible errors that this library can return. With `#[derive(Error)]` this enum gets everything needed to be a true Rust error struct. Every error can have a formatting string and a struct of data.  
The bin-executable does not want to be involved in every possible error separately. It needs an umbrella for all possible errors with `anyhow::Result`.  
Inside the code, mostly propagate the errors with the `?` Operator after the `Result` value instead of unwrap() or the match expression.
In the tests we don't want to work with Error handling. There, instead of `.unwrap()`, use the similar function `.expect(&str)` that has an additional description string. I use expect() when I am 100% sure the panic cannot happen because I checked some conditions before it.  

## Windows git-bash

This CLI program can run in `windows git-bash`. This environment has also other names like: cygwin, msys2, msys64, MingW64, git-for-windows, msys or msysGit.

There is still a library crate `ssh-agent-client-rs` that does not run on windows, but I sent a [Pull Request](https://github.com/nresare/ssh-agent-client-rs/pull/29) to solve this problem. In the meantime I use my fork on `https://github.com/bestia-dev/ssh-agent-client-rs-win-git-bash` and the branch `win-git-bash`.
