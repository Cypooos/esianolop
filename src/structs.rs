use std::fmt;
use std::env;
use std::fs;
use std::num;
use std::collections::HashMap;


// Définition de la liste des instructions (sans les fonctions, ni les for, ils se font à coté)
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
    DpL(Box<EsianolopInstruction>),
    DpR(Box<EsianolopInstruction>),
    Sqr(Box<EsianolopInstruction>),
    Num(usize),
}

// ajoute de "méthodes" à l'objet EsianolopInstruction
impl EsianolopInstruction {

    // La fonction execute donne le résultat. Elle est récursive car elle appelle ces/son fil(s) pour connaitre sa valeur
    pub fn execute(&self) -> usize {
        match self {
            EsianolopInstruction::Nul => {return 0},
            EsianolopInstruction::Add(a,b) => {return a.execute()+b.execute()},
            EsianolopInstruction::Sub(a,b) => {return a.execute()-b.execute()},
            EsianolopInstruction::Mul(a,b) => {return a.execute()*b.execute()},
            EsianolopInstruction::Div(a,b) => {return a.execute()/b.execute()},
            EsianolopInstruction::Pow(a,b) => {return a.execute().pow(b.execute() as u32)},
            EsianolopInstruction::Dup(a) => {return a.execute()},
            EsianolopInstruction::DpL(a) => {return a.execute()},
            EsianolopInstruction::DpR(a) => {return a.execute()},
            EsianolopInstruction::Sqr(a) => {return (a.execute() as f64).sqrt() as usize},
            EsianolopInstruction::Num(a) => {return *a},
        }
    }
}

// Implémentation du charactère affichage pour une instruction (similaire à classe.__str__)
// La fonction est récursive car lors de l'affichage, il demande à afficher le(s) fil(s) au noeu.
impl fmt::Display for EsianolopInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        match self { // Selon mon type, j'affiche XXX(valeur1, valeur2 ...)
            EsianolopInstruction::Nul => {write!(f,"Nul")},
            EsianolopInstruction::Add(a,b) => {return write!(f,"Add({},{})",a,b)},
            EsianolopInstruction::Sub(a,b) => {return write!(f,"Sub({},{})",a,b)},
            EsianolopInstruction::Mul(a,b) => {return write!(f,"Mul({},{})",a,b)},
            EsianolopInstruction::Div(a,b) => {return write!(f,"Div({},{})",a,b)},
            EsianolopInstruction::Pow(a,b) => {return write!(f,"Pow({},{})",a,b)},
            EsianolopInstruction::Dup(a) => {return write!(f,"Dup({})",a)},
            EsianolopInstruction::DpL(a) => {return write!(f,"DpL({})",a)},
            EsianolopInstruction::DpR(a) => {return write!(f,"DpR({})",a)},
            EsianolopInstruction::Sqr(a) => {return write!(f,"Sqr({})",a)},
            EsianolopInstruction::Num(a) => {return write!(f,"Num({})",a)},
        }
    }
}


// Definition de la structure pour le compilateur
// avec values    : Stack d'Arbres 
// et   fonctions : Des bouts de codes stoqué sous des Strings
pub struct Esianolop {
    pub values:Vec<EsianolopInstruction>,
    pub functions:HashMap::<String,String>,
}


// Implémentation de "méthodes" -en rust c'est des fonction car tout est statique- pour l'objet Esianolop
impl Esianolop {

    // méthode new: retourne une nouvelle instance vide de Esianolop
    pub fn new() -> Esianolop {
        Esianolop {
            values:vec![],
            functions:HashMap::new()
        }
    }

    // Execute un fichier. Retourne soit Err(message d'erreur) ou Ok(())
    pub fn parse_file(&mut self,filename:&str) -> Result<(),String> {

        match fs::read_to_string(filename.to_owned()) { // Est-ce que le ficher à pu etre lu ?

            // Si oui, appelle de self.parse_text() avec le contenu du fichier et on retourne (car self.parse_text() à la même signiature)
            Ok(e) => {return self.parse_text(&e)}, 
            // Si non, on retourne une erreur
            Err(e)=> {return Err(format!("Error while parsing the file: {}",e))}, 
        }
    }


    // retourne le stack avec toutes les arbres calculés.
    pub fn get_result(&self) -> Vec<usize> {
        return self.values.iter().map(|x| x.execute()).collect::<Vec<usize>>()
    }

    fn execute_instruction(&mut self, vec_from_down:bool,specified:bool, instruction:&str) -> Result<(),String> {

        match instruction {
            // ----- les opérations qui prennent 2 entrées dans le stack -----
            "+" | "add" |
            "-" | "sub" |
            "*" | "mul" |
            "/" | "div" |
            "^" | "pow" => {

                let operation_fn = match instruction {
                    "+" | "add" => EsianolopInstruction::Add,
                    "-" | "sub" => EsianolopInstruction::Sub,
                    "*" | "mul" => EsianolopInstruction::Mul,
                    "/" | "div" => EsianolopInstruction::Div,
                    "^" | "pow" => EsianolopInstruction::Pow,
                    _           => EsianolopInstruction::Add // should never happen
                };
                
                // Obtenir les 2 premières valeures du stack / deux dernières
                let vals = if vec_from_down {
                        match self.values.get(0..2) {
                            Some(e) => e,
                            None => return Err(format!("not enogth values in buffer to left '{:?}'",operation_fn))
                        }
                    } else {
                        match self.values.get((self.values.len()-2)..(self.values.len())) {
                            Some(e) => e,
                            None => return Err(format!("not enogth values in buffer to right '{:?}'",operation_fn))
                        }
                    };
                
                
                let operation = operation_fn(Box::new(vals[0].to_owned()), Box::new(vals[1].to_owned()));

                // Mettre dans le stack
                if vec_from_down {
                    self.values.drain(1..2);
                    self.values[0] = operation;
                } else {
                    self.values.drain((self.values.len()-2)..(self.values.len()));
                    self.values.push(operation);
                };

                Ok(()) // Tout est bon, on retourne Ok(()) !
            },


            // ----- The duplicate instructions -----
            "~" | "dup" |
            "<" | "dpl" |
            ">" | "dpr" => {

                // Tout les cas ou la destination est à push sur la pile
                let right_is_destination:bool = (instruction == "dpr") | (instruction ==  ">") | ((instruction == "~") & (!vec_from_down)); 
                println!("Is right destination ? {}",right_is_destination); // Tempory debug

                let val = if vec_from_down {
                        let temp = self.values.get(0);
                        match temp {
                            Some(e) => e,
                            None => return Err(format!("no value in buffer to duplicate left to {}.", if right_is_destination {"right"} else {"left"}))
                        }
                    } else {
                        let temp = self.values.get(self.values.len()-1);
                        match temp {
                            Some(e) => e,
                            None => return Err(format!("no value in buffer to duplicate right to {}.",if right_is_destination {"right"} else {"left"}))
                        }
                    };
                 
                if right_is_destination {
                    // push back
                    self.values.push(EsianolopInstruction::Dup(Box::new(val.clone())));
                } else {
                    // push front
                    let mut tmp = Vec::new();
                    tmp.push(EsianolopInstruction::Dup(Box::new(val.clone())));
                    tmp.extend(self.values.to_owned());
                    self.values = tmp;
                };

                Ok(()) // Tout est bon, on retourne Ok(()) !
            },
            // ----- Les opérations qui prennent 1 entrée -----
            "$" | "sqr" => {
                match vec_from_down {
                    false => {
                        let temp = self.values.get(self.values.len()-1);
                        let val = match temp {
                            Some(e) => e,
                            None => return Err("no value in buffer to take the left square root.".to_owned())
                        };
                        self.values.push(EsianolopInstruction::Sqr(Box::new(val.clone())));
                        self.values.drain((self.values.len()-2)..(self.values.len()-1));
                    },
                    true => {
                        let temp = self.values.get(0);
                        let val = match temp {
                            Some(e) => e,
                            None => return Err("no value in buffer to take the right square root.".to_owned())
                        };
                        self.values[0] = EsianolopInstruction::Sqr(Box::new(val.clone()));
                    }
                };
                Ok(())
            }
            // ----- Delete -----
            "!" | "del" => {
                if self.values.len() == 0 {
                    return Err("no value to remove".to_owned());
                }
                match vec_from_down {
                    false => self.values.remove(self.values.len()-1),
                    true => self.values.remove(0),
                };
                Ok(())
            }
                // ----- Le reste (fonctions, nombre, non-définie) -----
            ins => {
                match ins.to_owned().parse::<usize>() {
                    Ok(e) => {

                        match (specified,vec_from_down) {
                            (true,true) => {
                                let mut tmp = Vec::new();
                                tmp.push(EsianolopInstruction::Num(e as usize));
                                tmp.extend(self.values.to_owned());
                                self.values = tmp;
                            },
                            (false,_) | (true,false) => {
                                self.values.push(EsianolopInstruction::Num(e as usize))
                            }
                        }
                    },
                    Err(_) => return Err("not a valid expression.".to_owned())
                };
                Ok(())
            }
        }
    }
    


    // Execute du code Esianolop multilignes, retourne soit Ok(()), ou Err(message d'erreur)
    pub fn parse_text(&mut self,text:&str) -> Result<(),String> {
        
        // Pour chaque ligne
        for (line_nb,line) in text.split("\n").map(|x| x.to_owned()).enumerate() {

            // Retire les commentaires du code
            let no_comment = line.split(";").collect::<Vec<&str>>()[0].split("#").collect::<Vec<&str>>()[0];

            // On énumère toutes les instructions
            for (ins_nb, mut instruction) in no_comment.split(" ").filter(|x| x.to_owned().trim() != "" ).enumerate() {

                // On test si il y a un "<" ou ">" devant
                let mut vec_from_down = true;
                let mut specified = false;
                if (instruction.len() >= 2) & (instruction.chars().nth(0) == Some('<') || instruction.chars().nth(0) == Some('>')) {
                    vec_from_down = instruction.chars().nth(0) == Some('<');
                    instruction = instruction.chars().next().map(|c| &instruction[c.len_utf8()..]).unwrap();
                    specified = true; // Utile pour les nombres, car par défault on l'ajoute à droite du stack
                }

                match self.execute_instruction(vec_from_down, specified, &instruction.to_ascii_lowercase()) {
                    Err(e) => return Err(format!("Error at {}:{}, {}",line_nb,ins_nb,e)),
                    _ => (),
                };
            }
            
        }
        Ok(()) // Tout c'est bien passé, on retourne Ok(())
    }

}