# Description
This is intended to be a discord /command interactions and graphql api server for Norseline.

# Contributing

## Method One (Recommended) - VS Code Remote Container Development
This is the recommended method because it guarantees the environment will match what's expected and prevents you from having to install more than two pieces of software on your computer. This is also easy to use for any other language or development environment you need.

1. Fork this repo.
2. Install Docker on your computer.
    * **Windows**: [Docker Desktop][dockerdesktop] 2.0+ on Windows 10 Pro/Enterprise.
        * Prefer a [WSL2][wsl2] setup if you can.
        * Docker Toolbox is not supported.
    * **Linux**: [Docker CE/EE][dockerce] 18.06+ and [Docker Compose][dockercompose] 1.21+.
        * The Ubuntu snap package is not supported.
    * **Mac**: [Docker Desktop][dockerdesktop] 2.0+.
3. Install Visual Studio Code.
4. Install the [VS Code Remote Development][vscode-remote-dev-extension] extension.
5. Clone this repo and open it in VS Code.
6. Click `Yes` when it asks if you want to open it remotely OR press `CTRL-SHIFT-P` and type `Remote-Containers` and click `Reopen in Container`.
7. Send pull requests with your changes.

To run the app in the container:
1. After the container is created, press `CTRL ~` to bring up the terminal panel, if it is not already visible.
2. Press the + button to create a new bash terminal, if needed.
3. `cd src` to change into the src directory.
4. `flask run --host 0.0.0.0` to run the app and listen on all IP addresses.
    * This is necessary to access the webpage from outside the container. If you use `localhost` or `127.0.0.1` you would be required to access the web page from another program running inside the container.
5. On your computer, browse to http://localhost:5000

Notes:
* There is a really good article called "[Please, everyone, put your entire development environment in Github][article]" to read and understand this method.
* See Microsoft's article "[Developing inside a Container][msgetstarted]" guide

## Method Two - Traditional Development Environment
This is not recommended but is the more traditional development process. While there are fewer steps for this particular project, this method is unsupported and you may be on your own to figure out problems with your own environment.

1. Fork this repo.
2. Install your python IDE/text editor of choice.
3. Install python 3.6 or newer.
4. Submit your PRs to the main project.

# Running the Application
## config<span></span>.py
You will need to set up a config.py file for your local development.
1. Create a new folder underneath `src` called `instance`
2. Inside `src/instance`, create a file called `config.py` and open it for editing.
3. Copy the following into it:
    ```python
    import os

    GOOGLE_CLIENT_ID = os.getenv('GOOGLE_CLIENT_ID')
    GOOGLE_CLIENT_SECRET = os.getenv('GOOGLE_CLIENT_SECRET')

    DISCORD_CLIENT_ID = os.getenv('DISCORD_CLIENT_ID')
    DISCORD_CLIENT_SECRET = os.getenv('DISCORD_CLIENT_SECRET')

    ```
## Oauth2 / OpenID Connect Environment Variables
The app uses environment variables to set secrets. This prevents accidental git commits and uploads to the internet. You will need to set these secrets before you can use the app's authentication features.

1. Log into the developer's console for Google, Discord, or both:
    1. Create an app id and secret.
    2. Create valid callback and javascript urls.
    3. Save the application settings.
    * *See the documentation for each on how to do this.*
2. Set environment variables:
    * Linux / VS Code Remote Containers command line:
        ```bash
        export GOOGLE_CLIENT_ID=exampleid
        export GOOGLE_CLIENT_SECRET=examplesecret
        export DISCORD_CLIENT_ID=exampleid
        export DISCORD_CLIENT_SECRET=examplesecret
        ```
    * Windows Powershell
        ```powershell
        $env:GOOGLE_CLIENT_ID=exampleid
        $env:GOOGLE_CLIENT_SECRET=examplesecret
        $env:DISCORD_CLIENT_ID=exampleid
        $env:DISCORD_CLIENT_SECRET=examplesecret
        ```
    * *Ensure you replace the examples with your actual secrets*


# Important Information
## Pull Requests
Please do not include any local development environment files in your pull requests. The `.devcontainer` files are included as the recommended way to contribute to this project.

## Port Information
The server is set up to listen on port 5000.

During deployment, the port will not change, but it will run in docker a container that will map to port 80 of the server.

If you change the default port, you may face problems you'll have to work out yourself.

[article]:https://www.freecodecamp.org/news/put-your-dev-env-in-github/ "Please, everyone, put your entire development environment in Github"
[dockerdesktop]:https://www.docker.com/products/docker-desktop "Docker Desktop"
[dockerce]:https://docs.docker.com/install/#supported-platforms "Docker CE"
[dockercompose]:https://docs.docker.com/compose/install "Docker Compose"
[msgetstarted]:https://code.visualstudio.com/docs/remote/containers "Developing inside a Container"

[wsl2]:https://docs.microsoft.com/en-us/windows/wsl/install-win10#update-to-wsl-2 "WSL2 Setup Information"

[vscode-remote-dev-extension]:https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack "Visual Studio Code Remote Development Extension"