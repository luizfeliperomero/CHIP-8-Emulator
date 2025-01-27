use std::str::FromStr;

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
            _ => Err(format!("No such arg: {input}").to_string()),
        }
    }
}
