
pub enum EsianolopInstruction {
    Nul,
    Add(Box<EsianolopInstruction>,Box<EsianolopInstruction>),
    Mul(Box<EsianolopInstruction>,Box<EsianolopInstruction>),
    Sub(Box<EsianolopInstruction>,Box<EsianolopInstruction>),
    Dup(Box<EsianolopInstruction>),
    Del(Box<EsianolopInstruction>),
    Num(usize),
    Prn(Box<EsianolopInstruction>),
}

struct Esianolop {
    values:Vec<usize>,
    tree:EsianolopInstruction,
}

impl Esianolop {

    pub fn new() -> Esianolop {
        Esianolop {
            values:vec![],
            tree:EsianolopInstruction::Nul,
        }
    }


    
    pub fn parse_text(&mut self,text:&str) -> Result<EsianolopInstruction,String> {
        
        for (line_nb,line) in text.split("\n").map(|x| x.to_owned()).enumerate() {
            let mut instruction_num = 0;
            for (ins_nb, instruction) in line.split(";").collect::<Vec<&str>>()[0].split("#").collect::<Vec<&str>>()[0].split(" ").filter(|x| x.to_owned().trim() != "" ).enumerate() {
                match instruction {
                    "+" | "add" => {
                        let vals = match self.values.get(0..1) {
                            Some(e) => e,
                            None => return Err(format!("Error at {}:{}, not engoth values to add.",line_nb,instruction_num))
                        };
                        let left = self.values.get(0);
                    }
                }
                println!("{}:{} - {}",line_nb,ins_nb,instruction);
            }
        }
        EsianolopInstruction::Nul{}
    }

}