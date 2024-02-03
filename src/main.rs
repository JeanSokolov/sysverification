use parser::*;
use rand::Rng;
use std::{fs::File, io::BufReader};
mod parser;

fn main() -> std::io::Result<()> {
    let num_simulation_cycles = 4;

    let filepath = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./C17_orig_.v".to_owned());
    let f = File::open(filepath)?;
    let reader = BufReader::new(f);

    let (mut network, mut gates, mut outputs) = parse(reader);

    // generate random input values
    for (_, gate) in network.iter_mut() {
        let mut gate = gate.lock().unwrap(); // Acquire lock
        if gate.input.iter().any(|item| item.is_none()) {
            gate.value = Some(rand::thread_rng().gen::<bool>());
        }
    }

    // Simulation Loop
    //for _ in 0..num_simulation_cycles {
    // Evaluate gates
    for i in gates.clone() {
        let mut gate = network[&i].lock().unwrap(); // Acquire lock
        gate.value = gate.evaluate();
    }

    // Print Outputs
    for (name, gate) in network.iter() {
        let gate = gate.lock().unwrap(); // Acquire lock
                                         //println!("{:?}\n", gate);
        if let Some(value) = gate.value {
            println!("{}: {}", name, value);
        }
    }
    println!("Done processing");

    Ok(())
}
