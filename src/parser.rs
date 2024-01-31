use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter,
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
#[derive(Debug, Clone, PartialEq)]
pub struct Gate {
    name: String,
    kind: i8,
    input: Vec<Option<Gate>>,
    //    output: Option<Vec<Gate>>,
    inverted_input: Option<Vec<bool>>,
    stuck_at: Option<Vec<bool>>,
    value: Option<bool>,
    op: Option<bool>,
}
impl Gate {
    fn new(gate_name: String, gate_type: i8) -> Gate {
        Gate {
            name: gate_name,
            kind: gate_type,
            input: vec![None],
            //           output: None,
            inverted_input: None,
            stuck_at: None,
            value: None,
            op: None,
        }
    }
}

pub fn parse(file: BufReader<File>) -> HashMap<String, Gate> {
    let mut network: HashMap<String, Gate> = HashMap::new();
    let mut parsing_flags: Vec<bool> = vec![false, false, false, false];
    let mut inputs = String::new();
    let mut outputs = String::new();
    let mut wires = String::new();
    let mut assignments = String::new();

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
        if content.contains("endmodule") {
            break;
        }
        if parsing_flags[0] {
            inputs += &content
                .replace("input", "")
                .replace(" ", "")
                .replace(";", "");
        }
        if parsing_flags[1] {
            outputs += &content
                .replace("output", "")
                .replace(" ", "")
                .replace(";", "");
        }
        if parsing_flags[2] {
            wires += &content
                .replace("wire", "")
                .replace(" ", "")
                .replace(";", "");
        }
        if parsing_flags[3] {
            assignments += &content.replace("assign", "").replace(" ", "");
        }
    }

    let mut _in_str: Vec<&str> = inputs.split(',').collect();

    for i in _in_str.clone() {
        network.insert(i.to_string(), Gate::new(i.to_owned(), 0));
    }
    network.insert("1'b1".to_string(), Gate::new("1'b1".to_string(), 0));

    let mut _wire_str: Vec<&str> = wires.split(",").collect();
    let mut _out_str: Vec<&str> = outputs.split(",").collect();
    let mut _assign_str: Vec<&str> = assignments.split(";").collect();
    _assign_str.pop();

    for assignment in _assign_str {
        let equation: Vec<&str> = assignment.clone().split("=").collect();
        let mut input: Vec<&str> = Vec::new();
        let mut gate_inputs: Vec<Option<Gate>> = Vec::new();
        let mut inverted_inputs: Option<Vec<bool>> = None;
        let mut op: Option<bool> = None;
        if equation[1].contains(&['&', '|'][..]) {
            op = Some(equation[1].contains('&'));
            input = equation[1].split(&['&', '|'][..]).collect();
            inverted_inputs = Some(vec![input[0].contains('~'), input[1].contains('~')]);
            let input_0 = network.get(&input[0].replace("~", ""));
            let input_1 = network.get(&input[1].replace("~", ""));
            gate_inputs.push(input_0.cloned());
            gate_inputs.push(input_1.cloned());
        } else {
            inverted_inputs = Some(vec![equation[1].contains('~')]);
            let input_0 = network.get(&equation[1].replace("~", ""));
            gate_inputs.push(input_0.cloned());
        }
        if gate_inputs.contains(&None) {
        } else {
            network.insert(
                equation[0].to_string(),
                Gate {
                    name: equation[0].to_string(),
                    kind: 1,
                    input: gate_inputs,
                    //                output: (),
                    inverted_input: inverted_inputs,
                    stuck_at: None,
                    value: None,
                    op: op,
                },
            );
        }
        // Implement network.push(new_gate)
        // println!("{}", ind);
        // println!("{:?},{:?},\n{}", index_input_0, index_input_1, assignment);
    }

    println!("\ndone");
    network
}
