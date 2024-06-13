
# Rust Sensor Data Simulation

Welcome to the Rust Sensor Data Simulation project! This repository contains a Rust application that simulates sensor data, encodes and decodes it to/from binary format, and transmits it over a TCP socket. The application is designed with a publisher and a listener running on separate threads to handle the data transmission.

## To do:
- Add docker file
- Add docker compose file
- Reorganise file structure
- Generate several data and simulate with more data

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [License](#license)

## Overview

This project demonstrates:
- Generating simulated sensor data.
- Encoding the data to a binary format.
- Transmitting the encoded data over a TCP socket.
- Receiving and decoding the data from the TCP socket.

Two threads are utilized:
- **Publisher:** Encodes and sends the data.
- **Listener:** Receives and decodes the data.

## Features

- **Sensor Data Simulation:** Generate mock data representing sensor readings.
- **Binary Encoding/Decoding:** Convert data to/from binary format.
- **TCP Socket Communication:** Send and receive data over TCP sockets.
- **Multi-threading:** Use Rust's concurrency model to handle data publishing and listening in separate threads.

## Prerequisites

- Rust (minimum version: 1.78.0)
- Cargo (Rust package manager)

Ensure you have Rust and Cargo installed. You can install them via [rustup](https://rustup.rs/).

## Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/davidedelerma/rust-pub-sub.git
    cd rust-sensor-simulation
    ```

2. Build the project:
    ```bash
    cargo build
    ```

## Usage

To run the project, use the following command:

```bash
cargo run
```

This command starts both the publisher and listener threads. The publisher generates and sends simulated sensor data, while the listener receives and decodes the data.

### Running in Separate Terminals (TO DO!)

To simulate the program running in different terminals:

1. Start the listener:
    ```bash
    cargo run -- --role listener
    ```

2. Start the publisher in another terminal:
    ```bash
    cargo run -- --role publisher
    ```

Ensure the addresses match if you're running these in separate terminals or systems.

## Project Structure

- `src/`: Contains the Rust source files.
- `src/main.rs`: Entry point of the application.
- `src/publisher.rs`: Contains the publisher and listener thread logic, plus the encoding and decoding part.
- `Cargo.toml`: Project configuration file.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Feel free to modify this README to better suit your project and its requirements.