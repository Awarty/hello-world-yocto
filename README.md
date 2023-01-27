# VSCode extensions

- Better TOML
- CodeLLDB
- crates
- Error Lens
- rust-analyzer

# Conventions

- [Package layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)
- [Styling](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)

# Rust
Install rustup (https://rustup.rs/):
´´´bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
´´´

# Cross compilations
cargo install -f cross

cargo build --verbose --target armv7-unknown-linux-gnueabihf

# Description
GPT Edge consists of 3 different daemons:
* Device Service
* Passenger Information Service
* Vehicle Assigment Service


And an QT application, which is the front end.


We assume that there are a MQTT broker (Mosquitto) and a DNS-SD service (Avahi) installed on the devcie the system runs on.

## Devuce Servive 
Written i Rust and runs as a Linux daemon.
Connects to the cloud (Azure IoT Hub) and gets the configuration (Twin), and distributes the configuration on the local MQTT broker.
Has the ability to start, stop and configure a local MQTT broker (Mosquitto).
Has the ability to adverties the MQTT broker on the LAN using DNS-SD (Avahi).

## Passenger Information Service
Written in Rust and runs as a Linux daemon.
Handles the different interfaces to the end user.
* Outer signage, LED matrices.
* Inner signage.
* Call out, audio.

### Front end (we have to come up with a better name)
This a Qt (C++ and QML) application. Stand alone from the other services.
Which acts as the front end for the Passenger Information Service. It will get its information from a restful API that is controlled by Passenger Information Service.
A main feature is to display something when the system malfunctions.

## Vehicle Assigment Service
Handles journes, trips etc. etc. ?

## In a Azure IoT Edge environment
### The gateway (Stratopi/Large Waveshare) is not replaced
Run together with the rest of the system. Configured to use the MQTT broker which runs on the gateway, it will find the broker using DNS-SD and looking for a service named "_mqtt._tcp.local.".

## In a IoT Core environment
### The gateway (Stratopi) is no replaced
Runs in a "stand alone" mode, where every device has its own connection to the MQTT broker in the cloud, and establish a brdige to the local broker. I.e. the devices are not aware of each other at all. The other services on the device connects to the local broker (localhost), DNS-SD is not used at all.
