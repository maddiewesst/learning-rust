use std::collections::HashMap;
use std::num::ParseFloatError;

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        Flag {
            short_hand: "-".to_string() + &l_h.chars().nth(0).unwrap().to_string(),
            long_hand: "--".to_string() + l_h,
            desc: d.to_string(),
        }
    }
}

// let d = Flag::opt_flag("diff", "gives the difference between two numbers");

// println!("short hand: {}, long hand: {}, description: {}", d.short_hand, d.long_hand, d.desc);
// output: "short hand: -d, long hand: --diff, description: gives the difference between two numbers"

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }

    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        match self.flags.get(&flag) {
            Some(func) => match func(argv[0], argv[1]) {
                Ok(result) => result,
                Err(error) => error.to_string(),
            },
            None => format!("Flag {:?} not found", flag),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let divided = a.parse::<f32>()? / b.parse::<f32>()?;
    Ok(divided.to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let remainder = a.parse::<f32>()? % b.parse::<f32>()?;
    Ok(remainder.to_string())
}
