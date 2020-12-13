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
    Dup(Box<EsianolopInstruction>),
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
            EsianolopInstruction::Dup(a) => {return write!(f,"Dup({})",a)},
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
            for (ins_nb, mut instruction) in line.split(";").collect::<Vec<&str>>()[0].split("#").collect::<Vec<&str>>()[0].split(" ").filter(|x| x.to_owned().trim() != "" ).enumerate() {
                let mut vec_from_down = true;
                if instruction.chars().nth(0) == Some('<') || instruction.chars().nth(0) == Some('>') {
                    vec_from_down = instruction.chars().nth(0) != Some('>');
                    instruction = match instruction.chars().next().map(|c| &instruction[c.len_utf8()..]) {
                        Some(e) => e,
                        None => {return Err(format!("Error at {}:{}, Empty instruction with only stack orientation specified",line_nb,ins_nb))}
                    }
                }
                match instruction.to_ascii_lowercase().as_str() {
                    "+" | "add" => {
                        match vec_from_down {
                            true => {
                                let vals = match self.values.get(0..2) {
                                    Some(e) => e,
                                    None => return Err(format!("Error at {}:{}, not enogth values in buffer to add.",line_nb,ins_nb))
                                };
                                let t = EsianolopInstruction::Add(Box::new(vals[0].to_owned()),Box::new(vals[1].to_owned()));
                                self.values.drain(1..2);
                                self.values[0] = t;
                            },
                            false => {
                                let vals = match self.values.get((self.values.len()-2)..(self.values.len())) {
                                    Some(e) => e,
                                    None => return Err(format!("Error at {}:{}, not enogth values in buffer to add.",line_nb,ins_nb))
                                };
                                let t = EsianolopInstruction::Add(Box::new(vals[0].to_owned()),Box::new(vals[1].to_owned()));
                                self.values.drain((self.values.len()-2)..(self.values.len()));
                                self.values.push(t);
                            }
                        };
                    }
                    "-" | "sub" => {
                        match vec_from_down {
                            true => {
                                let vals = match self.values.get(0..2) {
                                    Some(e) => e,
                                    None => return Err(format!("Error at {}:{}, not enogth values in buffer to substract.",line_nb,ins_nb))
                                };
                                let t = EsianolopInstruction::Sub(Box::new(vals[0].to_owned()),Box::new(vals[1].to_owned()));
                                self.values.drain(1..2);
                                self.values[0] = t;
                            },
                            false => {
                                let vals = match self.values.get((self.values.len()-2)..(self.values.len())) {
                                    Some(e) => e,
                                    None => return Err(format!("Error at {}:{}, not enogth values in buffer to substract.",line_nb,ins_nb))
                                };
                                let t = EsianolopInstruction::Sub(Box::new(vals[0].to_owned()),Box::new(vals[1].to_owned()));
                                self.values.drain((self.values.len()-2)..(self.values.len()));
                                self.values.push(t);
                            }
                        };
                    }
                    "*" | "mul" => {
                        match vec_from_down {
                            true => {
                                let vals = match self.values.get(0..2) {
                                    Some(e) => e,
                                    None => return Err(format!("Error at {}:{}, not enogth values in buffer to multiply.",line_nb,ins_nb))
                                };
                                let t = EsianolopInstruction::Mul(Box::new(vals[0].to_owned()),Box::new(vals[1].to_owned()));
                                self.values.drain(1..2);
                                self.values[0] = t;
                            },
                            false => {
                                let vals = match self.values.get((self.values.len()-2)..(self.values.len())) {
                                    Some(e) => e,
                                    None => return Err(format!("Error at {}:{}, not enogth values in buffer to multiply.",line_nb,ins_nb))
                                };
                                let t = EsianolopInstruction::Mul(Box::new(vals[0].to_owned()),Box::new(vals[1].to_owned()));
                                self.values.drain((self.values.len()-2)..(self.values.len()));
                                self.values.push(t);
                            }
                        };
                    }
                    "^" | "pow" => {
                        match vec_from_down {
                            true => {
                                let vals = match self.values.get(0..2) {
                                    Some(e) => e,
                                    None => return Err(format!("Error at {}:{}, not enogth values in buffer to powify.",line_nb,ins_nb))
                                };
                                let t = EsianolopInstruction::Pow(Box::new(vals[0].to_owned()),Box::new(vals[1].to_owned()));
                                self.values.drain(1..2);
                                self.values[0] = t;
                            },
                            false => {
                                let vals = match self.values.get((self.values.len()-2)..(self.values.len())) {
                                    Some(e) => e,
                                    None => return Err(format!("Error at {}:{}, not enogth values in buffer to powify.",line_nb,ins_nb))
                                };
                                let t = EsianolopInstruction::Pow(Box::new(vals[0].to_owned()),Box::new(vals[1].to_owned()));
                                self.values.drain((self.values.len()-2)..(self.values.len()));
                                self.values.push(t);
                            }
                        };
                    }
                    "/" | "div" => {
                        match vec_from_down {
                            true => {
                                let vals = match self.values.get(0..2) {
                                    Some(e) => e,
                                    None => return Err(format!("Error at {}:{}, not enogth values in buffer to divide.",line_nb,ins_nb))
                                };
                                let t = EsianolopInstruction::Div(Box::new(vals[0].to_owned()),Box::new(vals[1].to_owned()));
                                self.values.drain(1..2);
                                self.values[0] = t;
                            },
                            false => {
                                let vals = match self.values.get((self.values.len()-2)..(self.values.len())) {
                                    Some(e) => e,
                                    None => return Err(format!("Error at {}:{}, not enogth values in buffer to divide.",line_nb,ins_nb))
                                };
                                let t = EsianolopInstruction::Div(Box::new(vals[0].to_owned()),Box::new(vals[1].to_owned()));
                                self.values.drain((self.values.len()-2)..(self.values.len()));
                                self.values.push(t);
                            }
                        };
                    }
                    "~" | "dup" => {
                        match vec_from_down {
                            false => {
                                let temp = self.values.get(self.values.len()-1);
                                let val = match temp {
                                    Some(e) => e,
                                    None => return Err(format!("Error at {}:{}, no value in buffer to do duplicate.",line_nb,ins_nb))
                                };
                                self.values.push(EsianolopInstruction::Dup(Box::new(val.clone())));
                            },
                            true => {
                                let temp = self.values.get(0);
                                let val = match temp {
                                    Some(e) => e,
                                    None => return Err(format!("Error at {}:{}, no value in buffer to do duplicate.",line_nb,ins_nb))
                                };
                                self.values[0] = EsianolopInstruction::Dup(Box::new(val.clone()));
                            }
                        }
                    }
                    "$" | "sqr" => {
                        match vec_from_down {
                            false => {
                                let temp = self.values.get(self.values.len()-1);
                                let val = match temp {
                                    Some(e) => e,
                                    None => return Err(format!("Error at {}:{}, no value in buffer to take the square root.",line_nb,ins_nb))
                                };
                                self.values.push(EsianolopInstruction::Sqr(Box::new(val.clone())));
                            },
                            true => {
                                let temp = self.values.get(0);
                                let val = match temp {
                                    Some(e) => e,
                                    None => return Err(format!("Error at {}:{}, no value in buffer to take the square root.",line_nb,ins_nb))
                                };
                                self.values[0] = EsianolopInstruction::Sqr(Box::new(val.clone()));
                            }
                        }
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