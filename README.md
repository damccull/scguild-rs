[![CI](https://github.com/damccull/norseline-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/damccull/norseline-rs/actions/workflows/ci.yml)[![Docker Package Build and Push](https://github.com/damccull/norseline-rs/actions/workflows/deploy-gh-package.yml/badge.svg)](https://github.com/damccull/norseline-rs/actions/workflows/deploy-gh-package.yml)
# Description
This is intended to be a discord /command interactions and graphql api server for Norseline.

# Contributing
## Method Two - Traditional Development Environment
1. Fork this repo.
2. Install your IDE/text editor of choice. [Visual Studio Code][vscode] is recommended.
3. Install [rust][rustlang].
4. Make appropriate changes, ensuring that both `cargo build` and `cargo test` complete successfully.
4. Submit your PRs to the main project.

# Running the Application
## Oauth2 / OpenID Connect Environment Variables
The app uses environment variables to set secrets. This prevents accidental git commits and uploads to the internet. You will need to set these secrets before you can use the app's authentication features.

# Important Information
## Pull Requests
Please do not include any local development environment files in your pull requests.

## Port Information
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