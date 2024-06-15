# Logic Gate Simulation Game

A Rust-based logic gate simulation game where users can combine basic logic gates to create complex circuits and obtain the desired outputs. The game simulates the logic gates' behavior over clock cycles.

## Features

- Basic logic gates: AND, OR, NOT...
- Circuit creation with multiple gates and inputs.
- Simulation over clock cycles to evaluate circuit outputs.
- Easy-to-extend for additional gate types and features.
- Endless possibilities

## Installation

1. **Clone the repository:**

    ```bash
    git clone https://github.com/pushkar-dev/logic-gate-game.git
    cd logic-gate-game
    ```

2. **Build the project:**

    Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

    ```bash
    cargo build --release
    ```

## Usage

1. **Run the simulation:**

    ```bash
    cargo run
    ```

## Project Structure

- `src/main.rs`: The main entry point of the application. Contains the setup and execution of the circuit simulation.
- `src/gates/logic_gate.rs`: Contains the definition and implementation of the `LogicGate` enum.
- and `src/gates/circuit.rs` contains `Circuit` struct, which has methods to create circuits and run simulation for a given number of clock cycles.
- `resources/config.toml` contains configuration for the window of the simulation.
- `src/frontend` contains logic to draw the game state.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any bug reports or feature requests.

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature`).
3. Commit your changes (`git commit -am 'Add some feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Create a new Pull Request.

## TODO:

 - [X] Create branch evaluation logic
 - [X] Make branch evaluation logic to be computed parallel
 - [ ] $${\color{red}{Create methods for merging branches}}$$
 - [ ] $${\color{red}{Code multiple branch evaluation logic}}$$
 - [ ] List a new syntax for building and defining circuits
 - [ ] Create GUI for interaction (in progress)
 - [ ] Create level design 

## Acknowledgments

- [Rust Programming Language](https://www.rust-lang.org/)
- Inspired by  "computer engineering for babies" 

