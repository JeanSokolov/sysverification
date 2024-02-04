use json;
use parser::*;
use rand::Rng;
use std::{
    fs::File,
    io::{BufReader, Write},
};
mod parser;

fn main() -> std::io::Result<()> {
    let num_simulation_cycles = 4;
    let mut data = json::JsonValue::new_object();

    let filepath = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./ethernet_synth.v".to_owned());
    let f = File::open(filepath)?;
    let reader = BufReader::new(f);

    let mut gcs_names: Vec<String> = Vec::new();
    let mut gcs_values: Vec<Option<bool>> = Vec::new();
    let mut stuck_ats: Vec<String> = Vec::new();
    let (mut network, mut gates, mut outputs) = parse(reader);
    let mut iteration_input_values: Vec<bool> = Vec::new();

    // generate random input values
    for (_, gate) in network.iter_mut() {
        let mut gate = gate.lock().unwrap(); // Acquire lock
        if gate.input.iter().any(|item| item.is_none()) {
            let rdm = rand::thread_rng().gen::<bool>();
            iteration_input_values.push(rdm);
            gate.value = Some(rdm);
            data["test_pattern"][&gate.name] = (rdm as i32).into();
        }
    }

    // good circuit simulation loop
    for i in gates.clone() {
        let mut gate = network[&i].lock().unwrap(); // Acquire lock
        gate.value = gate.evaluate();
        gcs_values.push(gate.value.clone());
        gcs_names.push(gate.name.clone());
        data["gcs"][&gate.name] = (gate.value.unwrap() as i32).into();
    }

    for n in 0..num_simulation_cycles {
        let mut bcs_values: Vec<Option<bool>> = Vec::new();
        // Setting stuck at
        for (_, gate) in network.iter_mut() {
            let mut unlocked_gate = gate.lock().unwrap();
            if unlocked_gate.kind.eq(&1) {
                if rand::thread_rng().gen::<bool>() {
                    if !stuck_ats.contains(&unlocked_gate.name) {
                        let rdm = rand::thread_rng().gen::<bool>();
                        println!(
                            "Setting stuck-at for input[0] at {} for Gate: {}",
                            rdm as i32, unlocked_gate.name
                        );
                        unlocked_gate.stuck_at = vec![Some(rdm), None];
                        stuck_ats.push(unlocked_gate.name.clone());
                        data[&format!("bcs_iteration_{}", n)]["stuck_at"] =
                            (format!("input[0]={} for Gate: {}", rdm as i32, unlocked_gate.name))
                                .into();
                        break;
                    }
                }
            }
        }
        // bad circuit simulation loop
        for i in gates.clone() {
            let mut gate = network[&i].lock().unwrap(); // Acquire lock
            gate.value = gate.evaluate();
            bcs_values.push(gate.value.clone());
        }

        // If results of good circuit simulation are different from results of bad circuit simulation, output all values that differ
        if gcs_values.ne(&bcs_values) {
            for i in 0..gcs_names.clone().len() {
                if gcs_values[i].ne(&bcs_values[i]) {
                    println!(
                        "GATE: {}, GCS: {:?}, BCS: {:?}, is_output: {:?}",
                        gcs_names[i],
                        gcs_values[i],
                        bcs_values[i],
                        outputs.contains(&gcs_names[i])
                    );
                }
                data[&format!("bcs_iteration_{}", n)][&gcs_names[i]] =
                    (bcs_values[i].unwrap() as i32).into();
            }
        } else {
            data[&format!("bcs_iteration_{}", n)]["result"] = ("same as gcs").into();
            println!("Stuck-at of iteration: {} had no impact", n);
        }

        println!("\n-------------------------\n");

        // clear stuck at
        network[stuck_ats.last().unwrap()].lock().unwrap().stuck_at = vec![None, None];

        // Print Outputs
        /* for (name, gate) in network.iter() {
            let gate = gate.lock().unwrap(); // Acquire lock
            if let Some(value) = gate.value {
                println!("{}: {}", name, value);
            }
        } */
        //println!("{:?}\n {:?}", gcs_names, gcs_values);

        //println!("\n-------------------------\n");
    }
    println!("Done processing");
    //println!("{}", data.dump());

    // write inputs, results of good circuit sim, stuck-ats and bad circuit sims to .json file
    let mut file = File::create("./logs.json")?;
    file.write_all(data.dump().as_bytes())?;

    Ok(())
}
