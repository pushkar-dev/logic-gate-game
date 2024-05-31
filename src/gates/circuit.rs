
use crate::gates::logic_gates::LogicGate;

pub struct Component{
    gate: LogicGate,
    input_indices: Vec<usize>,
    output_index: usize,
}

impl Component{
    fn fire(&self, outputs: &Vec<bool>)->bool{
        let pins: Vec<bool> = self.input_indices.iter().map(|&i| outputs[i]).collect();
        let result = self.gate.evaluate(&pins);
        println!("Inputs = {:?}, op_index = {} | output = {}",  pins, self.output_index,result);
        println!("____________________________________________________\n");
        result
    }
}

pub struct Circuit {
    components: Vec<Component>, // (gate, input indices)
    pins: Vec<bool>
}

impl Circuit {
    pub fn new() -> Self {
        Circuit {
            components: Vec::new(),
            pins: Vec::new()
        }
    }

    pub fn add_input(&mut self, input: bool) {
        self.pins.push(input);
    }

    pub fn add_gate(&mut self, gate: LogicGate, input_indices: Vec<usize>) {
        let component = Component{
            gate,
            input_indices,
            output_index: (self.components.len()+self.pins.len())
        };
        self.components.push(component);
    }

    pub fn evaluate(&self, cycles: usize) -> Vec<bool> {
        let mut outputs = vec![false; self.pins.len() + self.components.len()];

        // Initialize the outputs with initial pins
        outputs[..self.pins.len()].copy_from_slice(&self.pins);
        
        for clk in 0..cycles-1 {
            let mut new_outputs = outputs.clone(); // start a new transaction at each cycle

            // TODO: parallelize this for loop
            for component in &self.components{
                
                println!("\nPropogating gate {}", component.gate.info());
                new_outputs[component.output_index] = component.fire(&outputs);
                
            };

            outputs = new_outputs; // only commit after one cycle
            println!("---------- {} cycles complete ------------ : evaluated to {:?}", clk+1, outputs);
        }

        outputs
    }
}
