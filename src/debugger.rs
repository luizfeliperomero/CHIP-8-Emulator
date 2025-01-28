use std::str::FromStr;

pub const HELP_MESSAGE: &str = r#"
Debugger Commands Help

General Commands:
- `step`: Execute a single CPU cycle and print the current cycle state.
  Example: step

- `run`: Run the emulator continuously at 500 Hz (2 ms per cycle). Exit the loop as if you were closing the window.
  Example: run

- `quit`: Exit the debugger and stop the emulator.
  Example: quit

- `help`: Display this help message.
  Example: help

Show Commands:
The `show` command is used to display the value of specific registers, memory locations, or other components.
Usage: show <arg>

Arguments for `show`:
- `pc`: Print the current value of the Program Counter (PC).
  Example: show pc

- `mem <addr>`: Print the value at the specified memory address.
  Example: show mem 0x200

- `stack <addr>`: Print the value at the specified stack address.
  Example: show stack 0x0

- `sp`: Print the current value of the Stack Pointer (SP).
  Example: show sp

- `v <n>`: Print the value of the specified V register (where `n` is a register index, e.g., `0` to `F`).
  Example: show v 0

- `i`: Print the current value of the Index Register (I).
  Example: show i

- `dt`: Print the current value of the Delay Timer (DT).
  Example: show dt

- `st`: Print the current value of the Sound Timer (ST).
  Example: show st

- `waiting_key`: Print whether the emulator is waiting for a key press.
  Example: show waiting_key

Notes:
- All numeric values (e.g., memory addresses, register indices) are in hexadecimal format.
- Use `help` to display this message at any time.
"#;

pub enum ShowArgs {
    PC,
    Mem(usize),
    Stack(usize),
    SP,
    V(u8),
    I,
    DT,
    ST,
    WaitingKey,
}

impl FromStr for ShowArgs {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();
        match input {
            "pc" => Ok(Self::PC),
            input if input.starts_with("mem") => {
                if let Some(address) = input.strip_prefix("mem").map(|s| s.trim()) {
                    match usize::from_str_radix(address, 16) {
                        Ok(addr) if addr <= 0x1000 => Ok(Self::Mem(addr)),
                        Ok(_) => Err("Memory address must be between 0 and 0x1000".to_string()),
                        Err(_) => Err("Invalid Address".to_string()),
                    }
                } else {
                    Err("You must provide an address".to_string())
                }
            }
            input if input.starts_with("stack") => {
                if let Some(address) = input.strip_prefix("stack").map(|s| s.trim()) {
                    match usize::from_str_radix(address, 16) {
                        Ok(addr) if addr <= 0x10 => Ok(Self::Stack(addr)),
                        Ok(_) => Err("Stack address must be between 0 and 0x10".to_string()),
                        Err(_) => Err("Invalid Address".to_string()),
                    }
                } else {
                    Err("You must provide an address".to_string())
                }
            }
            "sp" => Ok(Self::SP),
            input if input.starts_with("v") => {
                if let Some(register_num) = input.strip_prefix("v") {
                    match register_num.parse::<u8>() {
                        Ok(n) if n <= 15 => Ok(Self::V(n)),
                        Ok(_) => Err("v registers ranges start from 0 to 15".to_string()),
                        Err(_) => Err("Invalid value for register v".to_string()),
                    }
                } else {
                    Err("Missing register number".to_string())
                }
            }
            "i" => Ok(Self::I),
            "dt" => Ok(Self::DT),
            "st" => Ok(Self::ST),
            "waiting_key" | "wk" => Ok(Self::WaitingKey),
            _ => Err(format!("No such arg: {input}").to_string()),
        }
    }
}

pub enum DebuggerAction {
    Step,
    Show(ShowArgs),
    Run,
    Quit,
    Help,
}

impl FromStr for DebuggerAction {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();
        match input {
            "step" => Ok(Self::Step),
            input if input.starts_with("show") => {
                if let Some(arg) = input.strip_prefix("show") {
                    if let Ok(show_arg) = ShowArgs::from_str(arg) {
                        Ok(Self::Show(show_arg))
                    } else {
                        Err(format!("No such arg: {arg} for show").to_string())
                    }
                } else {
                    Err("No args provided".to_string())
                }
            }
            "run" => Ok(Self::Run),
            "quit" => Ok(Self::Quit),
            "help" => Ok(Self::Help),
            _ => Err(format!("No such arg: {input}").to_string()),
        }
    }
}
