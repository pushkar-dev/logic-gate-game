mod gates;

use gates::logic_gates::LogicGate;
use gates::circuit::Circuit;

fn main() {
    let mut circuit = Circuit::new();

    // Add initial inputs
    circuit.add_input(true);
    circuit.add_input(false);

    // Add gates
    circuit.add_gate(LogicGate::AndGate, vec![0, 1]);
    circuit.add_gate(LogicGate::NotGate, vec![2]);

    // Simulate for a number of cycles
    let cycles = 3;
    let outputs = circuit.evaluate(cycles);
    println!("Outputs after {} cycles: {:?}", cycles, outputs);
}
