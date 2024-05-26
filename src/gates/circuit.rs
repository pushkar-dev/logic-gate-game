
use crate::gates::logic_gates::LogicGate;

pub struct Circuit {
    gates: Vec<(LogicGate, Vec<usize>)>, // (gate, input indices)
    inputs: Vec<bool>
}

impl Circuit {
    pub fn new() -> Self {
        Circuit {
            gates: Vec::new(),
            inputs: Vec::new()
        }
    }

    // TODO: implement a join function
    //		- create branch
	//  		- function add input and add gate
	// 		- join branch
	//  		- add the outputs of two branch via a gate

    pub fn add_input(&mut self, input: bool) {
        self.inputs.push(input);
    }

    pub fn add_gate(&mut self, gate: LogicGate, input_indices: Vec<usize>) {
        self.gates.push((gate, input_indices));
    }


    pub fn evaluate(&self, cycles: usize) -> Vec<bool> {
        let mut outputs = vec![false; self.inputs.len() + self.gates.len()];

        // Initialize the outputs with initial inputs
        outputs[..self.inputs.len()].copy_from_slice(&self.inputs);
        for clk in 0..cycles-1 {
            let mut new_outputs = outputs.clone(); // start a new transaction at each cycle

            for (index, (gate, input_indices)) in self.gates.iter().enumerate() {
                let inputs: Vec<bool> = input_indices.iter().map(|&i| outputs[i]).collect();
                let result = gate.evaluate(&inputs);
                new_outputs[self.inputs.len() + index] = result;
                
                println!("\nPropogating gate {} at index {}", gate.info(), index);
                println!("Inputs = {:?} | output = {}",  inputs, result);
                println!("change at index {}, new val {:?}", self.inputs.len() + index-1, new_outputs);
                println!("____________________________________________________\n")
                // result for ith gate is stored at [inputs.len()+index_of_gate]
            }

            outputs = new_outputs; // only commit after one cycle
            println!("---------- {} cycles complete ------------ : evaluated to {:?}", clk+1, outputs);
        }

        outputs
    }
}
