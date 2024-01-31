use parser::*;
use std::collections::HashMap;
use std::fs::File;
///
///
///
///
use std::io::BufReader;
mod parser;

fn main() -> std::io::Result<()> {
    let filepath = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./vga_lcd_synth.v".to_owned());
    let f = File::open(filepath)?;
    let reader = BufReader::new(f);
    let network = parse(reader);
    Ok(())
}
