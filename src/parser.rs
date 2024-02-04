use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter,
    sync::{Arc, Mutex},
};

/// Parser for .v circuit benchmark files
///
///
///
///

/// Gate struct
/// Input & output are Vec as amount
/// NOT as bool
/// op as bool, 0 - or, 1 - and
#[derive(Debug, Clone)]
pub struct Gate {
    pub name: String,
    pub kind: i8,
    pub input: Vec<Option<Arc<Mutex<Gate>>>>,
    //    output: Option<Vec<Gate>>,
    pub inverted_input: Option<Vec<bool>>,
    pub stuck_at: Vec<Option<bool>>,
    pub value: Option<bool>,
    pub op: Option<bool>,
}
impl Gate {
    fn new(gate_name: String, gate_type: i8) -> Gate {
        Gate {
            name: gate_name,
            kind: gate_type,
            input: vec![None],
            //           output: None,
            inverted_input: None,
            stuck_at: vec![None],
            value: None,
            op: None,
        }
    }
    pub fn evaluate(&self) -> Option<bool> {
        let mut result = false;
        if let inputs = &self.input {
            let mut input_gates = inputs.iter();
            if let Some(Some(input_gate_0_tmp)) = input_gates.next() {
                //println!("{:?}", input_gate_0_tmp);
                let input_gate_0 = input_gate_0_tmp.lock().unwrap(); // Locking the Mutex
                                                                     //println!("{:?}", input_gate_0.value);
                let mut input_value_0 = input_gate_0.value.unwrap();
                if self.inverted_input.as_ref().unwrap()[0] {
                    input_value_0 = !input_value_0;
                }
                if self.stuck_at[0].is_some() {
                    input_value_0 = self.stuck_at[0].unwrap();
                }
                // if two gates form input
                if let Some(Some(input_gate_1)) = input_gates.next() {
                    let input_gate_1 = input_gate_1.lock().unwrap(); // Locking the Mutex
                    let mut input_value_1 = input_gate_1.value.unwrap();
                    if self.inverted_input.as_ref().unwrap()[1] {
                        input_value_1 = !input_value_1;
                    }
                    if self.stuck_at[1].is_some() {
                        input_value_1 = self.stuck_at[1].unwrap();
                    }
                    let op = self.op.unwrap();
                    if op {
                        result = input_value_0 && input_value_1;
                    } else {
                        result = input_value_0 || input_value_1;
                    }
                } else {
                    result = input_value_0;
                }
            }
        }

        Some(result)
    }
}

pub fn parse(
    file: BufReader<File>,
) -> (HashMap<String, Arc<Mutex<Gate>>>, Vec<String>, Vec<String>) {
    let mut network: HashMap<String, Arc<Mutex<Gate>>> = HashMap::new();
    let mut parsing_flags: Vec<bool> = vec![false, false, false, false];
    let mut gates: Vec<String> = Vec::new();
    let mut ret_outputs: Vec<String> = Vec::new();
    let mut inputs = String::new();
    let mut outputs = String::new();
    let mut wires = String::new();
    let mut assignments = String::new();

    // parse file contents, divide it into inputs, outputs, wires and assignments
    for lines in file.lines() {
        let content = lines.unwrap();
        if content.contains("input") && !parsing_flags[0] {
            parsing_flags[0] = true;
        }
        if content.contains("output") && !parsing_flags[1] {
            parsing_flags[0] = false;
            parsing_flags[1] = true;
        }
        if content.contains("wire") && !parsing_flags[2] {
            parsing_flags[1] = false;
            parsing_flags[2] = true;
        }
        if content.contains("assign") && !parsing_flags[3] {
            parsing_flags[2] = false;
            parsing_flags[3] = true;
        }
        if content.contains("endmodule") || content.eq("end;") {
            break;
        }
        if parsing_flags[0] {
            inputs += &content
                .replace("input", "")
                .replace(" ", "")
                .replace(";", ",");
        }
        if parsing_flags[1] {
            outputs += &content
                .replace("output", "")
                .replace(" ", "")
                .replace(";", ",");
        }
        if parsing_flags[2] {
            wires += &content
                .replace("wire", "")
                .replace(" ", "")
                .replace(";", ",");
        }
        if parsing_flags[3] {
            assignments += &content.replace("assign", "").replace(" ", "");
        }
    }

    // initialize input gates
    let mut _in_str: Vec<&str> = inputs.split(',').collect();
    _in_str.pop();
    for i in _in_str.clone() {
        let new_gate = Arc::new(Mutex::new(Gate::new(i.to_owned(), 0)));
        network.insert(i.to_string(), Arc::clone(&new_gate));
    }

    let constant_gate = Arc::new(Mutex::new(Gate::new("1'b1".to_string(), 0)));
    network.insert("1'b1".to_string(), Arc::clone(&constant_gate));

    let mut _wire_str: Vec<&str> = wires.split(",").collect();
    let mut _out_str: Vec<&str> = outputs.split(",").collect();
    let mut _assign_str: Vec<&str> = assignments.split(";").collect();
    _wire_str.pop();
    _out_str.pop();
    _assign_str.pop();

    // initialize wire- and output gates based on assigments
    while !_assign_str.is_empty() {
        let mut ind_vec: Vec<usize> = Vec::new();
        for (ind, assignment) in _assign_str.iter().enumerate() {
            let equation: Vec<&str> = assignment.clone().split("=").collect();
            let mut input: Vec<&str> = Vec::new();
            let mut gate_inputs: Vec<Option<Arc<Mutex<Gate>>>> = Vec::new();
            let mut inverted_inputs: Option<Vec<bool>> = None;
            let mut op: Option<bool> = None;
            // determine operation type and split assignment into lhs and rhs, then split rhs into inputs
            if equation[1].contains(&['&', '|'][..]) {
                op = Some(equation[1].contains('&'));
                input = equation[1].split(&['&', '|'][..]).collect();
                inverted_inputs = Some(vec![input[0].contains('~'), input[1].contains('~')]);
                let input_0 = network.get(&input[0].replace("~", ""));
                let input_1 = network.get(&input[1].replace("~", ""));
                gate_inputs.push(input_0.map(|gate| Arc::clone(&gate)));
                gate_inputs.push(input_1.map(|gate| Arc::clone(&gate)));
            } else {
                inverted_inputs = Some(vec![equation[1].contains('~')]);
                let input_0 = network.get(&equation[1].replace("~", ""));
                gate_inputs.push(input_0.map(|gate| Arc::clone(&gate)));
            }
            // initialize gates in assignment if all input gates (rhs values) exist in the network
            if gate_inputs.iter().any(|input| input.is_none()) {
            } else {
                let new_gate = Arc::new(Mutex::new(Gate {
                    name: equation[0].to_string(),
                    kind: 1,
                    input: gate_inputs,
                    inverted_input: inverted_inputs,
                    stuck_at: vec![None, None],
                    value: None,
                    op: op,
                }));
                network.insert(equation[0].to_string(), Arc::clone(&new_gate));
                gates.push(equation[0].to_string());
                ind_vec.push(ind);
            }
        }
        // remove assignments that were successfully parsed
        ind_vec.reverse();
        for i in ind_vec.clone() {
            _assign_str.remove(i);
        }
    }
    for i in _out_str {
        ret_outputs.push(i.to_owned());
    }
    println!("\nDone parsing!");
    return (network, gates, ret_outputs);
}
