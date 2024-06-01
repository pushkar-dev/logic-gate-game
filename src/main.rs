mod gates;
mod tests; // testcases

use gates::logic_gates::LogicGate;
use gates::circuit::{Circuit, RunType};

const RUN_TYPE: RunType = RunType::PARALLEL;

fn main() {
    let mut circuit = Circuit::new();

    // // Add initial inputs
    // circuit.add_input(true);
    // circuit.add_input(false);

    // Add gates
    circuit.add_gate(LogicGate::AndGate, vec![0, 1], None);
    circuit.add_gate(LogicGate::NotGate, vec![2], None);

    // Simulate for a number of cycles
    
    let cycles = 3;
    let outputs = circuit.evaluate(cycles, RUN_TYPE);
    println!("Outputs after {} cycles: {:?}", cycles, outputs);

    // simulate a cross coupled inverter
    let mut circuit2= Circuit::new();
    circuit2.add_input(true);
    circuit2.add_gate(LogicGate::NotGate, vec![1], Some(2));
    circuit2.add_gate(LogicGate::NotGate, vec![2], Some(1));

    let ops = circuit2.evaluate(cycles, RUN_TYPE); // after each clk cycle the op flips
    println!("Outputs after {} cycles: {:?}", cycles, ops);


}
