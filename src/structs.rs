use std::fmt;
use std::fs;
use std::collections::HashMap;

// rust fonctionne avec des structures, non pas des classes, mais tout est pareil (sauf que techniquement parlant, tout est statique en rust)


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
    Dup(usize), // Par soucis de performence, les duplications conserve le résultat directement (non pas une référence)
    DpL(usize), 
    DpR(usize), 
    Sqr(Box<EsianolopInstruction>),
    Num(usize),
}

// ajoute de "méthodes" à l'objet EsianolopInstruction
impl EsianolopInstruction {

    // La fonction execute donne le résultat. Elle est récursive car elle appelle ces/son fil(s) pour connaitre sa valeur
    // Elle est aussi sécurisé. Si une opération rate (exemple: 5-8) elle ne paniquera pas mais arretera le programme
    pub fn execute(&self) -> Result<usize,&str> {
        match self {
            EsianolopInstruction::Nul => {return Ok(0)},
            EsianolopInstruction::Add(a,b) => {return a.execute()?.checked_add(b.execute()?).ok_or("overflow in addition")},
            EsianolopInstruction::Sub(a,b) => {return a.execute()?.checked_sub(b.execute()?).ok_or("negative or overflow in substraction")},
            EsianolopInstruction::Div(a,b) => {return a.execute()?.checked_div(b.execute()?).ok_or("can't divide")},
            EsianolopInstruction::Mul(a,b) => {return a.execute()?.checked_mul(b.execute()?).ok_or("overflow in multiplication")},
            EsianolopInstruction::Pow(a,b) => {return a.execute()?.checked_pow(b.execute()? as u32).ok_or("overflow in powering")},
            EsianolopInstruction::Dup(a) => {return Ok(*a)},
            EsianolopInstruction::DpL(a) => {return Ok(*a)},
            EsianolopInstruction::DpR(a) => {return Ok(*a)},
            EsianolopInstruction::Sqr(a) => {
                let res = (a.execute()? as f64).sqrt();
                if res.is_nan() {return Err("negative square-root")};
                return Ok(res as usize)
            },
            EsianolopInstruction::Num(a) => {return Ok(*a)},
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


// Definition de la structure pour le interpréteur
// avec    values : Stack d'Arbres 
// et   fonctions : Des bouts de codes stoqué sous des Strings executes dès que appelé. Un dictionnaire au final.
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

    // reset les valeures
    pub fn clear(&mut self) {
        self.values.clear();
        self.functions.clear();
    }

    // Execute un fichier. Retourne soit Err(message d'erreur) ou Ok(())
    pub fn parse_file(&mut self,filename:&str) -> Result<(),String> {

        match fs::read_to_string(filename.to_owned()) { // Est-ce que le ficher à pu etre lu ?

            // Si oui, appelle de self.parse_text() avec le contenu du fichier et on retourne (car self.parse_text() à la même signiature)
            Ok(e) => {
                self.clear();
                return self.parse_text(&e)
            }, 
            // Si non, on retourne une erreur
            Err(e)=> {return Err(format!("Error while parsing the file: {}",e))}, 
        }
    }


    // retourne le stack avec toutes les arbres calculés.
    pub fn get_result(&self) -> Vec<Result<usize,&str>> {
        return self.values.iter().map(|x| x.execute()).collect::<Vec<Result<usize,&str>>>()
    }

    fn execute_instruction(&mut self, vec_from_down:bool,specified:bool, mut instruction:&str) -> Result<(),String> {

        instruction = instruction.trim();

        match instruction.trim() {
            // ----- les opérations qui prennent 2 entrées dans le stack -----
            "+" | "add" |
            "-" | "sub" |
            "*" | "mul" |
            "/" | "div" |
            "^" | "pow" => {


                // On obtiens la classe correspondante à notre instruction
                let operation_fn = match instruction {
                    "+" | "add" => EsianolopInstruction::Add,
                    "-" | "sub" => EsianolopInstruction::Sub,
                    "*" | "mul" => EsianolopInstruction::Mul,
                    "/" | "div" => EsianolopInstruction::Div,
                    "^" | "pow" => EsianolopInstruction::Pow,
                    e           => {println!("What ???:{:?}",e);unreachable!()} // Ne devrai jamais arriver, mais si oui, panique le programme (arret brutal)
                };
                
                // Obtenir les 2 premières valeures du stack / deux dernières
                let vals = if vec_from_down {
                        match self.values.get(0..2) {
                            Some(e) => e,
                            None => return Err(format!("not enogth values in buffer to left '{}'",instruction))
                        }
                    } else {
                        match self.values.get((self.values.len()-2)..(self.values.len())) {
                            Some(e) => e,
                            None => return Err(format!("not enogth values in buffer to right '{}'",instruction))
                        }
                    };
                
                // Instancier avec les deux valeures
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
                //println!("Is right destination ? {}",right_is_destination); // Tempory debug

                let val = if vec_from_down {
                        let temp = self.values.get(0);
                        match temp {
                            Some(e) => e,
                            None => return Err(format!("no value in buffer to duplicate left to {}", if right_is_destination {"right"} else {"left"}))
                        }
                    } else {
                        let temp = self.values.get(self.values.len()-1);
                        match temp {
                            Some(e) => e,
                            None => return Err(format!("no value in buffer to duplicate right to {}",if right_is_destination {"right"} else {"left"}))
                        }
                    };
                 
                let val = match val.clone().execute() {
                    Ok(e) => e,
                    Err(e) => return Err(format!("{} in duplicate {} to {}",e,if vec_from_down {"left"} else {"right"},if right_is_destination {"right"} else {"left"}))
                };
                if right_is_destination {
                    // push back
                    self.values.push(EsianolopInstruction::Dup(val));
                } else {
                    // push front
                    let mut tmp = Vec::new();
                    tmp.push(EsianolopInstruction::Dup(val));
                    tmp.extend(self.values.to_owned());
                    self.values = tmp;
                };

                Ok(()) // Tout est bon, on retourne Ok(()) !
            },
            // ----- Les opérations qui prennent 1 entrée -----
            "$" | "sqr" => {
                match vec_from_down { // Si on prend la valeure depuis la guache ou droite 
                    false => {
                        let temp = self.values.get(self.values.len()-1);
                        let val = match temp {
                            Some(e) => e,
                            None => return Err("no value in buffer to take the left square root".to_owned())
                        };
                        self.values.push(EsianolopInstruction::Sqr(Box::new(val.clone())));
                        self.values.drain((self.values.len()-2)..(self.values.len()-1));
                    },
                    true => {
                        let temp = self.values.get(0);
                        let val = match temp {
                            Some(e) => e,
                            None => return Err("no value in buffer to take the right square root".to_owned())
                        };
                        self.values[0] = EsianolopInstruction::Sqr(Box::new(val.clone()));
                    }
                };
                Ok(())
            }
            // ----- Delete -----
            "!" | "del" => {
                if self.values.len() == 0 { // Si aucune valeure dans le stack d'arbre
                    return Err("no value to remove".to_owned()); // Erreur
                }
                match vec_from_down { // Sinon, retirer la valeur correspondante
                    false => self.values.remove(self.values.len()-1),
                    true => self.values.remove(0),
                };
                Ok(())
            }
                // ----- Le reste (fonctions, nombre, non-définie) -----
            ins => {
                match ins.to_owned().parse::<usize>() {
                    Ok(e) => {

                        match (specified,vec_from_down) { // Par défault, la position d'un nombre est à droite, et pas à gauche, donc on vérifie si la position à été définie
                            (true,true) => { // Si position définie ET à gauche
                                let mut tmp = Vec::new();
                                tmp.push(EsianolopInstruction::Num(e as usize));
                                tmp.extend(self.values.to_owned());
                                self.values = tmp;
                            },
                            (false,_) | (true,false) => { // si position pas définie OU position définie et droite
                                self.values.push(EsianolopInstruction::Num(e as usize))
                            }
                        }
                    },
                    Err(_) => {
                        
                        if self.functions.contains_key(ins) { // Si c'est dans la liste des fonctions
                            let x = &self.functions.get(ins).unwrap().clone(); // On prend le code défini par la fonction
                            println!("Executing function {} with {}",ins,x);
                            return match self.parse_text(x) { // Execute le code de la fonction (marche pour les fonctions récursive donc)
                                Err(e) => Err(e+" in function "+ins), // Ajout à l'erreur des informations de la trace
                                Ok(_) => Ok(())
                            } 
                        } else {
                            return Err(format!("{} is not a valid expression nor function",ins)) // Sinon on retourne une erreur
                        }
                        
                    }
                };
                Ok(()) // Tout vas bien !
            }
        } 
    }
    


    // Execute du code Esianolop multilignes, retourne soit Ok(()), ou Err(message d'erreur)
    pub fn parse_text(&mut self,text:&str) -> Result<(),String> {

        //println!("Executing '{}'",text);
        
        // Pour chaque ligne
        for (line_nb,line) in text.to_ascii_lowercase().split("\n").map(|x| x.to_owned()).enumerate() {

            // Retire les commentaires du code
            let no_comment = line.split(";").collect::<Vec<&str>>()[0].split("#").collect::<Vec<&str>>()[0];

            // On énumère toutes les instructions, mais avec la possibilité d'en skipper (en utillisant table.next()), pour les fonctions 
            let mut table_iter = no_comment.split(" ").filter(|x| x.to_owned().trim() != "" ).enumerate();
            
            let mut active_instruction = table_iter.next();
            let mut in_function = false;
            let mut temp_lifetime_instruction:String;
            while let Some((ins_nb, mut instruction)) = active_instruction {

                //println!("{}",instruction);

                if in_function {
                    instruction = instruction.split(":").collect::<Vec<&str>>()[1];
                    in_function = false;
                }

                if instruction.trim() == "" {active_instruction = table_iter.next();continue};

                // Si c'est une définition de fonction/for, on skip j'usqu'a la fin de la def
                if instruction.contains(":") {

                    //println!("Founded function at {}:{}",line_nb,ins_nb);

                    let mut line_rest = table_iter.clone().map(|(_,y)|y) // on prend le reste de la ligne (derrère l'instruction)
                        .collect::<Vec<&str>>().join(" ").to_owned();
                    
                    line_rest = instruction.to_owned()+ " "+&line_rest; // On rajoute notre instruction à la ligne
                    //println!("Line_rest:{}",line_rest);
                    
                    let function_name = line_rest.split(":").collect::<Vec<&str>>()[0].trim(); // On prend le nom de la fonction définie
                    let mut function_code = line_rest.split(":").collect::<Vec<&str>>()[1]; // On prend le reste (le code dans la fonction ET le reste de la ligne)
                    

                    if function_code.contains(":") { // En cas de continuation du code derrière la fonction
                        function_code = function_code.split(":").collect::<Vec<&str>>()[0]; // On le retire du code de la fonction
                    };

                    // Si le nom / code de la fonction est vide
                    if (function_name == "") | (function_code.trim() == "") { return Err(format!("trying to define an empty function at {}:{}",line_nb,ins_nb))} 
                    // Si on redéfinie la fonction
                    if self.functions.contains_key(function_name) {return Err(format!("trying to define already-defined function at {}:{}",line_nb,ins_nb))}
                    println!("Defing function {} with {}",function_name,function_code.trim());

                    // On saute à la fin de la définition de la fonction, pour la prochaine instruction
                    let skip_count = function_code.matches(' ').count();
                    if skip_count == 0{
                        // Si il y a pas d'espaces, on reste sur l'instrcution actuelle, mais retire la définition de la fonction
                        in_function = false;
                        temp_lifetime_instruction = instruction.split(":").map(|x|x.to_owned()).collect::<Vec<String>>().get(2..).unwrap().join(":");
                        active_instruction = Some((ins_nb, &temp_lifetime_instruction));
                    } else {
                        // Sinon on saute et coupe le fin de définition
                        active_instruction =table_iter.nth(skip_count-1);
                        let (pos,ins) = match active_instruction {
                            None  => {return Err(format!("Jump to function end failed at {}:{}",line_nb,ins_nb))},
                            Some((pos,ins)) => (pos,ins),
                        };
                        temp_lifetime_instruction = ins.split(":").map(|x|x.to_owned()).collect::<Vec<String>>().get(1..).unwrap().join(":");
                        active_instruction = Some((pos,&temp_lifetime_instruction));
                    }

                    match function_name {
                        // Si on définie une fonction avec le nom "for", on execute le code spécial for
                        "for" | "<for" | ">for" => {

                            // si il n'y a pas de valeurs pour executer la boucle
                            if self.values.len() == 0 {
                                return Err(format!("Aptended a for with nothing in the stack at {}:{}",line_nb,ins_nb))
                            }

                            // On prend le premier charactère du for, pour savoir si c'est < ou pas
                            let index = if &function_name[0..1] == "<" {0} else {self.values.len()-1};
                            let nb = match self.values[index].execute() {
                                Ok(e) => e,
                                Err(e) => return Err(format!("{} in accesing number of for loop at {}:{}",e,line_nb,ins_nb))
                            }; // On récupère le nombre de boucle 

                            self.values.remove(index); // On retire le nombre d'execution

                            for _ in 0..nb { // On execute le for
                                match self.parse_text(function_code.trim()) {
                                    Err(e) => return Err(format!("{} in for loop at {}:{}",e,line_nb+1,ins_nb+1)),
                                    Ok(_)=> (),
                                }
                            }
                        }
                        // Sinon, on créé la fonction
                        _ => {
                            self.functions.insert(function_name.to_owned(), function_code.trim().to_owned()); // Ajouter la fonction à la hashmap de fonctions
                            //println!("Defined !");
                        }
                    };
                    continue;
                }

                // On charge l'instruction suivante pour la prochaine boucle
                active_instruction =table_iter.next();

                // On test si il y a un "<" ou ">" devant
                let mut vec_from_down = true;
                let mut specified = false;
                if (instruction.len() >= 2) & (instruction.chars().nth(0) == Some('<') || instruction.chars().nth(0) == Some('>')) {
                    vec_from_down = instruction.chars().nth(0) == Some('<');
                    instruction = instruction.chars().next().map(|c| &instruction[c.len_utf8()..]).unwrap();
                    specified = true; // Utile pour les nombres, car par défault on l'ajoute à droite du stack
                }

                // On execute le code, et si il y a une erreur, on l'affiche
                match self.execute_instruction(vec_from_down, specified, &instruction.to_ascii_lowercase()) {
                    
                    Err(e) => {
                        
                        return Err(format!("Error at {}:{}, {{\n\t{}\n}}",line_nb+1,ins_nb+1,e.replace("\n", "\n\t"))) 
                    },
                    _ => (),
                };
            }
            
        }
        Ok(()) // Tout c'est bien passé, on retourne Ok(())
    }

}