use std::cmp::{Ord, Ordering};

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd, Clone)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}
// #[derive(Debug)]
// pub struct ParseError {
//     pub details: String,
// }
// impl ParseError {
//     fn new(msg: &str) -> ParseError {
//         ParseError {
//             details: msg.to_string(),
//         }
//     }
// }
// impl fmt::Display for ParseError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.details)
//     }
// }
// impl Error for ParseError {
//     fn description(&self) -> &str {
//         &self.details
//     }
// }

impl FromStr for Antigen {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
            "A"  => {return Ok(Antigen::A)}, 
            "B"  => {return Ok(Antigen::B)},
            "AB" => {return Ok(Antigen::AB)}
            "O"  => {return Ok(Antigen::O)},
            _    => {return Err("Antigen in wrong format".to_string())}
			
        }
    }

}

impl FromStr for RhFactor {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"-" => Ok(RhFactor::Negative),
			"+" => Ok(RhFactor::Positive),
			_ => Err("Rh Factor in wrong format".to_string()),
		}
	}
}

impl Ord for BloodType {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.antigen == other.antigen {
            self.rh_factor.cmp(&other.rh_factor)
        } else {
            self.antigen.cmp(&other.antigen)
        }
    }
}

impl FromStr for BloodType {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {

        let antigen_str = &s[..s.len() - 1];
        let rh_factor_str = &s[s.len() - 1..];

		let antigen: Antigen = match antigen_str.parse() {
            Ok(resp) => resp,
            Err(e) => {return Err(e)}
        };
        let rh_factor: RhFactor = match rh_factor_str.parse() {
            Ok(resp) => resp,
            Err(e) => {return Err(e)}
        };

        Ok(BloodType {
            antigen,
            rh_factor,
        })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };

        let rh_factor_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };

        write!(f, "{}{}", antigen_str, rh_factor_str)
    }
}

impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
		match (&self.antigen, &self.rh_factor, &other.antigen, &other.rh_factor) {
            (_, RhFactor::Negative, _, RhFactor::Positive) => false,
            (Antigen::A, _, Antigen::B | Antigen::AB, _) => false,
            (Antigen::B, _, Antigen::A | Antigen::AB, _) => false,
            (Antigen::O, _, Antigen::A | Antigen::B | Antigen::AB, _) => false,
            _ => true
        }
	}

	pub fn donors(&self) -> Vec<Self> {
		let mut result: Vec<BloodType> = Vec::new();
		match self.antigen {
			Antigen::A => {
				result.push(BloodType {
					antigen: Antigen::O,
					rh_factor: RhFactor::Negative
				});
				result.push(BloodType {
					antigen: Antigen::A,
					rh_factor: RhFactor::Negative
				});
			},
			Antigen::B => {
				result.push(BloodType {
					antigen: Antigen::O,
					rh_factor: RhFactor::Negative
				});
				result.push(BloodType {
					antigen: Antigen::B,
					rh_factor: RhFactor::Negative
				});
			},
			Antigen::AB => {
				result.push(BloodType {
					antigen: Antigen::AB,
					rh_factor: RhFactor::Negative
				});
				result.push(BloodType {
					antigen: Antigen::O,
					rh_factor: RhFactor::Negative
				});
				result.push(BloodType {
					antigen: Antigen::A,
					rh_factor: RhFactor::Negative
				});
				result.push(BloodType {
					antigen: Antigen::B,
					rh_factor: RhFactor::Negative
				});
			},
			Antigen::O => {
				result.push(BloodType {
					antigen: Antigen::O,
					rh_factor: RhFactor::Negative
				});
			}
		}
		result
		
	}

	pub fn recipients(&self) -> Vec<BloodType> {
		let mut result: Vec<BloodType> = Vec::new();
    
        match self.antigen {
            Antigen::A  => {
                result.push(BloodType {
                    antigen: Antigen::AB,
                    rh_factor: RhFactor::Positive
                });
                result.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Positive
                });
            },
            Antigen::B  => {
                result.push(BloodType {
                    antigen: Antigen::AB,
                    rh_factor: RhFactor::Positive
                });
                result.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Positive
                });
            },
            Antigen::AB => {
                result.push(BloodType {
                    antigen: Antigen::AB,
                    rh_factor: RhFactor::Positive
                });
            },
            Antigen::O  => {
                result.push(BloodType {
                    antigen: Antigen::AB,
                    rh_factor: RhFactor::Positive
                });
                result.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Positive
                });
                result.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Positive
                });
                result.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Positive
                });
            }
        }
		if self.rh_factor == RhFactor::Negative {
            for mut blood in result.clone() {
                blood.rh_factor = RhFactor::Negative;
                result.push(blood)
            }
        }
		result
	}
}



// _    => {return Err(ParseError::new("Antigen in wrong format"))}