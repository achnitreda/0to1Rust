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

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(format!("")),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(format!("")),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            _ => todo!(),
        }
    }
}

impl FromStr for BloodType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen_str, rh_str) = if s.starts_with("AB") {
            ("AB", &s[2..])
        } else {
            (&s[0..1], &s[1..])
        };
        
        let antigen = antigen_str.parse()?;
        let rh_factor = rh_str.parse()?;
        
        Ok(BloodType { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::AB => "AB",
            Antigen::B => "B",
            Antigen::O => "O",
        };
        
        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        
        write!(f, "{}{}", antigen_str, rh_str)
    }
}

impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
		let rh_compatible = match (&self.rh_factor, &other.rh_factor) {
			(RhFactor::Positive,_) => true,
			(RhFactor::Negative, RhFactor::Negative) => true,
			(RhFactor::Negative, RhFactor::Positive) => false,
		};

		if !rh_compatible {
			return false
		}

		match (&self.antigen,&other.antigen) {
			(_,Antigen::O) => true,
			(Antigen::AB,_) => true,
			(Antigen::A,Antigen::A) => true,
			(Antigen::B,Antigen::B) => true,

			(Antigen::A,Antigen::B) => false,
			(Antigen::A,Antigen::AB) => false,
			(Antigen::B,Antigen::A) => false,
			(Antigen::B,Antigen::AB) => false,
			(Antigen::O,_) => false,
		}
	}

	pub fn donors(&self) -> Vec<Self> {
		let other_antigen = [Antigen::A,Antigen::B,Antigen::AB,Antigen::O];
		let other_rh = [RhFactor::Positive,RhFactor::Negative];

		let mut res = vec![];

		for v in &other_antigen {
			for rh in &other_rh {
				let other = BloodType{antigen: v.clone(), rh_factor: rh.clone()};
				if self.can_receive_from(&other) {
					res.push(other);
				}
			}
		}
		res
	}

	pub fn recipients(&self) -> Vec<Self> {
		let other_antigen = [Antigen::A,Antigen::B,Antigen::AB,Antigen::O];
		let other_rh = [RhFactor::Positive,RhFactor::Negative];

		let mut res = vec![];

		for v in &other_antigen {
			for rh in &other_rh {
				let other = BloodType{antigen: v.clone(), rh_factor: rh.clone()};
				if other.can_receive_from(&self) {
					res.push(other);
				}
			}
		}
		res
	}
}
