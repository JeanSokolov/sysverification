# sysverification
## Repoübersicht
- sys_verifikation.exe - executable
- ordner /src/:
  - parser.rs: source code des parsers
  - main.rs: source code der good- & bad-circuit simulation
- *.v dateien - benchmarks
- logs.json:
  - beispiel des programmoutputs mit dem benchmark ethernet_synth.v

## Usage
- sys_verifikation.exe herunterladen
- ausführen in PowerShell oder cmd mit ./sys_verifikation.exe [verilog_datei.v]
- z.B.: ./sys_verifikation.exe ethernet_synth.v
- im working directory wird eine datei logs.json erstellt
- inhalt von logs.json:
  - generierte inputwerte
  - ergebnisse der good circuit simulation
  - generierter stuck-at fehler
  - ergebnisse der bad circuit simulation

## Kompilierprozess
- rust compiler "cargo" installieren (https://www.rust-lang.org/tools/install)
- repository klonen
- zum kompilieren im repository "cargo build -r" für release build und "cargo build" für debug build
- die kompilierte .exe datei wird im ordner ./target/release/ oder ./target/debug/ abgelegt 

## todo:

|  Status |  Aufgabe  |
|---|---|
| X | Parser |
| X | GoodCircuit Sim |
| X | BadCircuit Sim |
