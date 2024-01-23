# MQTT Practice

## Overview
MQTT Practice is a Rust-based project demonstrating the use of MQTT (Message Queuing Telemetry Transport) protocol for messaging. The project consists of two main components: a publisher (`pub`) and a subscriber (`sub`). It showcases the basic functionality of MQTT, including connecting to a broker, publishing messages, and subscribing to topics.

## Features
- **MQTT Publisher**: Sends messages to specified topics.
- **MQTT Subscriber**: Subscribes to topics and receives messages.
- **Connection Management**: Handles connecting and disconnecting from the MQTT broker.
- **Message Handling**: Demonstrates sending and receiving messages using the MQTT protocol.

## Project Structure
- **Publisher (`pub/main.rs`)**: Contains the logic for the MQTT publisher.
- **Subscriber (`sub/main.rs`)**: Contains the logic for the MQTT subscriber.
- **Cargo Configuration (`Cargo.toml`)**: Manages Rust package configurations and dependencies.

## How to Run
- Ensure you have Rust and the necessary dependencies installed.
- Clone the repository and navigate to the project directory.
- To run the publisher:
cargo run --bin pub
- To run the subscriber:
cargo run --bin sub

## Dependencies
- Rust Programming Language
- `paho-mqtt`: MQTT client library for Rust.

## Note
This project is intended for educational purposes to demonstrate the use of MQTT in Rust. It serves as a basic example of implementing MQTT clients for message publishing and subscribing.
