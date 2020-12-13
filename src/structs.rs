use std::fmt;
#[derive(Clone)]
#[derive(Debug)]
pub enum EsianolopInstruction {
    Nul,
    Add(Box<EsianolopInstruction>,Box<EsianolopInstruction>),
    Mul(Box<EsianolopInstruction>,Box<EsianolopInstruction>),
    Sub(Box<EsianolopInstruction>,Box<EsianolopInstruction>),
    Div(Box<EsianolopInstruction>,Box<EsianolopInstruction>),
    Pow(Box<EsianolopInstruction>,Box<EsianolopInstruction>),
    DpL(Box<EsianolopInstruction>),
    DpR(Box<EsianolopInstruction>),
    Sqr(Box<EsianolopInstruction>),
    Num(usize),
}

impl fmt::Display for EsianolopInstruction {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        match self {
            EsianolopInstruction::Nul => {write!(f,"Nul")},
            EsianolopInstruction::Add(a,b) => {return write!(f,"Add({},{})",a,b)},
            EsianolopInstruction::Sub(a,b) => {return write!(f,"Sub({},{})",a,b)},
            EsianolopInstruction::Mul(a,b) => {return write!(f,"Mul({},{})",a,b)},
            EsianolopInstruction::Div(a,b) => {return write!(f,"Div({},{})",a,b)},
            EsianolopInstruction::Pow(a,b) => {return write!(f,"Pow({},{})",a,b)},
            EsianolopInstruction::DpL(a) => {return write!(f,"DpL({})",a)},
            EsianolopInstruction::DpR(a) => {return write!(f,"DpR({})",a)},
            EsianolopInstruction::Sqr(a) => {return write!(f,"Sqr({})",a)},
            EsianolopInstruction::Num(a) => {return write!(f,"Num({})",a)},
        }
    }
}


pub struct Esianolop {
    pub values:Vec<EsianolopInstruction>,
}


impl Esianolop {

    pub fn new() -> Esianolop {
        Esianolop {
            values:vec![],
        }
    }


    
    pub fn parse_text(&mut self,text:&str) -> Result<(),String> {
        
        for (line_nb,line) in text.split("\n").map(|x| x.to_owned()).enumerate() {
            for (ins_nb, instruction) in line.split(";").collect::<Vec<&str>>()[0].split("#").collect::<Vec<&str>>()[0].split(" ").filter(|x| x.to_owned().trim() != "" ).enumerate() {
                match instruction.to_ascii_lowercase().as_str() {
                    "+" | "add" => {
                        let vals = match self.values.get(0..2) {
                            Some(e) => e,
                            None => return Err(format!("Error at {}:{}, not enogth values in buffer to add.",line_nb,ins_nb))
                        };
                        let t = EsianolopInstruction::Add(Box::new(vals[0].to_owned()),Box::new(vals[1].to_owned()));
                        self.values.drain(1..2);
                        self.values[0] = t;
                    }
                    "-" | "sub" => {
                        let vals = match self.values.get(0..2) {
                            Some(e) => e,
                            None => return Err(format!("Error at {}:{}, not enogth values in buffer to substract.",line_nb,ins_nb))
                        };
                        let t = EsianolopInstruction::Sub(Box::new(vals[0].to_owned()),Box::new(vals[1].to_owned()));
                        self.values.drain(1..2);
                        self.values[0] = t;
                    }
                    "*" | "mul" => {
                        let vals = match self.values.get(0..2) {
                            Some(e) => e,
                            None => return Err(format!("Error at {}:{}, not enogth values in buffer to multiply.",line_nb,ins_nb))
                        };
                        let t = EsianolopInstruction::Mul(Box::new(vals[0].to_owned()),Box::new(vals[1].to_owned()));
                        self.values.drain(1..2);
                        self.values[0] = t;
                    }
                    "^" | "pow" => {
                        let vals = match self.values.get(0..2) {
                            Some(e) => e,
                            None => return Err(format!("Error at {}:{}, not enogth values in buffer to powerize.",line_nb,ins_nb))
                        };
                        let t = EsianolopInstruction::Pow(Box::new(vals[0].to_owned()),Box::new(vals[1].to_owned()));
                        self.values.drain(1..2);
                        self.values[0] = t;
                    }
                    "/" | "div" => {
                        let vals = match self.values.get(0..2) {
                            Some(e) => e,
                            None => return Err(format!("Error at {}:{}, not enogth values in buffer to divide.",line_nb,ins_nb))
                        };
                        let t = EsianolopInstruction::Div(Box::new(vals[0].to_owned()),Box::new(vals[1].to_owned()));
                        self.values.drain(1..2);
                        self.values[0] = t;
                    }
                    "<" | "dpl" | "d" => {
                        let temp = self.values.get(0);
                        let val = match temp {
                            Some(e) => e,
                            None => return Err(format!("Error at {}:{}, no value in buffer to do duplicate left.",line_nb,ins_nb))
                        };
                        self.values.push(EsianolopInstruction::DpL(Box::new(val.clone())));
                    }
                    ">" | "dpr" => {
                        let temp = self.values.get(self.values.len()-1);
                        let val = match temp {
                            Some(e) => e,
                            None => return Err(format!("Error at {}:{}, no value in buffer to do duplicate right.",line_nb,ins_nb))
                        };
                        self.values.push(EsianolopInstruction::DpR(Box::new(val.clone())));
                    }
                    "$" | "sqr" => {
                        let temp = self.values.get(0);
                        let val = match temp {
                            Some(e) => e,
                            None => return Err(format!("Error at {}:{}, no value in buffer to take the square root.",line_nb,ins_nb))
                        };
                        self.values.push(EsianolopInstruction::DpR(Box::new(val.clone())));
                    }
                    ins => {
                        match ins.to_owned().parse::<usize>() {
                            Ok(e) => {self.values.push(EsianolopInstruction::Num(e as usize))}
                            Err(_) => return Err(format!("Error at {}:{}, not a valid expression.",line_nb,ins_nb))
                        }

                    }
                }
                //println!("{}:{} - {} Ok()",line_nb,ins_nb,instruction);
            }
        }
        Ok(())
    }

}