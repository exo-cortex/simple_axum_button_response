A simple website with buttons that activate a buzzer connected to a raspberry pi which is running a webserver.

Cross-compilation files can be found [here](https://github.com/abhiTronix/raspberry-pi-cross-compilers#-toolchain-downloads)

# About this project
- host a website ✔
- host it on a Raspberry Pi Zero ✔
- the website contains a button that triggers a server-side event ✔
- use the Raspberry Pis GPIO to make a sound ✔
- make the button trigger the sound on the Raspberry Pi ✔
- can use https ✔ (only tested with self-signed certificate at the moment)
- use https

## Compiling the project for a Raspberry Pi Zero W with Raspberry Pi OS
- Connect a buzzer to GPIO 2 (pin #3) on your raspberry pi
- On your main computer install Rust via the `rustup` - see https://rustup.rs/ for instructions
- Clone this repository and enter it (run a shell from inside that folder)
- Install the correct target: \
`$ rustup target add arm-unknown-linux-gnueabihf`
- download the toolchain files for crosscompilation\ (you can theoretically compile this directly on your Raspberry Pi, but I haven't actually tested it as it takes *ages*)
- In `.cargo/config.toml` change the 2 paths under `[target.arm-unknown-linux-gnueabihf]` so that they point to the correct location on your machine.
- To compile run the command `$ cargo build --release` and wait. Once compiled (successfully) the binary should lie at `target/arm-unknown-linux-gnueabihf/`.

## Running the program on your raspberry pi zero
Copy the files
- `buzzer_control_webserver` (binary file),
- `index.html`,
- `main.js`,
- `key.pem` - your key,
- `cert.pem` - your certificate
- `favicon.svg` \
to your Raspberry Pi Zero.\
It is HIGHLY RECOMMENDED to put `index.html`, `main.js` and `favicon.svg` in a separate directory `html` as the `*.pem`-files! Otherwise `key.pem` can be accessed from outside! That's a **BIG NO-NO**!
- Now you should be able to run the binary
- Specify the location of your website files with `-f <html-directory>`, the port with `-p <port>` and the folder that holds `key.pem` and `cert.pem` by setting commandline option `-t <tls-directory>`.

Example usage: \
`$ ./buzzer_control_server -f html/ -p 8080 -t certificates/ `

# todo
- use websockets to communicate the state of the buzzer and remove client-side polling