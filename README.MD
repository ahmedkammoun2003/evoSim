# EvoSim

EvoSim is a simulation project that visualizes the behavior of animals and food in a 2D environment using HTML5 Canvas. The project leverages WebAssembly for performance and provides a simple interface for rendering shapes like triangles and circles.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Building the Rust Code](#building-the-rust-code)
- [Usage](#usage)
## Features

- Real-time simulation of animals and food.
- Custom drawing functions for triangles and circles.
- Responsive rendering based on viewport size.
- Utilizes WebAssembly for efficient computation.

## Installation

To get started with EvoSim, clone the repository and install the necessary dependencies:
```bash
git clone https://github.com/yourusername/evoSim.git
cd evoSim
npm install
```

Make sure you have Node.js and npm installed on your machine.

## Building the Rust Code

To build the Rust components of the project, you need to have Rust and Cargo installed. If you haven't installed them yet, you can do so from [rustup.rs](https://rustup.rs/).

Once you have Rust installed, navigate to the Rust library directory and build the project:

```bash
cd libs
cargo build --release
cd front
wasm-pack build
```

This will compile the Rust code and generate the necessary WebAssembly files in the `target/wasm32-unknown-unknown/release` directory.

## Usage

To run the simulation, use the following command:

```bash
npm run start
```

This will start a local development server. Open your browser and navigate to `http://localhost:3000` to view the simulation.