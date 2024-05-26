pub enum LogicGate {
    AndGate,
    OrGate,
    NotGate,
}

impl LogicGate {
    pub fn evaluate(&self, inputs: &[bool]) -> bool {
        match self {
            LogicGate::AndGate => inputs.iter().all(|&x| x),
            LogicGate::OrGate => inputs.iter().any(|&x| x),
            LogicGate::NotGate => !inputs[0],
        }
    }
    pub fn info(&self) -> &str{
        match self{
            LogicGate::AndGate => "AndGate",
            LogicGate::OrGate => "OrGate",
            LogicGate::NotGate => "NotGate",

        }
    }
}
