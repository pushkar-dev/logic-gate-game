
use std::collections::HashMap;
use crate::gates::logic_gates::LogicGate;
use rayon::prelude::*;

pub enum RunType {
    SEQUENTIAL,
    PARALLEL
}

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
    pins: Vec<bool>,
    output_map: HashMap<usize, usize>,
    op_pin_size: usize
}

impl Circuit {
    pub fn new() -> Self {
        let mut c = Circuit {
            components: Vec::new(),
            pins: Vec::new(),
            output_map: HashMap::new(),
            op_pin_size: 0
        };
        c.add_input(false);
        c.add_input(true);
        c
    }

    pub fn add_input(&mut self, input: bool) {
        self.pins.push(input);
    }

    pub fn add_gate(&mut self, gate: LogicGate, input_indices: Vec<usize>, output: Option<usize>) {
        
        let output_index= match output{
            Some(index)=>{
                index
            }
            None=>{
                self.op_pin_size += 1;
                self.components.len()+self.pins.len()

            }
        };
        
        let component = Component{
            gate,
            input_indices,
            output_index
        };
        self.output_map.insert( component.output_index, self.components.len());
        self.components.push(component);
    }


    fn get_component_output(&self, idx: usize)->Option<&Component>{
        match self.output_map.get(&idx){
            Some(&val) =>{
                return Some(&self.components[val]);
            }
            None =>{
                None
            }
        }
        
    }

    pub fn evaluate(&self, cycles: usize, run_type: RunType) -> Vec<bool> {
        let mut outputs = vec![false; self.pins.len() + self.op_pin_size];

        // Initialize the outputs with initial pins
        outputs[..self.pins.len()].copy_from_slice(&self.pins);
        
        for clk in 0..cycles-1 {
            let mut new_outputs = outputs.clone(); // start a new transaction at each cycle

            
            
            
            match run_type{
                RunType::PARALLEL =>{
                    // paraller execute logic
                    new_outputs.par_iter_mut().enumerate().for_each(|(idx, val)|{
                        let component = self.get_component_output(idx);
                        match component {
                            Some(component)=>{
                                println!("\nPropogating gate {}", component.gate.info());
                                *val = component.fire(&outputs);

                            }
                            None =>{
                                println!("Skipping Invalid gate");
                            }
                        }
                    });
                }
                RunType::SEQUENTIAL =>{
                    // Sequential Execute logic
                    for component in &self.components{
                        
                        println!("\nPropogating gate {}", component.gate.info());
                        new_outputs[component.output_index] = component.fire(&outputs);
                        
                    };
                }
            }

            outputs = new_outputs; // only commit after one cycle
            println!("---------- {} cycles complete ------------ : evaluated to {:?}", clk+1, outputs);
        }

        outputs
    }
}
