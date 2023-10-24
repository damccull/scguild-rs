[![CI](https://github.com/damccull/scguild-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/damccull/scguild-rs/actions/workflows/ci.yml)[![Docker Package Build and Push](https://github.com/damccull/scguild-rs/actions/workflows/deploy-gh-package.yml/badge.svg)](https://github.com/damccull/scguild-rs/actions/workflows/deploy-gh-package.yml)
# Description
This is intended to be a discord /command interactions and graphql api server for scguild.

# Development Prerequisites
* Docker / Docker Desktop
    * This project is set up to use Docker Desktop for development. By default the xtasks (see below) are written under the assumption docker will be available.
* Postgres CLI
    * This project assumes the development machine has the `psql` tool installed to setup a postgres docker image.
* sqlx CLI v0.5.13
    * This project uses the rust sqlx library and assumes the development machine has the sqlx-cli installed to handle database migrations. Please use the correct version.

None of these are technically required if you want to run your own postgres server on another machine and manually run the SQL migration files. `cargo xtask` commands expect these to be present, however.

# Contributing
## cargo xtask
This cargo workspace contains two projects: `scguild-rs` and `xtask`. `xtask` is a simple, bespoke CLI tool written in rust to perform various tasks for this repository that might normally be handled by a makefile or similar script runner.

Instead, these tasks are written directly in rust and the tool is aliased to `cargo xtask` for ease of use anywhere in the repository.

Type `cargo xtask` to see the tasks that can be used. Start here when looking to contribute.

[More info on `cargo xtask`][cargo-xtask]

## How to Contribute
1. Fork this repo
2. Install your IDE/text editor of choice. [Visual Studio Code][vscode] is recommended
3. Install [rust][rustlang-install]
4. Make appropriate changes.
5. Run `cargo xtask ci` and fix any reported errors
6. Submit your PRs to the main project


## Pull Requests
Please do not include any local development files in your pull requests, such as IDE configurations, vscode files, workspace files, etc. The exception to this is the files already provided in this repo. Don't remove those during a commit.

## Development Port Information
The server is set up to listen on port 5000 when running.

When running `cargo test`, each of the integration tests sets up a copy of the webserver and listens on a random port for the lifetime of the test. The port will be freed when the test completes. This allows the integration tests to check the entire chain of functionality while running many test in parallel to finish more quickly.


# Discord /commands
Commands will be grouped by common function. Below is a breakdown of each command structure. All parameters containing whitespace need to be surrounded with quotation marks.

* fleet
    * Description
        * Lists the users configured fleet of ships.
        * Contains subcommands to make changes to the fleet. 
    * Usage:
        * `/fleet`
    * Subcommands
        * add
            * Description
                * Allows a user to add a ship to their fleet.
            * Usage
                * `/fleet add`
                    * Triggers a wizard to allow a user to add a ship step by step
                * `/fleet add <manufacturer> <model> [<desired name>]`
                    * Adds a ship with the specified information.
        * remove
            * Description
                * Allows a player to remove a specific ship from their fleet by its ID number.
            * Usage
                * `/fleet remove <id>`
        * name
            * Description
                * Allows a player to update the name of a ship by its ID number.
            * Usage
                * `/fleet name <id> <new name>`

[wsl2]: https://docs.microsoft.com/en-us/windows/wsl/install-win10#update-to-wsl-2 "WSL2 Setup Information"
[vscode]: https://code.visualstudio.com/ "Visual Studio Code"
[rustlang-install]: https://www.rust-lang.org/learn/get-started "Install Rust"
[cargo-xtask]: https://github.com/matklad/cargo-xtask "cargo-xtask"
