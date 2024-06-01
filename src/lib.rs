mod gates;

#[cfg(test)]
mod tests{
    use crate::gates::{circuit::{Circuit, RunType}, logic_gates::LogicGate};


    #[test]
    fn test_and(){
        let mut cqt = Circuit::new();
        cqt.add_input(true);
        cqt.add_gate(LogicGate::AndGate, vec![0,1], None);
        cqt.add_gate(LogicGate::AndGate, vec![1,2], None);

        let opt= cqt.evaluate(3, RunType::PARALLEL);
        assert_eq!(opt, vec![false,true,true,false,true]);

    }

    #[test]
    fn test_or(){
        let mut cqt = Circuit::new();
        cqt.add_input(false);
        cqt.add_gate(LogicGate::OrGate, vec![0,1], None);
        cqt.add_gate(LogicGate::OrGate, vec![1,2], None);

        let opt= cqt.evaluate(3, RunType::PARALLEL);
        assert_eq!(opt, vec![false,true,false,true,true]);

    }


}